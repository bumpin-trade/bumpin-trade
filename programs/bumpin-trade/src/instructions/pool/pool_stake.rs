use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::can_sign_for_user;
use crate::instructions::Either;
use crate::processor::optional_accounts::load_maps;
use crate::processor::{pool_processor, stake_processor};
use crate::state::bump_events::StakeOrUnStakeEvent;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::utils;

#[derive(Accounts)]
#[instruction(pool_index: u16, trade_token_index: u16)]
pub struct PortfolioStake<'info> {
    #[account(
        mut,
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
        seeds = [b"user", authority.key.as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), trade_token_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(pool_index: u16)]
pub struct WalletStake<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"user", authority.key.as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
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
        seeds = [b"pool_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct StakeParams {
    pub request_token_amount: u128,
    pub pool_index: u16,
    pub trade_token_index: u16,
}

pub fn handle_portfolio_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PortfolioStake>,
    pool_index: u16,
    trade_token_index: u16,
    request_token_amount: u128,
) -> Result<()> {
    handle_pool_stake0(
        Either::Left(ctx),
        StakeParams { request_token_amount, pool_index, trade_token_index },
    )
}

pub fn handle_wallet_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, WalletStake>,
    pool_index: u16,
    trade_token_index: u16,
    request_token_amount: u128,
) -> Result<()> {
    handle_pool_stake0(
        Either::Right(ctx),
        StakeParams { request_token_amount, pool_index, trade_token_index },
    )
}

fn handle_pool_stake0<'a, 'b, 'c: 'info, 'info>(
    ctx: Either<
        Context<'a, 'b, 'c, 'info, PortfolioStake>,
        Context<'a, 'b, 'c, 'info, WalletStake>,
    >,
    stake_params: StakeParams,
) -> Result<()> {
    match ctx {
        Either::Left(ctx) => {
            let pool = &mut ctx.accounts.pool.load_mut()?;
            let user = &mut ctx.accounts.user.load_mut()?;
            let remaining_accounts = ctx.remaining_accounts;
            let mut account_maps = load_maps(remaining_accounts, &ctx.accounts.state.bump_signer)?;

            let base_mint_amount = stake_processor::stake(
                pool.deref_mut(),
                user.deref_mut(),
                &account_maps.trade_token_map,
                &mut account_maps.oracle_map,
                stake_params.request_token_amount,
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
                stake_params.request_token_amount,
            )?;
            pool.add_amount_and_supply(stake_params.request_token_amount, supply_amount)?;
            emit!(StakeOrUnStakeEvent {
                user_key: user.key,
                token_mint: pool.mint_key,
                change_supply_amount: supply_amount,
                user_stake,
            });
        },
        Either::Right(ctx) => {
            let pool = &mut ctx.accounts.pool.load_mut()?;
            let user = &mut ctx.accounts.user.load_mut()?;

            let remaining_accounts = ctx.remaining_accounts;
            let mut account_maps = load_maps(remaining_accounts, &ctx.accounts.state.admin)?;

            let base_mint_amount = stake_processor::stake(
                pool.deref_mut(),
                user.deref_mut(),
                &account_maps.trade_token_map,
                &mut account_maps.oracle_map,
                stake_params.request_token_amount,
            )?;
            let user_key = user.key;
            let user_stake = user.get_user_stake_mut_ref(&pool.key)?;

            let supply_amount = pool_processor::stake(
                pool,
                base_mint_amount,
                &account_maps.trade_token_map,
                &mut account_maps.oracle_map,
                &account_maps.market_map,
                &ctx.accounts.state,
            )?;
            user_stake.add_staked_share(supply_amount)?;
            utils::token::receive(
                &ctx.accounts.token_program,
                &ctx.accounts.user_token_account,
                &ctx.accounts.pool_vault,
                &ctx.accounts.authority,
                stake_params.request_token_amount,
            )?;
            pool.add_amount_and_supply(stake_params.request_token_amount, supply_amount)?;
            emit!(StakeOrUnStakeEvent {
                user_key,
                token_mint: pool.mint_key,
                change_supply_amount: supply_amount,
                user_stake: *user_stake,
            });
        },
    };

    Ok(())
}
