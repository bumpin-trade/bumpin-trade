use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solana_program::pubkey::Pubkey;

use crate::state::state::State;

#[derive(Accounts)]
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
    pub min_order_margin_usd: u128,
    pub max_maintenance_margin_rate: u128,
    pub funding_fee_base_rate: u128,
    pub max_funding_base_rate: u128,
    pub trading_fee_staking_rewards_ratio: u128,
    pub trading_fee_pool_rewards_ratio: u128,
    pub trading_fee_usd_pool_rewards_ratio: u128,
    pub borrowing_fee_staking_rewards_ratio: u128,
    pub borrowing_fee_pool_rewards_ratio: u128,
    pub min_precision_multiple: u128,
    pub mint_fee_staking_rewards_ratio: u128,
    pub mint_fee_pool_rewards_ratio: u128,
    pub redeem_fee_staking_rewards_ratio: u128,
    pub redeem_fee_pool_rewards_ratio: u128,
    pub pool_rewards_interval_limit: u128,
    pub init_fee: u64,
    pub staking_fee_reward_ratio: u128,
    pub pool_fee_reward_ratio: u128,
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
        keeper_signer: Pubkey::default(),
        bump_signer_nonce,
        number_of_markets: 0,
        number_of_pools: 0,
        number_of_trade_tokens: 0,
        min_order_margin_usd: initialize_state_params.min_order_margin_usd,
        max_maintenance_margin_rate: initialize_state_params.max_maintenance_margin_rate,
        funding_fee_base_rate: initialize_state_params.funding_fee_base_rate,
        max_funding_base_rate: initialize_state_params.max_funding_base_rate,
        min_precision_multiple: initialize_state_params.min_precision_multiple,
        pool_rewards_interval_limit: initialize_state_params.pool_rewards_interval_limit,
        init_fee: initialize_state_params.init_fee,
        trading_fee_usd_pool_rewards_ratio: 0,
        staking_fee_reward_ratio: initialize_state_params.staking_fee_reward_ratio,
        pool_fee_reward_ratio: initialize_state_params.pool_fee_reward_ratio,
    };
    Ok(())
}
