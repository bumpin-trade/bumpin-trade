use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpResult;
use crate::processor::position_processor;
use crate::processor::position_processor::DecreasePositionParams;
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::utils::pda::generate_position_key;

#[derive(Accounts)]
#[instruction(
    _market_index: u16, _pool_index: u16, _stable_pool_index: u16, _trade_token_index: u16, _user_authority_key: Pubkey
)]
pub struct LiquidatePosition<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        seeds = [b"user", _user_authority_key.as_ref()],
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
        seeds = [b"market", _market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool", _pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.mint_key.eq(& market.load() ?.pool_mint_key)
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool", _stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.mint_key.eq(& market.load() ?.stable_pool_mint_key)
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), _stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"trade_token_vault".as_ref(), _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,

    pub keeper_signer: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_liquidate_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, LiquidatePosition>,
    position_key: Pubkey,
    liquidation_price: u128,
) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let remaining_accounts = ctx.remaining_accounts;
    let mut market = ctx.accounts.market.load_mut()?;
    let mut trade_token = ctx.accounts.trade_token.load_mut()?;
    let mut oracle_map = OracleMap::load(remaining_accounts)?;
    let mut base_token_pool = ctx.accounts.pool.load_mut()?;
    let mut stable_pool = ctx.accounts.stable_pool.load_mut()?;

    let (is_long, is_cross, margin_mint, position_size) = update_borrowing_fee_and_funding_fee(
        &position_key,
        &user,
        base_token_pool.deref_mut(),
        stable_pool.deref_mut(),
        &ctx.accounts.state,
        &trade_token,
        market.deref_mut(),
        &mut oracle_map,
    )?;

    let symbol = market.symbol;
    let user_key = user.key;
    position_processor::decrease_position(
        DecreasePositionParams {
            order_id: 0,
            is_liquidation: true,
            is_portfolio_margin: is_cross,
            margin_token: margin_mint,
            decrease_size: position_size,
            execute_price: liquidation_price,
        },
        &mut user,
        &mut market,
        &mut base_token_pool,
        &mut stable_pool,
        &ctx.accounts.state,
        Some(&ctx.accounts.user_token_account),
        if is_long { &ctx.accounts.pool_vault } else { &ctx.accounts.stable_pool_vault },
        trade_token.deref_mut(),
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
        &mut oracle_map,
        &generate_position_key(&user_key, symbol, is_cross, ctx.program_id)?,
    )?;
    Ok(())
}

fn update_borrowing_fee_and_funding_fee(
    position_key: &Pubkey,
    user: &User,
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state: &State,
    trade_token: &TradeToken,
    market: &mut Market,
    oracle_map: &mut OracleMap,
) -> BumpResult<(bool, bool, Pubkey, u128)> {
    let user_position = user.get_user_position_ref(position_key)?;
    let pool = if user_position.is_long { base_token_pool } else { stable_pool };

    market.update_market_funding_fee_rate(
        state,
        oracle_map.get_price_data(&trade_token.oracle_key)?.price,
        trade_token.decimals,
    )?;

    pool.update_pool_borrowing_fee_rate()?;
    Ok((
        user_position.is_long,
        user_position.is_portfolio_margin,
        user_position.margin_mint_key,
        user_position.position_size,
    ))
}
