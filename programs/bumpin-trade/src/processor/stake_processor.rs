use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::calculator;
use crate::math::safe_math::SafeMath;
use crate::processor::{fee_processor, user_processor};
use crate::state::infrastructure::user_stake::UserStakeStatus;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::{price, validate};

pub fn stake(
    pool: &mut Pool,
    user: &mut User,
    trade_tokens: &TradeTokenMap,
    oracles: &mut OracleMap,
    requested_token_amount: u128,
) -> BumpResult<u128> {
    let trade_token = trade_tokens.get_trade_token_by_mint_ref(&pool.mint_key).unwrap();
    let token_price = price!(trade_token, oracles);

    validate!(
        pool.config.minimum_stake_amount
            <= calculator::token_to_usd_u(
                requested_token_amount,
                trade_token.decimals,
                token_price
            )?,
        BumpErrorCode::StakeToSmall
    )?;

    //check user stake exist, if not, create new user stake
    let user_stake = user.get_or_new_user_stake_ref_mut(&pool.key)?;
    validate!(
        user_stake.user_stake_status.eq(&UserStakeStatus::USING),
        BumpErrorCode::CouldNotFindUserStake
    )?;

    user_processor::update_account_fee_reward(pool, user)?;

    let stake_fee = fee_processor::charge_staking_fee(pool, requested_token_amount)?;
    let base_mint_amount = requested_token_amount.safe_sub(stake_fee)?;
    Ok(base_mint_amount)
}
