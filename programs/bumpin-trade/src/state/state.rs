use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
#[repr(C)]
pub struct State {
    pub admin: Pubkey,
    pub bump_signer: Pubkey,
    pub keeper_key: Pubkey,
    pub essential_account_alt: Pubkey,
    pub funding_fee_base_rate: u128,
    pub maximum_funding_base_rate: u128,
    pub minimum_precision_multiple: u128,
    pub pool_rewards_interval_limit: u128,
    pub minimum_order_margin_usd: u128,
    pub init_fee: u64,
    pub trading_fee_usd_pool_rewards_ratio: u32,
    pub maximum_maintenance_margin_rate: u32,
    pub pool_fee_reward_ratio: u32,
    pub market_sequence: u16,
    pub pool_sequence: u16,
    pub trade_token_sequence: u16,
    pub bump_signer_nonce: u8,
    pub padding: [u8; 5],
    pub reserve_padding: [u8; 32],
}
