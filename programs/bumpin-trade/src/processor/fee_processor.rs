use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;

pub fn collect_stake_fee(stake_pool: &mut Pool, amount: u128) -> BumpResult<u128> {
    let fee_amount = amount.safe_mul(stake_pool.pool_config.stake_fee_rate)?;
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;

    Ok(fee_amount)
}

pub fn collect_un_stake_fee(stake_pool: &mut Pool, un_stake_amount: u128) -> BumpResult<u128> {
    let fee_amount = un_stake_amount.safe_mul(stake_pool.pool_config.un_stake_fee_rate)?;
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    Ok(fee_amount)
}

pub fn collect_long_open_position_fee(
    market: &Market,
    pool: &mut Pool,
    margin: u128,
    cross_margin: bool,
) -> BumpResult<u128> {
    let fee_amount = margin.safe_mul(market.market_trade_config.open_fee_rate)?;
    pool.fee_reward.add_fee_amount(fee_amount)?;
    if cross_margin {
        pool.fee_reward.add_un_settle_amount(fee_amount)?;
    }

    Ok(fee_amount)
}

pub fn collect_short_open_position_fee(
    market: &Market,
    pool: &mut Pool,
    stable_pool: &mut Pool,
    state: &State,
    margin: u128,
    cross_margin: bool,
) -> BumpResult<u128> {
    let fee_amount = margin.safe_mul(market.market_trade_config.open_fee_rate)?;

    let usd_pool_rewards_fee = fee_amount.safe_mul(state.trading_fee_usd_pool_rewards_ratio)?;
    let pool_rewards_fee = fee_amount.safe_sub(usd_pool_rewards_fee)?;

    pool.fee_reward.add_fee_amount(pool_rewards_fee)?;
    stable_pool.stable_fee_reward.add_fee_amount(usd_pool_rewards_fee)?;

    if cross_margin {
        pool.fee_reward.add_un_settle_amount(pool_rewards_fee)?;
        stable_pool.stable_fee_reward.add_un_settle_amount(usd_pool_rewards_fee)?;
    }

    Ok(fee_amount)
}

pub fn collect_long_close_position_fee(
    stake_pool: &mut Pool,
    fee_amount: u128,
    cross_margin: bool,
) -> BumpResult<u128> {
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    if cross_margin {
        stake_pool.fee_reward.add_un_settle_amount(fee_amount)?;
    }
    Ok(fee_amount)
}

pub fn collect_short_close_position_fee(
    stable_pool: &mut Pool,
    pool: &mut Pool,
    state: &State,
    close_fee: u128,
    cross_margin: bool,
) -> BumpResult {
    let usd_pool_rewards_fee = close_fee.safe_mul(state.trading_fee_usd_pool_rewards_ratio)?;
    let left_rewards = close_fee.safe_sub(usd_pool_rewards_fee)?;

    stable_pool.fee_reward.add_fee_amount(usd_pool_rewards_fee)?;
    pool.stable_fee_reward.add_fee_amount(left_rewards)?;
    if cross_margin {
        stable_pool.fee_reward.add_un_settle_amount(usd_pool_rewards_fee)?;
        pool.stable_fee_reward.add_un_settle_amount(left_rewards)?;
    }

    Ok(())
}

pub fn collect_borrowing_fee(
    stake_pool: &mut Pool,
    fee_amount: u128,
    cross_margin: bool,
) -> BumpResult<u128> {
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    if cross_margin {
        stake_pool.fee_reward.add_un_settle_amount(fee_amount)?;
    }

    Ok(fee_amount)
}
