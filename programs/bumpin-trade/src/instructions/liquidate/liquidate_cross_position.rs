use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::user_processor::UserProcessor;
use crate::state::market_map::MarketMap;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool_map::PoolMap;
use crate::state::state::State;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct LiquidateCrossPosition<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    pub user: AccountLoader<'info, User>,

    pub keeper_signer: Signer<'info>,
}

pub fn handle_liquidate_cross_position(ctx: Context<LiquidateCrossPosition>, user: Pubkey) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;

    let mut remaining_accounts = &mut ctx.remaining_accounts.iter().peekable();
    let mut oracle_map = OracleMap::load(remaining_accounts)?;
    let mut market_map = MarketMap::load(remaining_accounts)?;
    let mut pool_map = PoolMap::load(remaining_accounts)?;
    let trade_token_map = TradeTokenMap::load(remaining_accounts)?;

    let mut user_processor = UserProcessor { user };
    user_processor.cancel_all_orders()?;

    for user_position in user_processor.user.user_positions {
        let mut pool = &mut pool_map.get_mut_ref(&user_position.margin_mint)?;
        let mut pool_processor = PoolProcessor { pool };
        pool_processor.update_pool_borrowing_fee_rate()?;
        drop(pool_processor);

        let market = &mut market_map.get_mut_ref(&user_position.symbol)?;
        let mut market_processor = MarketProcessor { market };
        market_processor.update_market_funding_fee_rate(&ctx.accounts.state, &mut oracle_map)?;
        drop(market_processor);
    }
    let portfolio_net_value = user_processor.get_portfolio_net_value(&trade_token_map, &mut oracle_map)?;
    let used_value = user_processor.get_total_used_value(&trade_token_map, &mut oracle_map)?;
    let (total_im_usd, total_un_pnl_usd, total_position_fee, total_position_mm) = user_processor.get_user_cross_position_value(&ctx.accounts.state, &market_map, &pool_map, &mut oracle_map)?;
    let cross_net_value = portfolio_net_value.safe_add(total_im_usd)?.safe_add(user.hold)?.cast::<i128>()?.
        safe_add(total_un_pnl_usd)?.
        safe_sub(used_value.cast()?)?.
        safe_sub(total_position_fee)?;
    if cross_net_value <= 0 || cross_net_value.cast::<u128>()? <= total_position_mm {}
    Ok(())
}