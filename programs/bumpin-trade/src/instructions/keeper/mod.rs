pub mod adl;
pub mod collect_rewards;
pub mod execute_order;
pub mod liquidate_position;
pub mod rebalance;
pub mod update_user_status;

pub use adl::*;
use anchor_lang::prelude::*;
pub use collect_rewards::*;
pub use execute_order::*;
pub use liquidate_position::*;
pub use rebalance::*;
pub use update_user_status::*;
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct LiquidateIsolatePositionParams {
    position_key: Pubkey,
    market_index: u16,
    trade_token_index: u16,
    pool_index: u16,
    stable_pool_index: u16,
    user_authority_key: Pubkey,
}
