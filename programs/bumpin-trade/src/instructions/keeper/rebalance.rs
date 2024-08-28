use crate::processor::rebalance_processor;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(
    pool_index: u16, stable_pool_index: u16, _trade_token_index: u16
)]
pub struct AutoRebalance<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = ! pool.load() ?.stable,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.stable,
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

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
        seeds = [b"pool_vault".as_ref(), stable_pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = stable_pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
        constraint = trade_token.load() ?.mint_key.eq(& pool.load() ?.mint_key ),
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), _trade_token_index.to_le_bytes().as_ref()],
        bump,
        token::mint = trade_token.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(
    pool_index: u16, stable_pool_index: u16, _trade_token_index: u16
)]
pub struct RewardsRebalance<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = ! pool.load() ?.stable,
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
        seeds = [b"trade_token_vault".as_ref(), _trade_token_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_auto_reblance<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, AutoRebalance<'info>>,
) -> Result<()> {
    rebalance_processor::rebalance_pool_unsettle(
        &ctx.accounts.state,
        &ctx.accounts.pool,
        &ctx.accounts.trade_token,
        &ctx.accounts.pool_vault,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
    )?;
    rebalance_processor::rebalance_rewards(
        &ctx.accounts.state,
        &ctx.accounts.pool,
        &ctx.accounts.pool_vault,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
    )?;
    rebalance_processor::rebalance_stable_pool(
        &ctx.accounts.state,
        &ctx.accounts.pool,
        &ctx.accounts.pool_vault,
        &ctx.accounts.stable_pool_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
    )?;
    Ok(())
}

pub fn handle_rewards_reblance<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, RewardsRebalance<'info>>,
    _pool_index: u16,
    _stable_pool_index: u16,
    _trade_token_index: u16,
) -> Result<()> {
    rebalance_processor::rebalance_rewards(
        &ctx.accounts.state,
        &ctx.accounts.pool,
        &ctx.accounts.pool_vault,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
    )?;
    Ok(())
}
