use anchor_lang::prelude::AccountLoader;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{cal_utils, StakeParams};
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::state::infrastructure::user_stake::{UserStake, UserStakeStatus};
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
    stake_params: &StakeParams,
) -> BumpResult<u128> {
    let mut pool = &mut pool_account_loader
        .load_mut()
        .map_err(|_| BumpErrorCode::UnableToLoadAccountLoader)?;
    let user =
        &mut user_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadUserData)?;
    let trade_token = trade_token_map.get_trade_token(&pool.pool_mint)?;

    let token_price = oracle_map.get_price_data(&trade_token.oracle)?.price;

    validate!(
        pool.pool_config.mini_stake_amount
            <= cal_utils::token_to_usd_u(
                stake_params.request_token_amount,
                trade_token.decimals,
                token_price
            )?,
        BumpErrorCode::StakeToSmall
    )?;

    let user_stake_option = user.get_user_stake_mut(&pool.pool_key)?;
    //make sure user_stake exist
    match user_stake_option {
        None => {
            //add default user_stake to user
            let res = &mut UserStake {
                user_stake_status: UserStakeStatus::USING,
                pool_key: pool.pool_key,
                amount: 0,
                user_rewards: Default::default(),
                padding: [0; 15],
            };

            let next_index = user.next_usable_stake_index()?;
            user.add_user_stake(res, next_index)?;
            res
        },
        Some(user_stake) => user_stake,
    };

    update_account_fee_reward(user_account_loader, pool_account_loader)?;

    let stake_fee = fee_processor::collect_stake_fee(&mut pool, stake_params.request_token_amount)?;
    let base_mint_amount = stake_params.request_token_amount.safe_sub(stake_fee)?;
    Ok(base_mint_amount)
}
