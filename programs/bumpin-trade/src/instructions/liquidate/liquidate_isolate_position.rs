use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;
use crate::errors::{BumpErrorCode};
use crate::processor::market_processor::MarketProcessor;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::position_processor::PositionProcessor;
use crate::state::market::Market;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::validate;
use solana_program::msg;

#[derive(Accounts)]
#[instruction(market_index: u16, trade_token_index: u16, user: Pubkey)]
pub struct LiquidateIsolatePosition<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"market_index", market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        address = market.load() ?.pool_key
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        address = market.load() ?.stable_pool_key
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"trade_token", trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    pub keeper_signer: Signer<'info>,

}

pub fn handle_liquidate_isolate_position(ctx: Context<LiquidateIsolatePosition>, market_index: u16, trade_token_index: u16, position_key: Pubkey, user: Pubkey) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;
    let user_position = user.find_position_mut_by_key(&position_key)?;

    validate!(!user_position.cross_margin, BumpErrorCode::OnlyLiquidateIsolatePosition);
    let market = &mut ctx.accounts.market.load_mut()?;

    let remaining_accounts = &mut ctx.remaining_accounts.iter().peekable();
    let mut oracle_map = OracleMap::load(remaining_accounts)?;

    let mut market_processor = MarketProcessor { market };
    market_processor.update_market_funding_fee_rate(&ctx.accounts.state, &mut oracle_map)?;

    let mut pool = &mut ctx.accounts.pool.load_mut().unwrap();
    let mut stable_pool = &mut ctx.accounts.stable_pool.load_mut().unwrap();
    let mut pool_processor = if user_position.is_long { PoolProcessor { pool } } else { PoolProcessor { pool: stable_pool } };
    pool_processor.update_pool_borrowing_fee_rate()?;

    let mut position_processor = PositionProcessor { position: user_position };
    let liquidation_price = position_processor.get_liquidation_price(&market, &pool, &ctx.accounts.state, &mut oracle_map)?;

    let index_price = oracle_map.get_price_data(&user_position.index_mint)?;
    if (user_position.is_long && index_price.price > liquidation_price) ||
        (!user_position.is_long && index_price.price < liquidation_price) {
        //  position_processor.decrease_position()
    }
    Ok(())
}