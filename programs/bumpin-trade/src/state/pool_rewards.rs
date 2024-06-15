use anchor_lang::prelude::*;
use crate::state::dao_rewards::DaoRewards;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::traits::Size;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct PoolRewards {
    pub pool_index: u16,
    pub poo_rewards_vault: Pubkey,
    pub un_claim_amount: u128,
    pub total_rewards_amount: u128,
}

impl Size for PoolRewards {
    const SIZE: usize = std::mem::size_of::<PoolRewards>() + 8;
}
