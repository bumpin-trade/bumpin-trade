use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::market::Market;
use crate::state::pool::Pool;
use anchor_lang::prelude::msg;

#[track_caller]
pub fn charge_staking_fee(stake_pool: &mut Pool, amount: u128) -> BumpResult<u128> {
    let fee_amount = amount.safe_mul_rate(stake_pool.config.stake_fee_rate.into())?;
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;

    Ok(fee_amount)
}

#[track_caller]
pub fn collect_un_stake_fee(stake_pool: &mut Pool, un_stake_amount: u128) -> BumpResult<u128> {
    let fee_amount = un_stake_amount.safe_mul_rate(stake_pool.config.un_stake_fee_rate.into())?;
    stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    Ok(fee_amount)
}

#[track_caller]
pub fn collect_open_position_fee(
    market: &Market,
    pool: &mut Pool,
    margin: u128,
    is_portfolio_margin: bool,
) -> BumpResult<u128> {
    let fee_amount = margin.safe_mul_rate(market.config.open_fee_rate)?;
    if is_portfolio_margin {
        pool.fee_reward.add_un_settle_amount(fee_amount)?;
    } else {
        pool.fee_reward.add_fee_amount(fee_amount)?;
    }

    Ok(fee_amount)
}

#[track_caller]
pub fn collect_close_position_fee(
    stake_pool: &mut Pool,
    fee_amount: u128,
    is_portfolio_margin: bool,
) -> BumpResult<u128> {
    if is_portfolio_margin {
        stake_pool.fee_reward.add_un_settle_amount(fee_amount)?;
    } else {
        stake_pool.fee_reward.add_fee_amount(fee_amount)?;
    }
    Ok(fee_amount)
}

#[track_caller]
pub fn collect_borrowing_fee(
    pool: &mut Pool,
    fee_amount: u128,
    is_portfolio_margin: bool,
) -> BumpResult<u128> {
    if is_portfolio_margin {
        pool.fee_reward.add_un_settle_amount(fee_amount)?;
    } else {
        pool.fee_reward.add_fee_amount(fee_amount)?;
    }

    Ok(fee_amount)
}

#[track_caller]
pub fn settle_funding_fee(pool: &mut Pool, fee_amount: i128, is_cross: bool) -> BumpResult<()> {
    msg!("=======settle_funding_fee, fee_amount:{}", fee_amount);
    msg!("=======settle_funding_fee, is_cross:{}", is_cross);
    if fee_amount <= 0i128 {
        //pool should pay to user
        pool.sub_amount(fee_amount.abs().cast::<u128>()?)?;
    } else if is_cross {
        pool.add_unsettle(fee_amount.abs().cast::<u128>()?)?;
    } else {
        //user should pay to stable_pool
        pool.add_amount(fee_amount.abs().cast::<u128>()?)?;
    }
    pool.settle_pool_funding_fee(fee_amount)?;
    Ok(())
}
