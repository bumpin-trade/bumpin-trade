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

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ModifyState<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Account<'info, State>,
    pub admin: Signer<'info>,
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

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct ModifyStateParams {
    pub min_order_margin_usd: Option<u128>,
    pub maximum_maintenance_margin_rate: Option<u32>,
    pub funding_fee_base_rate: Option<u128>,
    pub max_funding_base_rate: Option<u128>,
    pub trading_fee_staking_rewards_ratio: Option<u32>,
    pub trading_fee_pool_rewards_ratio: Option<u32>,
    pub trading_fee_usd_pool_rewards_ratio: Option<u32>,
    pub min_precision_multiple: Option<u128>,
    pub pool_rewards_interval_limit: Option<u128>,
    pub init_fee: Option<u64>,
    pub staking_fee_reward_ratio: Option<u32>,
    pub pool_fee_reward_ratio: Option<u32>,
    pub essential_account_alt: Option<[u8; 32]>,
}

#[track_caller]
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
        essential_account_alt: Pubkey::default(),
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
        padding: [0u8; 5],
        reserve_padding: [0u8; 32],
    };
    Ok(())
}

#[track_caller]
pub fn handle_modify_state(
    ctx: Context<ModifyState>,
    modify_state_params: ModifyStateParams,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    if let Some(min_order_margin_usd) = modify_state_params.min_order_margin_usd {
        state.minimum_order_margin_usd = min_order_margin_usd;
    }
    if let Some(maximum_maintenance_margin_rate) =
        modify_state_params.maximum_maintenance_margin_rate
    {
        state.maximum_maintenance_margin_rate = maximum_maintenance_margin_rate;
    }
    if let Some(funding_fee_base_rate) = modify_state_params.funding_fee_base_rate {
        state.funding_fee_base_rate = funding_fee_base_rate;
    }
    if let Some(max_funding_base_rate) = modify_state_params.max_funding_base_rate {
        state.maximum_funding_base_rate = max_funding_base_rate;
    }
    if let Some(trading_fee_staking_rewards_ratio) =
        modify_state_params.trading_fee_staking_rewards_ratio
    {
        state.trading_fee_usd_pool_rewards_ratio = trading_fee_staking_rewards_ratio;
    }
    if let Some(trading_fee_pool_rewards_ratio) = modify_state_params.trading_fee_pool_rewards_ratio
    {
        state.trading_fee_usd_pool_rewards_ratio = trading_fee_pool_rewards_ratio;
    }
    if let Some(trading_fee_usd_pool_rewards_ratio) =
        modify_state_params.trading_fee_usd_pool_rewards_ratio
    {
        state.trading_fee_usd_pool_rewards_ratio = trading_fee_usd_pool_rewards_ratio;
    }
    if let Some(min_precision_multiple) = modify_state_params.min_precision_multiple {
        state.minimum_precision_multiple = min_precision_multiple;
    }
    if let Some(pool_rewards_interval_limit) = modify_state_params.pool_rewards_interval_limit {
        state.pool_rewards_interval_limit = pool_rewards_interval_limit;
    }
    if let Some(init_fee) = modify_state_params.init_fee {
        state.init_fee = init_fee;
    }
    if let Some(pool_fee_reward_ratio) = modify_state_params.pool_fee_reward_ratio {
        state.pool_fee_reward_ratio = pool_fee_reward_ratio;
    }
    if let Some(essential_account_alt) = modify_state_params.essential_account_alt {
        state.essential_account_alt = Pubkey::new_from_array(essential_account_alt);
    }
    Ok(())
}
