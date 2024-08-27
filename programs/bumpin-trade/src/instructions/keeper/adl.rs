use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::errors::BumpErrorCode::{OnlyCrossPositionAllowed, OnlyIsolatePositionAllowed};
use crate::instructions::ADLParams;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor;
use crate::processor::position_processor::DecreasePositionParams;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::User;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    params: ADLParams
)]
pub struct ADLIsolate<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_key,
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"user", params.user_authority_key.key().as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        constraint = user_token_account.owner.eq(& user.load() ?.authority)
        && (pool_vault.mint.eq(& user_token_account.mint) || stable_pool_vault.mint.eq(& user_token_account.mint)),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"market", params.market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.key.eq(& market.load() ?.pool_key),
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.key.eq(& market.load() ?.stable_pool_key),
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = stable_pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token", params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), params.trade_token_index.to_le_bytes().as_ref()],
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

pub fn handle_adl_isolate<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ADLIsolate<'info>>,
    params: ADLParams,
) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_mut()?;
    let mut stable_pool = ctx.accounts.stable_pool.load_mut()?;

    let market_account_loader = &ctx.accounts.market;
    let state_account = &ctx.accounts.state;
    let pool_vault_account = &ctx.accounts.pool_vault;
    let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
    let trade_token_loader = &ctx.accounts.trade_token;
    let trade_token_vault_account = &ctx.accounts.trade_token_vault;
    let bump_signer_account_info = &ctx.accounts.bump_signer;
    let token_program = &ctx.accounts.token_program;
    let mut user_account = ctx.accounts.user.load_mut()?;
    let user_token_account = &ctx.accounts.user_token_account;

    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { mut oracle_map, .. } = load_maps(remaining_accounts)?;

    let position = user_account.get_user_position_ref(&params.position_key)?;
    validate!(!position.is_portfolio_margin, OnlyIsolatePositionAllowed)?;
    let is_portfolio_margin = position.is_portfolio_margin;
    let margin_token = position.margin_mint_key;
    let decrease_size = position.position_size;
    let position_key = position.position_key;
    let is_long = position.is_long;
    position_processor::decrease_position(
        DecreasePositionParams {
            order_id: 0,
            is_liquidation: false,
            is_portfolio_margin,
            margin_token,
            decrease_size,
            execute_price: oracle_map
                .get_price_data(&position.index_mint_oracle)
                .map_err(|_e| BumpErrorCode::OracleNotFound)?
                .price,
        },
        user_account.deref_mut(),
        market_account_loader.load_mut()?.deref_mut(),
        pool.deref_mut(),
        stable_pool.deref_mut(),
        state_account,
        Some(user_token_account),
        if is_long { pool_vault_account } else { stable_pool_vault_account },
        trade_token_loader.load_mut()?.deref_mut(),
        trade_token_vault_account,
        bump_signer_account_info,
        token_program,
        &mut oracle_map,
        &position_key,
    )?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    params: ADLParams
)]
pub struct ADLCross<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_key,
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"user", params.user_authority_key.key().as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"market", params.market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.key.eq(& market.load() ?.pool_key),
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.key.eq(& market.load() ?.stable_pool_key),
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = stable_pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token", params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), params.trade_token_index.to_le_bytes().as_ref()],
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

pub fn handle_adl_cross<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ADLCross<'info>>,
    params: ADLParams,
) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_mut()?;
    let mut stable_pool = ctx.accounts.stable_pool.load_mut()?;

    let market_account_loader = &ctx.accounts.market;
    let state_account = &ctx.accounts.state;
    let pool_vault_account = &ctx.accounts.pool_vault;
    let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
    let trade_token_loader = &ctx.accounts.trade_token;
    let trade_token_vault_account = &ctx.accounts.trade_token_vault;
    let bump_signer_account_info = &ctx.accounts.bump_signer;
    let token_program = &ctx.accounts.token_program;
    let mut user_account = ctx.accounts.user.load_mut()?;

    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { mut oracle_map, .. } = load_maps(remaining_accounts)?;

    let position = user_account.get_user_position_ref(&params.position_key)?;
    validate!(position.is_portfolio_margin, OnlyCrossPositionAllowed)?;
    let is_portfolio_margin = position.is_portfolio_margin;
    let margin_token = position.margin_mint_key;
    let decrease_size = position.position_size;
    let position_key = position.position_key;
    let is_long = position.is_long;
    position_processor::decrease_position(
        DecreasePositionParams {
            order_id: 0,
            is_liquidation: false,
            is_portfolio_margin,
            margin_token,
            decrease_size,
            execute_price: oracle_map
                .get_price_data(&position.index_mint_oracle)
                .map_err(|_e| BumpErrorCode::OracleNotFound)?
                .price,
        },
        user_account.deref_mut(),
        market_account_loader.load_mut()?.deref_mut(),
        pool.deref_mut(),
        stable_pool.deref_mut(),
        state_account,
        None,
        if is_long { pool_vault_account } else { stable_pool_vault_account },
        trade_token_loader.load_mut()?.deref_mut(),
        trade_token_vault_account,
        bump_signer_account_info,
        token_program,
        &mut oracle_map,
        &position_key,
    )?;
    Ok(())
}
