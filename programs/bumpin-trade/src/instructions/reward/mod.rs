pub mod claim_reward;

use anchor_lang::prelude::*;
pub use claim_reward::*;

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Eq, PartialEq)]
pub struct ClaimRewardsParams {
    pool_index: u16,
}
