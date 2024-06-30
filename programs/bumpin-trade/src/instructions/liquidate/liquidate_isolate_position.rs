use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use std::ops::DerefMut;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::processor::market_processor::MarketProcessor;
use crate::processor::position_processor;
use crate::processor::position_processor::DecreasePositionParams;
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::utils::pda::generate_position_key;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    _market_index: u16, _pool_index: u16, _stable_pool_index: u16, _trade_token_index: u16, _index_trade_token_index: u16, _user_authority_key: Pubkey
)]
pub struct LiquidateIsolatePosition<'info> {
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
        && (pool_mint_vault.mint.eq(& user_token_account.mint) || stable_pool_mint_vault.mint.eq(& user_token_account.mint)),
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
        seeds = [b"pool_mint_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_mint_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), _stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool_mint_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub index_trade_token: AccountLoader<'info, TradeToken>,

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

pub fn handle_liquidate_isolate_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, LiquidateIsolatePosition>,
    position_key: Pubkey,
    _market_index: u16,
    _pool_index: u16,
    _stable_pool_index: u16,
    _index_trade_token_index: u16,
    _user_authority_key: Pubkey,
) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let remaining_accounts = ctx.remaining_accounts;
    let mut market = ctx.accounts.market.load_mut()?;
    let mut trade_token = ctx.accounts.trade_token.load_mut()?;
    let mut oracle_map = OracleMap::load(remaining_accounts)?;
    let mut base_token_pool = ctx.accounts.pool.load_mut()?;
    let mut stable_pool = ctx.accounts.stable_pool.load_mut()?;

    let (is_long, margin_mint, position_size, liquidation_price) = cal_liquidation_price(
        &position_key,
        &user,
        base_token_pool.deref_mut(),
        stable_pool.deref_mut(),
        &ctx.accounts.state,
        &trade_token,
        market.deref_mut(),
        &mut oracle_map,
    )?;

    let index_trade_token = ctx.accounts.index_trade_token.load()?;

    let index_price = oracle_map.get_price_data(&index_trade_token.oracle_key)?;
    if (is_long && index_price.price > liquidation_price)
        || (!is_long && index_price.price < liquidation_price)
    {
        let symbol = market.symbol;
        let user_key = user.key;
        position_processor::decrease_position(
            DecreasePositionParams {
                order_id: 0,
                is_liquidation: true,
                is_portfolio_margin: false,
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
            if is_long {
                &ctx.accounts.pool_mint_vault
            } else {
                &ctx.accounts.stable_pool_mint_vault
            },
            trade_token.deref_mut(),
            &ctx.accounts.trade_token_vault,
            &ctx.accounts.bump_signer,
            &ctx.accounts.token_program,
            &mut oracle_map,
            &generate_position_key(&user_key, symbol, false, ctx.program_id)?,
        )?;
    }
    Ok(())
}

fn cal_liquidation_price(
    position_key: &Pubkey,
    user: &User,
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state: &State,
    trade_token: &TradeToken,
    market: &mut Market,
    oracle_map: &mut OracleMap,
) -> BumpResult<(bool, Pubkey, u128, u128)> {
    let user_position = user.get_user_position_ref(position_key)?;
    let pool = if user_position.is_long { base_token_pool } else { stable_pool };

    validate!(!user_position.is_portfolio_margin, BumpErrorCode::OnlyLiquidateIsolatePosition)?;
    let mut market_processor = MarketProcessor { market };
    market_processor.update_market_funding_fee_rate(
        state,
        oracle_map.get_price_data(&trade_token.oracle_key)?.price,
        trade_token.decimals,
    )?;

    pool.update_pool_borrowing_fee_rate()?;

    let margin_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
    let liquidation_price = user_position.get_liquidation_price(
        market,
        pool,
        state,
        margin_token_price,
        trade_token.decimals,
    )?;
    Ok((
        user_position.is_long,
        user_position.margin_mint_key,
        user_position.position_size,
        liquidation_price,
    ))
}
