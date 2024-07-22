use crate::errors::{BumpErrorCode, BumpResult};
use anchor_lang::prelude::*;
use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::math::safe_math::SafeMath;
use crate::validate;

/// Represents a user's staking information
///
/// When a user stakes in an asset pool [`Pool`], a [`UserStake`] object is created to record the user's staking information.
/// Like other user information, it is a reusable object. See: [`UserStakeStatus::INIT`]
#[bumpin_zero_copy_unsafe]
pub struct UserStake {
    /// User's staking shares
    ///
    /// This value represents the user's staking shares (in USD), calculated as:
    /// ((amount of staked tokens - fees) * current token value) / current net value of the pool
    pub staked_share: u128,
    /// Rewards earned by the user from staking
    pub user_rewards: UserRewards,
    /// The pool in which the user has staked
    pub pool_key: Pubkey,
    /// The status of the user's stake
    pub user_stake_status: UserStakeStatus,
    /// Padding for alignment
    pub padding: [u8; 15],
    /// Reserved for future use
    pub reserve_padding: [u8; 16],
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
    pub total_claim_rewards_amount: u128,
    pub realised_rewards_token_amount: u128,
    pub open_rewards_per_stake_token: u128,
    pub token_key: Pubkey,
}
