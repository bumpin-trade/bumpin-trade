use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::calculator;
use crate::state::infrastructure::user_stake::UserStake;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::validate;

#[track_caller]
pub fn un_stake(
    pool: &Pool,
    user: &mut User,
    un_stake_amount: u128,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
) -> BumpResult<u128> {
    let base_trade_token = trade_token_map.get_trade_token_by_mint_ref(&pool.mint_key)?;
    let net_price = pool.get_pool_net_price(trade_token_map, oracle_map, market_map)?;
    let un_stake_usd =
        calculator::token_to_usd_u(un_stake_amount, base_trade_token.decimals, net_price)?;
    let token_price = oracle_map.get_price_data(&base_trade_token.feed_id)?.price;
    let token_amount =
        calculator::usd_to_token_u(un_stake_usd, base_trade_token.decimals, token_price)?;
    validate!(un_stake_usd > pool.config.minimum_un_stake_amount, BumpErrorCode::UnStakeTooSmall)?;
    validate!(
        token_amount
            <= pool.get_pool_available_liquidity(market_map, oracle_map, trade_token_map,)?,
        BumpErrorCode::UnStakeWithAmountNotEnough
    )?;

    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;
    user_stake.sub_staked_share(un_stake_amount)?;

    Ok(token_amount)
}

#[track_caller]
pub fn stake(
    pool: &mut Pool,
    mint_amount: u128,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
) -> BumpResult<u128> {
    let mut supply_amount = mint_amount;
    let trade_token = trade_token_map.get_trade_token_by_mint_ref(&pool.mint_key)?;
    if pool.total_supply > 0 {
        let oracle_price_data = oracle_map.get_price_data(&trade_token.feed_id)?;

        supply_amount = calculator::usd_to_token_u(
            calculator::token_to_usd_u(mint_amount, trade_token.decimals, oracle_price_data.price)?,
            trade_token.decimals,
            pool.get_pool_net_price(trade_token_map, oracle_map, market_map)?,
        )?;
    }
    Ok(supply_amount)
}

#[track_caller]
pub fn portfolio_to_stake(
    user: &mut User,
    pool: &mut Pool,
    mint_amount: u128,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
    _state: &State,
) -> BumpResult<(u128, UserStake)> {
    let mut supply_amount = mint_amount;
    let trade_token = trade_token_map.get_trade_token_by_mint_ref(&pool.mint_key)?;

    let user_token = user.get_user_token_ref(&pool.mint_key)?;
    validate!(user_token.amount > mint_amount, BumpErrorCode::AmountNotEnough)?;

    user.sub_user_token_amount(&pool.mint_key, mint_amount)?;
    validate!(
        user.get_available_value(trade_token_map, oracle_map)? > 0,
        BumpErrorCode::AmountNotEnough
    )?;
    if pool.total_supply > 0 {
        let oracle_price_data = oracle_map.get_price_data(&trade_token.feed_id)?;

        supply_amount = calculator::usd_to_token_u(
            calculator::token_to_usd_u(mint_amount, trade_token.decimals, oracle_price_data.price)?,
            trade_token.decimals,
            pool.get_pool_net_price(trade_token_map, oracle_map, market_map)?,
        )?;
    }
    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;
    user_stake.add_staked_share(supply_amount)?;
    Ok((supply_amount, *user_stake))
}
