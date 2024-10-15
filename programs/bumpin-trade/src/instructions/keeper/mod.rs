pub mod adl;
pub mod collect_rewards;
pub mod execute_portfolio_order;
pub mod execute_wallet_order;
pub mod liquidate_position;
pub mod rebalance;

pub use adl::*;
use anchor_lang::prelude::*;
pub use collect_rewards::*;
pub use execute_portfolio_order::*;
pub use execute_wallet_order::*;
pub use liquidate_position::*;
pub use rebalance::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct LiquidateIsolatePositionParams {
    position_key: Pubkey,
    market_index: u16,
    trade_token_index: u16,
    pool_index: u16,
    stable_pool_index: u16,
    user_authority_key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Eq, PartialEq)]
pub struct ADLParams {
    pool_index: u16,
    stable_pool_index: u16,
    market_index: u16,
    trade_token_index: u16,
    position_key: Pubkey,
    user_authority_key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Eq, PartialEq)]
pub struct ExecuteOrderParams {
    order_id: u64,
    user_authority_key: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Eq, PartialEq)]
pub struct RebalanceMarketStableLossParams {
    pool_index: u16,
    stable_pool_index: u16,
    market_index: u16,
    trade_token_index: u16,
    stable_trade_token_index: u16,
}
