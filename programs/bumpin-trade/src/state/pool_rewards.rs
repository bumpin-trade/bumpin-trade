use anchor_lang::prelude::*;

use crate::traits::Size;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct PoolRewards {
    pub un_claim_amount: u128,
    pub total_rewards_amount: u128,
    pub pool_rewards_vault: Pubkey,
    pub pool_index: u16,
    pub padding: [u8; 14],
}

impl Size for PoolRewards {
    const SIZE: usize = std::mem::size_of::<PoolRewards>() + 8;
}
