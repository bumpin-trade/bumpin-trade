use anchor_lang::account;
use solana_program::pubkey::Pubkey;
use crate::state::infrastructure::fee_reward::FeeReward;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct DaoRewards {
    pub pool_index: u16,
    pub dao_rewards_vault: Pubkey,
    pub rewards: FeeReward,
}