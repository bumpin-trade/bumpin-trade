use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::can_sign_for_user;
use crate::is_normal;
use crate::processor::optional_accounts::load_maps;
use crate::processor::{pool_processor, stake_processor};
use crate::state::bump_events::StakeOrUnStakeEvent;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::User;
use crate::utils;

#[derive(Accounts)]
#[instruction(_pool_index: u16, _trade_token_index: u16)]
pub struct PortfolioStake<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    /// CHECK: ?
    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    pub bump_signer: AccountInfo<'info>,

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
        seeds = [b"pool_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), _trade_token_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,
    pub authority: Signer<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_portfolio_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PortfolioStake>,
    _pool_index: u16,
    _trade_token_index: u16,
    request_token_amount: u128,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool.load_mut()?;
    let user = &mut ctx.accounts.user.load_mut()?;
    let remaining_accounts = ctx.remaining_accounts;
    let mut account_maps = load_maps(remaining_accounts)?;

    let base_mint_amount = stake_processor::stake(
        pool.deref_mut(),
        user.deref_mut(),
        &account_maps.trade_token_map,
        &mut account_maps.oracle_map,
        request_token_amount,
    )?;
    let (supply_amount, user_stake) = pool_processor::portfolio_to_stake(
        user,
        pool,
        base_mint_amount,
        &account_maps.trade_token_map,
        &mut account_maps.oracle_map,
        &account_maps.market_map,
        &ctx.accounts.state,
    )?;

    utils::token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.pool_vault,
        &ctx.accounts.bump_signer,
        ctx.accounts.state.bump_signer_nonce,
        request_token_amount,
    )?;
    pool.add_amount_and_supply(base_mint_amount, supply_amount)?;
    pool.update_pool_borrowing_fee_rate()?;
    emit!(StakeOrUnStakeEvent {
        user_key: user.key,
        token_mint: pool.mint_key,
        change_supply_amount: supply_amount,
        user_stake,
    });
    Ok(())
}
