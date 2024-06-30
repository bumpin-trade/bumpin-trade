use crate::errors::{BumpErrorCode, BumpResult};
use anchor_lang::prelude::*;
use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::math::safe_math::SafeMath;
use crate::validate;

#[bumpin_zero_copy_unsafe]
pub struct UserStake {
    pub staked_share: u128, //old: amount
    pub user_rewards: UserRewards,
    pub pool_key: Pubkey,
    pub user_stake_status: UserStakeStatus,
    pub padding: [u8; 15],
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UserStakeStatus {
    #[default]
    INIT,
    USING,
}

impl UserStake {
    pub fn add_staked_share(&mut self, stake_amount: u128) -> BumpResult {
        self.staked_share = self.staked_share.safe_add(stake_amount)?;
        Ok(())
    }

    pub fn sub_staked_share(&mut self, stake_amount: u128) -> BumpResult {
        validate!(self.staked_share >= stake_amount, BumpErrorCode::AmountNotEnough)?;
        self.staked_share = self.staked_share.safe_sub(stake_amount)?;
        Ok(())
    }

    pub fn add_user_rewards(&mut self, rewards: u128) -> BumpResult {
        self.user_rewards.realised_rewards_token_amount =
            self.user_rewards.realised_rewards_token_amount.safe_add(rewards)?;
        Ok(())
    }
}

#[bumpin_zero_copy_unsafe]
pub struct UserRewards {
    pub realised_rewards_token_amount: u128,
    pub open_rewards_per_stake_token: u128,
    pub token_key: Pubkey,
    pub padding: [u8; 8],
}
