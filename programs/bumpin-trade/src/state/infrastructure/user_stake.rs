use crate::errors::{BumpErrorCode, BumpResult};
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::math::safe_math::SafeMath;
use crate::validate;

#[zero_copy(unsafe)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct UserStake {
    pub user_stake_status: UserStakeStatus,
    pub pool_key: Pubkey,
    pub amount: u128,
    pub user_rewards: UserRewards,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UserStakeStatus {
    #[default]
    INIT,
    USING,
}

impl UserStake {
    pub fn add_user_stake(&mut self, stake_amount: u128) -> BumpResult {
        self.amount = self.amount.safe_add(stake_amount)?;
        Ok(())
    }

    pub fn sub_user_stake(&mut self, stake_amount: u128) -> BumpResult {
        validate!(self.amount >= stake_amount, BumpErrorCode::AmountNotEnough)?;
        self.amount = self.amount.safe_sub(stake_amount)?;
        Ok(())
    }

    pub fn add_user_rewards(&mut self, rewards: u128) -> BumpResult {
        self.user_rewards.realised_rewards_token_amount =
            self.user_rewards.realised_rewards_token_amount.safe_add(rewards)?;
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
