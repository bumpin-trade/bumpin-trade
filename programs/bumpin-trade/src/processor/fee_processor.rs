use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;

pub fn collect_stake_fee(stake_pool: &mut Pool, amount: u128) -> BumpResult<u128> {
    let fee_amount = amount.safe_mul_rate(stake_pool.config.stake_fee_rate.into())?;
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;

    Ok(fee_amount)
}

pub fn collect_un_stake_fee(stake_pool: &mut Pool, un_stake_amount: u128) -> BumpResult<u128> {
    let fee_amount = un_stake_amount.safe_mul_rate(stake_pool.config.un_stake_fee_rate.into())?;
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    Ok(fee_amount)
}

pub fn collect_long_open_position_fee(
    market: &Market,
    pool: &mut Pool,
    margin: u128,
    is_portfolio_margin: bool,
) -> BumpResult<u128> {
    let fee_amount = margin.safe_mul_rate(market.config.open_fee_rate)?;
    pool.fee_reward.add_fee_amount(fee_amount)?;
    if is_portfolio_margin {
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
    is_portfolio_margin: bool,
) -> BumpResult<u128> {
    let fee_amount = margin.safe_mul_rate(market.config.open_fee_rate)?;

    let usd_pool_rewards_fee =
        fee_amount.safe_mul_rate(state.trading_fee_usd_pool_rewards_ratio as u128)?;
    let pool_rewards_fee = fee_amount.safe_sub(usd_pool_rewards_fee)?;

    pool.stable_fee_reward.add_fee_amount(pool_rewards_fee)?;
    stable_pool.fee_reward.add_fee_amount(usd_pool_rewards_fee)?;

    if is_portfolio_margin {
        pool.stable_fee_reward.add_un_settle_amount(pool_rewards_fee)?;
        stable_pool.fee_reward.add_un_settle_amount(usd_pool_rewards_fee)?;
    }

    Ok(fee_amount)
}

pub fn collect_long_close_position_fee(
    stake_pool: &mut Pool,
    fee_amount: u128,
    is_portfolio_margin: bool,
) -> BumpResult<u128> {
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    if is_portfolio_margin {
        stake_pool.fee_reward.add_un_settle_amount(fee_amount)?;
    }
    Ok(fee_amount)
}

pub fn collect_short_close_position_fee(
    stable_pool: &mut Pool,
    pool: &mut Pool,
    state: &State,
    close_fee: u128,
    is_portfolio_margin: bool,
) -> BumpResult {
    let usd_pool_rewards_fee =
        close_fee.safe_mul_rate(state.trading_fee_usd_pool_rewards_ratio as u128)?;
    let left_rewards = close_fee.safe_sub(usd_pool_rewards_fee)?;

    stable_pool.fee_reward.add_fee_amount(usd_pool_rewards_fee)?;
    pool.stable_fee_reward.add_fee_amount(left_rewards)?;
    if is_portfolio_margin {
        stable_pool.fee_reward.add_un_settle_amount(usd_pool_rewards_fee)?;
        pool.stable_fee_reward.add_un_settle_amount(left_rewards)?;
    }

    Ok(())
}

pub fn collect_borrowing_fee(
    stake_pool: &mut Pool,
    fee_amount: u128,
    is_portfolio_margin: bool,
) -> BumpResult<u128> {
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    if is_portfolio_margin {
        stake_pool.fee_reward.add_un_settle_amount(fee_amount)?;
    }

    Ok(fee_amount)
}

pub fn settle_funding_fee(
    base_token_pool: &mut Pool,
    stable_token_pool: &mut Pool,
    fee_amount_usd: i128,
    fee_amount: i128,
    is_long: bool,
    is_cross: bool,
) -> BumpResult<()> {
    if !is_long {
        if fee_amount_usd <= 0i128 {
            //stable_pool should pay to user, count loss on base_token_pool
            base_token_pool.add_stable_loss_amount(fee_amount_usd.cast::<u128>()?)?;
            stable_token_pool.add_unsettle(fee_amount_usd.cast::<u128>()?)?;
        } else {
            if is_cross {
                stable_token_pool.add_unsettle(fee_amount_usd.cast::<u128>()?)?;
            } else {
                //user should pay to stable_pool, count amount on base_token_pool
                base_token_pool.add_stable_amount(fee_amount_usd.cast::<u128>()?)?;
            }
        }
        stable_token_pool.update_pool_funding_fee(fee_amount, false)?;
    } else {
        if fee_amount_usd <= 0i128 {
            //base_token_pool should pay to user, count amount on base_token_pool
            base_token_pool.sub_amount(fee_amount.cast::<u128>()?)?;
        } else {
            if is_cross {
                //user should pay to base_token_pool, count amount on base_token_pool
                base_token_pool.add_unsettle(fee_amount.cast::<u128>()?)?;
            } else {
                base_token_pool.add_amount(fee_amount.cast::<u128>()?)?;
            }
        }
        base_token_pool.update_pool_funding_fee(fee_amount, false)?;
    }
    Ok(())
}
