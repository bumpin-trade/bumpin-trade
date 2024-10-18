use crate::instructions::RebalanceMarketStableLossParams;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::rebalance_processor;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(
    _pool_index: u16, _trade_token_index: u16
)]
pub struct AutoRebalance<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

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
        constraint = state.keeper_key.eq(& keeper_key.key())
    )]
    pub keeper_key: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_auto_rebalance<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, AutoRebalance<'info>>,
) -> Result<()> {
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { market_map, .. } = load_maps(remaining_accounts)?;
    rebalance_processor::rebalance_pool_unsettle(
        &ctx.accounts.state,
        &ctx.accounts.pool,
        &ctx.accounts.trade_token,
        &ctx.accounts.pool_vault,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
        &market_map,
    )?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    _params: RebalanceMarketStableLossParams
)]
pub struct RebalanceMarketStableLoss<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"market", _params.market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), _params.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load()? .key.eq(& market.load() ?.pool_key)
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), _params.pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = market.load() ?.pool_mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), _params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = market.load() ?.stable_pool_mint_key,
        token::authority = state.bump_signer
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token", _params.trade_token_index.to_le_bytes().as_ref()],
        bump,
        constraint = trade_token.load()? .mint_key.eq(& market.load() ?.pool_mint_key)
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"trade_token", _params.stable_trade_token_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_trade_token.load()? .mint_key.eq(& market.load() ?.stable_pool_mint_key)
    )]
    pub stable_trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        token::mint = market.load() ?.pool_mint_key,
        token::authority = keeper_key.key()
    )]
    pub keeper_trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = market.load() ?.stable_pool_mint_key,
        token::authority = keeper_key.key()
    )]
    pub keeper_stable_trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = state.keeper_key.eq(& keeper_key.key())
    )]
    pub keeper_key: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_rebalance_market_stable_loss<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, RebalanceMarketStableLoss<'info>>,
) -> Result<()> {
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { mut oracle_map, .. } = load_maps(remaining_accounts)?;
    rebalance_processor::rebalance_market_stable_loss(
        &ctx.accounts.state,
        &ctx.accounts.pool,
        &ctx.accounts.pool_vault,
        &ctx.accounts.stable_pool_vault,
        &ctx.accounts.trade_token,
        &ctx.accounts.stable_trade_token,
        &ctx.accounts.keeper_trade_token_vault,
        &ctx.accounts.keeper_stable_trade_token_vault,
        &ctx.accounts.keeper_key,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
        &mut oracle_map,
        &ctx.accounts.market,
    )?;
    Ok(())
}
