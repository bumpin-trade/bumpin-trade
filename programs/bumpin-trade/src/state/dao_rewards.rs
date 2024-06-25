use crate::traits::Size;
use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug, Default)]
#[repr(C)]
pub struct DaoRewards {
    pub pool_index: u16,
    pub dao_rewards_vault: Pubkey,
    pub un_claim_amount: u128,
    pub total_rewards_amount: u128,
    pub padding: [u8; 6],
}

impl Size for DaoRewards {
    const SIZE: usize = std::mem::size_of::<DaoRewards>() + 8;
}
