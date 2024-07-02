use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::user_stake::UserStake;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::validate;

pub fn un_stake(
    pool: &Pool,
    user: &mut User,
    un_stake_amount: u128,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
) -> BumpResult<u128> {
    let trade_token = trade_token_map.get_trade_token_ref(&pool.mint_key)?;
    let pool_value = pool.get_pool_usd_value(trade_token_map, oracle_map, market_map)?;

    let un_stake_usd = cal_utils::mul_div_u(un_stake_amount, pool_value, pool.total_supply)?;
    let pool_price = oracle_map.get_price_data(&trade_token.oracle_key)?;
    let token_amount = cal_utils::div_u128(un_stake_usd, pool_price.price)?;
    validate!(token_amount > pool.config.minimum_un_stake_amount, BumpErrorCode::UnStakeTooSmall)?;

    let max_un_stake_amount = pool.get_current_max_un_stake()?;
    validate!(token_amount < max_un_stake_amount, BumpErrorCode::UnStakeTooLarge)?;

    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;
    user_stake.sub_staked_share(un_stake_amount)?;

    Ok(token_amount)
}

pub fn stake(
    pool: &mut Pool,
    mint_amount: u128,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
) -> BumpResult<u128> {
    let mut supply_amount = mint_amount;
    let trade_token = trade_token_map.get_trade_token_ref(&pool.mint_key)?;
    if pool.total_supply > 0 {
        let oracle_price_data = oracle_map.get_price_data(&trade_token.oracle_key)?;

        supply_amount =
            cal_utils::token_to_usd_u(mint_amount, trade_token.decimals, oracle_price_data.price)?
                .safe_div(pool.get_pool_net_price(
                    trade_token_map,
                    oracle_map,
                    market_map,
                )?)?;
    }
    Ok(supply_amount)
}

pub fn portfolio_to_stake(
    user: &mut User,
    pool: &mut Pool,
    mint_amount: u128,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
) -> BumpResult<(u128, UserStake)> {
    let mut supply_amount = mint_amount;
    let trade_token = trade_token_map.get_trade_token_ref(&pool.mint_key)?;

    let user_token = user.get_user_token_ref(&pool.mint_key)?;
    validate!(user_token.amount > mint_amount, BumpErrorCode::AmountNotEnough)?;

    user.sub_user_token_amount(&pool.mint_key, mint_amount)?;
    validate!(
        user.get_available_value(oracle_map, trade_token_map)? > 0,
        BumpErrorCode::AmountNotEnough
    )?;
    if pool.total_supply > 0 {
        let oracle_price_data = oracle_map.get_price_data(&trade_token.oracle_key)?;

        supply_amount =
            cal_utils::token_to_usd_u(mint_amount, trade_token.decimals, oracle_price_data.price)?
                .safe_div(pool.get_pool_net_price(
                    trade_token_map,
                    oracle_map,
                    market_map,
                )?)?;
    }
    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;
    user_stake.add_staked_share(supply_amount)?;
    Ok((supply_amount, *user_stake))
}
