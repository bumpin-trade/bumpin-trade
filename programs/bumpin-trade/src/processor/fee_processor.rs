use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;

pub fn collect_stake_fee(stake_pool: &mut Pool, state: &State, amount: u128) -> BumpResult<u128> {
    let fee_amount = amount.
        safe_mul(stake_pool.pool_config.stake_fee_rate)?;

    let pool_rewards_fee = fee_amount.safe_mul(state.mint_fee_pool_rewards_ratio)?;
    let staking_rewards_fee = fee_amount.safe_mul(state.mint_fee_staking_rewards_ratio)?;
    // let dao_rewards_fee = fee_amount.safe_sub(pool_rewards_fee)?.safe_sub(staking_rewards_fee)?;

    stake_pool.fee_reward.add_fee_amount(pool_rewards_fee)?;
    // state.dao_fee_reward.add_fee_amount(dao_rewards_fee)?;
    // state.staking_fee_reward.add_fee_amount(staking_rewards_fee)?;

    Ok(fee_amount)
}

pub fn collect_un_stake_fee(stake_pool: &mut Pool, state: &State, un_stake_amount: u128) -> BumpResult<u128> {
    let fee_amount = un_stake_amount.safe_mul(stake_pool.pool_config.un_stake_fee_rate)?;

    let pool_rewards_fee = fee_amount.safe_mul(state.redeem_fee_pool_rewards_ratio)?;
    let staking_rewards_fee = fee_amount.safe_mul(state.redeem_fee_staking_rewards_ratio)?;
    // let dao_rewards_fee = fee_amount.safe_sub(pool_rewards_fee)?.safe_sub(staking_rewards_fee)?;

    stake_pool.fee_reward.add_fee_amount(pool_rewards_fee)?;
    // state.dao_fee_reward.add_fee_amount(dao_rewards_fee)?;
    // state.staking_fee_reward.add_fee_amount(staking_rewards_fee)?;

    Ok(fee_amount)
}

pub fn collect_open_position_fee(market: &Market, pool: &mut Pool, state: &State, margin: u128, cross_margin: bool) -> BumpResult<u128> {
    let fee_amount = margin.safe_mul(market.market_trade_config.open_fee_rate)?;

    let pool_rewards_fee = fee_amount.safe_mul(state.trading_fee_pool_rewards_ratio)?;
    // let staking_rewards_fee = fee_amount.safe_mul(state.trading_fee_staking_rewards_ratio)?;
    // let dao_rewards_fee = fee_amount.safe_sub(pool_rewards_fee)?.safe_sub(staking_rewards_fee)?;


    pool.fee_reward.add_fee_amount(pool_rewards_fee)?;
    // state.dao_fee_reward.add_fee_amount(dao_rewards_fee)?;
    // state.staking_fee_reward.add_fee_amount(staking_rewards_fee)?;

    if cross_margin {
        pool.fee_reward.add_un_settle_amount(pool_rewards_fee)?;
        // state.dao_fee_reward.add_un_settle_amount(dao_rewards_fee)?;
        // state.staking_fee_reward.add_un_settle_amount(staking_rewards_fee)?;
    }

    Ok(fee_amount.cast::<u128>()?)
}

pub fn collect_close_position_fee(stake_pool: &mut Pool, state: &State, fee_amount: u128, cross_margin: bool) -> BumpResult<u128> {
    let pool_rewards_fee = fee_amount.safe_mul(state.trading_fee_pool_rewards_ratio)?;
    let staking_rewards_fee = fee_amount.safe_mul(state.trading_fee_staking_rewards_ratio)?;
    let dao_rewards_fee = fee_amount.safe_sub(pool_rewards_fee)?.safe_sub(staking_rewards_fee)?;


    stake_pool.fee_reward.add_fee_amount(pool_rewards_fee)?;
    // state.dao_fee_reward.add_fee_amount(dao_rewards_fee)?;
    // state.staking_fee_reward.add_fee_amount(staking_rewards_fee)?;

    if cross_margin {
        stake_pool.fee_reward.add_un_settle_amount(pool_rewards_fee)?;
        // state.dao_fee_reward.add_un_settle_amount(dao_rewards_fee)?;
        // state.staking_fee_reward.add_un_settle_amount(staking_rewards_fee)?;
    }

    Ok(fee_amount.cast::<u128>()?)
}

pub fn collect_borrowing_fee(stake_pool: &mut Pool, state: &State, fee_amount: u128, cross_margin: bool) -> BumpResult<()> {
    let pool_rewards_fee = fee_amount.safe_mul(state.borrowing_fee_pool_rewards_ratio)?;
    let staking_rewards_fee = fee_amount.safe_mul(state.borrowing_fee_staking_rewards_ratio)?;
    // let dao_rewards_fee = fee_amount.safe_sub(pool_rewards_fee)?.safe_sub(staking_rewards_fee)?;

    stake_pool.fee_reward.add_fee_amount(pool_rewards_fee)?;
    // state.dao_fee_reward.add_fee_amount(dao_rewards_fee)?;
    // state.staking_fee_reward.add_fee_amount(staking_rewards_fee)?;

    if cross_margin {
        stake_pool.fee_reward.add_un_settle_amount(pool_rewards_fee)?;
        // state.dao_fee_reward.add_un_settle_amount(dao_rewards_fee)?;
        // state.staking_fee_reward.add_un_settle_amount(staking_rewards_fee)?;
    }

    Ok(())
}