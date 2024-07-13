use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::state::State;

#[derive(Accounts)]
// #[instruction(param: InitializeStateParams)]
pub struct InitializeState<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        seeds = [b"bump_state".as_ref()],
        space = std::mem::size_of::< State > () + 8,
        bump,
        payer = admin
    )]
    pub state: Account<'info, State>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct InitializeStateParams {
    pub keeper_key: [u8; 32],
    pub min_order_margin_usd: u128, //最小下单头寸 param: InitializeStateParams
    pub maximum_maintenance_margin_rate: u32, //最大维持保证金率，类似于用来做adl
    pub funding_fee_base_rate: u128, //fundingfee 基础费率
    pub max_funding_base_rate: u128, //最大fundinfee率
    pub trading_fee_staking_rewards_ratio: u128, //stake reward最小单位
    pub trading_fee_pool_rewards_ratio: u128, // pool reward最小单位
    pub trading_fee_usd_pool_rewards_ratio: u128, // 稳定币pool reward最小单位
    pub borrowing_fee_staking_rewards_ratio: u128,
    pub borrowing_fee_pool_rewards_ratio: u128,
    pub min_precision_multiple: u128,
    pub mint_fee_staking_rewards_ratio: u128,
    pub mint_fee_pool_rewards_ratio: u128,
    pub redeem_fee_staking_rewards_ratio: u128,
    pub redeem_fee_pool_rewards_ratio: u128,
    pub pool_rewards_interval_limit: u128,
    pub init_fee: u64,
    pub staking_fee_reward_ratio: u32,
    pub pool_fee_reward_ratio: u32,
}

pub fn handle_initialize_state(
    ctx: Context<InitializeState>,
    initialize_state_params: InitializeStateParams,
) -> Result<()> {
    let (bump_signer, bump_signer_nonce) =
        Pubkey::find_program_address(&[b"bump_state".as_ref()], ctx.program_id);
    *ctx.accounts.state = State {
        admin: *ctx.accounts.admin.key,
        bump_signer,
        keeper_key: Pubkey::new_from_array(initialize_state_params.keeper_key),
        bump_signer_nonce,
        market_sequence: 0,
        pool_sequence: 0,
        trade_token_sequence: 0,
        minimum_order_margin_usd: initialize_state_params.min_order_margin_usd,
        maximum_maintenance_margin_rate: initialize_state_params.maximum_maintenance_margin_rate,
        funding_fee_base_rate: initialize_state_params.funding_fee_base_rate,
        maximum_funding_base_rate: initialize_state_params.max_funding_base_rate,
        minimum_precision_multiple: initialize_state_params.min_precision_multiple,
        pool_rewards_interval_limit: initialize_state_params.pool_rewards_interval_limit,
        init_fee: initialize_state_params.init_fee,
        trading_fee_usd_pool_rewards_ratio: 0,
        pool_fee_reward_ratio: initialize_state_params.pool_fee_reward_ratio,
        reserve_padding: [0u8; 32],
    };
    Ok(())
}
