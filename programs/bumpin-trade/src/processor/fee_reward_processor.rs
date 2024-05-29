use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::user_stake::UserStake;
use crate::state::pool::Pool;

pub fn update_account_fee_reward(account: &mut UserStake, stake_pool: &Pool) {
    let fee_reward = stake_pool.fee_reward;
    if account.user_rewards.open_rewards_per_stake_token != fee_reward.cumulative_rewards_per_stake_token &&
        account.amount > 0 &&
        fee_reward.cumulative_rewards_per_stake_token.safe_sub(account.user_rewards.open_rewards_per_stake_token)? > fee_reward.get_rewards_delta_limit()? {
        let realised_rewards_token_amount = stake_pool.fee_reward.cumulative_rewards_per_stake_token
            .safe_sub(account.user_rewards.open_rewards_per_stake_token)?
            .safe_mul_small_rate(account.amount)?;
        account.add_user_rewards(realised_rewards_token_amount);
    }
    account.user_rewards.open_rewards_per_stake_token = fee_reward.cumulative_rewards_per_stake_token;
}