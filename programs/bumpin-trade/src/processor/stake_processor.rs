use anchor_lang::prelude::AccountLoader;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::state::infrastructure::user_stake::UserStakeStatus;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::validate;

pub fn stake(
    pool_account_loader: &AccountLoader<Pool>,
    user_account_loader: &AccountLoader<User>,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    request_token_amount: u128,
) -> BumpResult<u128> {
    let mut pool = &mut pool_account_loader
        .load_mut()
        .map_err(|_| BumpErrorCode::UnableToLoadAccountLoader)?;
    let user =
        &mut user_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadUserData)?;
    let trade_token = trade_token_map.get_trade_token_ref(&pool.mint_key)?;

    let token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;

    validate!(
        pool.pool_config.minimum_stake_amount
            <= cal_utils::token_to_usd_u(request_token_amount, trade_token.decimals, token_price)?,
        BumpErrorCode::StakeToSmall
    )?;

    //check user stake exist, if not, create new user stake
    let user_stake = user.get_or_add_user_stake_ref_mut(&pool.key)?;
    validate!(
        user_stake.user_stake_status.eq(&UserStakeStatus::USING),
        BumpErrorCode::CouldNotFindUserStake
    )?;

    update_account_fee_reward(user_account_loader, pool_account_loader)?;

    let stake_fee = fee_processor::collect_stake_fee(&mut pool, request_token_amount)?;
    let base_mint_amount = request_token_amount.safe_sub(stake_fee)?;
    Ok(base_mint_amount)
}
