use anchor_lang::prelude::*;
use crate::errors::BumpErrorCode::RewardsNotFound;
use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;

#[derive(Default, Eq, PartialEq, Debug, Clone, Copy)]
pub struct FeeReward {
    pub fee_amount: u128,
    pub un_settle_fee_amount: u128,
    pub open_cumulative_rewards_per_stake_token: u128,
    pub cumulative_rewards_per_stake_token: u128,
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

    pub fn add_un_settle_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.fee_amount = self.un_settle_fee_amount.safe_add(amount)?;
        Ok(())
    }
}
