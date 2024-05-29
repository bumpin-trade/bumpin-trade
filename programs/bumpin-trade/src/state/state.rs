use anchor_lang::account;
use solana_program::pubkey::Pubkey;
use crate::state::infrastructure::fee_reward::FeeReward;
use anchor_lang::AnchorSerialize;
use borsh::io;

#[account]
#[derive(Default)]
#[repr(C)]
pub struct State {
    pub admin: Pubkey,
    pub bump_signer: Pubkey,
    pub keeper_signer: Pubkey,
    pub bump_signer_nonce: u8,
    pub number_of_markets: u16,
    pub number_of_pools: u16,
    pub number_of_trade_tokens: u16,
    pub min_order_margin_usd: u128,
    pub max_maintenance_margin_rate: u128,
    pub funding_fee_base_rate: u128,
    pub max_funding_base_rate: u128,
    pub trading_fee_staking_rewards_ratio: u128,
    pub trading_fee_pool_rewards_ratio: u128,
    pub borrowing_fee_staking_rewards_ratio: u128,
    pub borrowing_fee_pool_rewards_ratio: u128,
    pub min_precision_multiple: u128,
    pub mint_fee_staking_rewards_ratio: u128,
    pub mint_fee_pool_rewards_ratio: u128,
    pub redeem_fee_staking_rewards_ratio: u128,
    pub redeem_fee_pool_rewards_ratio: u128,
    pub pool_rewards_interval_limit: u128,
    pub init_fee: u128,
    pub open_fee_rate: u128,
    pub staking_fee_reward: FeeReward,
    pub dao_fee_reward: FeeReward,
}