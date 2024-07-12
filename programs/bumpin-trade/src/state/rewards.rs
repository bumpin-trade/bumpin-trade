use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::traits::Size;
use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Default, Debug)]
#[repr(C)]
pub struct Rewards {
    pub pool_un_claim_amount: u128,
    pub pool_total_rewards_amount: u128,
    pub pool_rewards_vault: Pubkey,
    pub dao_rewards_vault: Pubkey,
    pub dao_total_rewards_amount: u128,
    pub pool_index: u16,
    pub padding: [u8; 14],
    pub reserve_padding: [u8; 32],
}

impl Size for Rewards {
    const SIZE: usize = std::mem::size_of::<Rewards>() + 8;
}

impl Rewards {
    pub fn add_pool_un_claim_rewards(&mut self, amount: u128) -> BumpResult<()> {
        self.pool_un_claim_amount = self.pool_un_claim_amount.safe_add(amount)?;
        Ok(())
    }

    pub fn sub_pool_un_claim_rewards(&mut self, amount: u128) -> BumpResult<()> {
        if amount >= self.pool_un_claim_amount {
            self.pool_un_claim_amount = 0u128;
        } else {
            self.pool_un_claim_amount = self.pool_un_claim_amount.safe_sub(amount)?;
        }
        Ok(())
    }

    pub fn add_pool_total_rewards_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.pool_total_rewards_amount = self.pool_total_rewards_amount.safe_add(amount)?;
        Ok(())
    }

    pub fn add_dao_total_rewards_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.dao_total_rewards_amount = self.dao_total_rewards_amount.safe_add(amount)?;
        Ok(())
    }
}
