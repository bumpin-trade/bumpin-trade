use anchor_lang::emit;
use anchor_lang::prelude::AccountLoader;

use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::UserRewardsUpdateEvent;
use crate::state::pool::Pool;
use crate::state::user::User;

pub fn update_account_fee_reward(
    user_loader: &AccountLoader<User>,
    pool_loader: &AccountLoader<Pool>,
) -> BumpResult {
    let user = &mut user_loader.load_mut().unwrap();
    let stake_pool = pool_loader.load().unwrap();
    let user_key = user.user_key;

    let user_stake = user.get_user_stake_mut_ref(&stake_pool.key)?;

    let fee_reward = stake_pool.fee_reward;
    if user_stake.user_rewards.open_rewards_per_stake_token
        != fee_reward.cumulative_rewards_per_stake_token
        && user_stake.staked_share > 0
        && fee_reward
            .cumulative_rewards_per_stake_token
            .safe_sub(user_stake.user_rewards.open_rewards_per_stake_token)?
            > fee_reward.get_rewards_delta_limit()?
    {
        let realised_rewards_token_amount = stake_pool
            .fee_reward
            .cumulative_rewards_per_stake_token
            .safe_sub(user_stake.user_rewards.open_rewards_per_stake_token)?
            .safe_mul_small_rate(user_stake.staked_share)?;
        user_stake.add_user_rewards(realised_rewards_token_amount)?;
    }
    user_stake.user_rewards.open_rewards_per_stake_token =
        fee_reward.cumulative_rewards_per_stake_token;
    let user_rewards = user_stake.user_rewards.clone();
    emit!(UserRewardsUpdateEvent { user_key, token_mint: stake_pool.key, user_rewards });
    Ok(())
}
