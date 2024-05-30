use anchor_lang::zero_copy;
use solana_program::pubkey::Pubkey;
use crate::errors::BumpResult;

use crate::math::safe_math::SafeMath;

#[zero_copy(unsafe)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct UserStake {
    pub pool_key: Pubkey,
    pub amount: u128,
    pub user_rewards: UserRewards,
}

impl UserStake {
    pub fn add_user_stake(&mut self, stake_amount: u128) -> BumpResult {
        self.amount = self.amount.
            safe_add(stake_amount)?;
        Ok(())
    }

    pub fn sub_user_stake(&mut self, stake_amount: u128) -> BumpResult {
        self.amount = self.amount.
            safe_sub(stake_amount)?;
        Ok(())
    }

    pub fn add_user_rewards(&mut self, rewards: u128) -> BumpResult {
        self.user_rewards.realised_rewards_token_amount = self.user_rewards.realised_rewards_token_amount.safe_add(rewards)?;
        Ok(())
    }
}

#[zero_copy(unsafe)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct UserRewards {
    pub token: Pubkey,
    pub realised_rewards_token_amount: u128,
    pub open_rewards_per_stake_token: u128,
}