use anchor_lang::prelude::AccountLoader;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::safe_math::SafeMath;
use crate::state::pool::Pool;
use crate::state::user::User;

pub fn update_account_fee_reward(
    user_loader: &AccountLoader<User>,
    pool_loader: &AccountLoader<Pool>,
) -> BumpResult {
    let user = &mut user_loader.load_mut().unwrap();
    let stake_pool = pool_loader.load().unwrap();

    let user_stake =
        user.get_user_stake_mut(&stake_pool.pool_key)?.ok_or(BumpErrorCode::StakePaused)?;

    let fee_reward = stake_pool.fee_reward;
    if user_stake.user_rewards.open_rewards_per_stake_token
        != fee_reward.cumulative_rewards_per_stake_token
        && user_stake.amount > 0
        && fee_reward
            .cumulative_rewards_per_stake_token
            .safe_sub(user_stake.user_rewards.open_rewards_per_stake_token)?
            > fee_reward.get_rewards_delta_limit()?
    {
        let realised_rewards_token_amount = stake_pool
            .fee_reward
            .cumulative_rewards_per_stake_token
            .safe_sub(user_stake.user_rewards.open_rewards_per_stake_token)?
            .safe_mul_small_rate(user_stake.amount)?;
        user_stake.add_user_rewards(realised_rewards_token_amount)?;
    }
    user_stake.user_rewards.open_rewards_per_stake_token =
        fee_reward.cumulative_rewards_per_stake_token;
    Ok(())
}
