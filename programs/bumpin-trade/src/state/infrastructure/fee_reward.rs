use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::safe_math::SafeMath;
use crate::validate;
use anchor_lang::prelude::*;
use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

#[bumpin_zero_copy_unsafe]
pub struct FeeReward {
    /// Fees generated from staking, redeeming, and position operations (such as increasing or decreasing positions).
    /// Used in isolated margin mode (since fees are actually transferred each time they are generated).
    /// See: [`fee_processor::collect_long_open_position_fee`]
    pub fee_amount: u128,

    /// Accounting for fees generated from cross-margin position operations.
    pub un_settle_fee_amount: u128,

    /// Cursor that records how many tokens each stake share can receive.
    /// This increases during keeper collect operations.
    /// Users can calculate their rewards based on this cursor and the corresponding field in their UserStake.
    pub cumulative_rewards_per_stake_token: u128,

    /// Records the deltas of the last three keeper collect operations.
    /// Each time the keeper calls collect, a delta is recorded here.
    /// When distributing rewards to users, it must be determined if the user has experienced a sufficiently long period (i.e., three keeper calls).
    pub last_rewards_per_stake_token_deltas: [u128; 3],
}

impl FeeReward {
    pub fn get_rewards_delta_limit(&self) -> BumpResult<u128> {
        let mut delta_limit = 0u128;
        for delta in self.last_rewards_per_stake_token_deltas {
            delta_limit = delta_limit.safe_add(delta)?;
        }
        Ok(delta_limit)
    }
    pub fn add_fee_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.fee_amount = self.fee_amount.safe_add(amount)?;
        Ok(())
    }

    pub fn sub_fee_amount(&mut self, amount: u128) -> BumpResult<()> {
        if amount >= self.fee_amount {
            self.fee_amount = 0u128;
        } else {
            self.fee_amount = self.fee_amount.safe_sub(amount)?;
        }
        Ok(())
    }

    pub fn add_un_settle_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.un_settle_fee_amount = self.un_settle_fee_amount.safe_add(amount)?;
        Ok(())
    }
    pub fn sub_un_settle_amount(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.un_settle_fee_amount >= amount, BumpErrorCode::AmountNotEnough)?;
        self.un_settle_fee_amount = self.un_settle_fee_amount.safe_sub(amount)?;
        Ok(())
    }

    pub fn add_cumulative_rewards_per_stake_token(&mut self, amount: u128) -> BumpResult<()> {
        self.cumulative_rewards_per_stake_token =
            self.cumulative_rewards_per_stake_token.safe_add(amount)?;
        Ok(())
    }

    pub fn push_last_rewards_per_stake_token_deltas(&mut self, delta: u128) -> BumpResult<()> {
        // Shift elements to the right
        for i in (1..self.last_rewards_per_stake_token_deltas.len()).rev() {
            self.last_rewards_per_stake_token_deltas[i] =
                self.last_rewards_per_stake_token_deltas[i - 1];
        }
        // Insert the new delta at the first position
        self.last_rewards_per_stake_token_deltas[0] = delta;
        Ok(())
    }
}
