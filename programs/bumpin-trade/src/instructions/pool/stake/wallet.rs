use std::ops::DerefMut;

use crate::can_sign_for_user;
use crate::is_normal;
use crate::processor::optional_accounts::load_maps;
use crate::processor::{pool_processor, stake_processor};
use crate::state::bump_events::StakeOrUnStakeEvent;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::utils;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(_pool_index: u16)]
pub struct WalletStake<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ? && is_normal(& user) ?,
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        constraint = pool_vault.mint.key().eq(& user_token_account.mint.key()),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct StakeParams {
    pub request_token_amount: u128,
    pub pool_index: u16,
    pub trade_token_index: u16,
}

pub fn handle_wallet_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, WalletStake>,
    _pool_index: u16,
    request_token_amount: u128,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool.load_mut()?;
    let user = &mut ctx.accounts.user.load_mut()?;

    let remaining_accounts = ctx.remaining_accounts;
    let mut accounts = load_maps(remaining_accounts)?;

    let base_mint_amount = stake_processor::stake(
        pool.deref_mut(),
        user.deref_mut(),
        &accounts.trade_token_map,
        &mut accounts.oracle_map,
        request_token_amount,
    )?;
    let user_key = user.key;
    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;

    let supply_amount = pool_processor::stake(
        pool,
        base_mint_amount,
        &accounts.trade_token_map,
        &mut accounts.oracle_map,
        &accounts.market_map,
    )?;
    user_stake.add_staked_share(supply_amount)?;
    utils::token::receive(
        &ctx.accounts.token_program,
        &ctx.accounts.user_token_account,
        &ctx.accounts.pool_vault,
        &ctx.accounts.authority,
        request_token_amount,
    )?;
    pool.add_amount_and_supply(base_mint_amount, supply_amount)?;
    pool.update_pool_borrowing_fee_rate()?;
    emit!(StakeOrUnStakeEvent {
        user_key,
        token_mint: pool.mint_key,
        change_supply_amount: supply_amount,
        user_stake: *user_stake,
    });

    Ok(())
}
