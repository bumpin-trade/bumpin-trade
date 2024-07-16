use anchor_lang::prelude::*;

use instructions::*;

use crate::state::user::UserStatus;

pub mod errors;
pub mod ids;
pub mod instructions;
pub mod macros;
pub mod math;
pub mod processor;
pub mod state;
pub mod traits;
pub mod utils;

declare_id!("Ap5HaA55b1SrhMeBeiivgpbpA7ffTUtc64zcUJx7ionR");

#[program]
pub mod bumpin_trade {
    use super::*;

    pub fn initialize_state<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeState>,
        param: InitializeStateParams,
    ) -> Result<()> {
        handle_initialize_state(ctx, param)
    }

    pub fn initialize_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializePool>,
        params: InitializePoolParams,
    ) -> Result<()> {
        handle_initialize_pool(ctx, params)
    }

    pub fn initialize_user<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeUser>,
    ) -> Result<()> {
        handle_initialize_user(ctx)
    }

    pub fn initialize_trade_token<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeTradeToken>,
        discount: u32,
        mint_name: [u8; 32],
        liquidation_factor: u32,
    ) -> Result<()> {
        handle_initialize_trade_token(ctx, discount, mint_name, liquidation_factor)
    }

    pub fn initialize_market<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeMarket>,
        params: InitializeMarketParams,
    ) -> Result<()> {
        handle_initialize_market(ctx, params)
    }

    pub fn initialize_rewards(ctx: Context<InitializePoolRewards>, _pool_index: u16) -> Result<()> {
        handle_initialize_rewards(ctx)
    }

    /*-----pool pool------*/
    pub fn portfolio_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PortfolioStake>,
        pool_index: u16,
        trade_token_index: u16,
        request_token_amount: u128,
    ) -> Result<()> {
        handle_portfolio_stake(ctx, pool_index, trade_token_index, request_token_amount)
    }

    pub fn wallet_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WalletStake>,
        pool_index: u16,
        trade_token_index: u16,
        request_token_amount: u128,
    ) -> Result<()> {
        handle_wallet_stake(ctx, pool_index, trade_token_index, request_token_amount)
    }

    pub fn portfolio_un_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PortfolioUnStake>,
        params: UnStakeParams,
    ) -> Result<()> {
        handle_portfolio_un_stake(ctx, params).unwrap();
        Ok(())
    }

    pub fn wallet_un_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WalletUnStake>,
        params: UnStakeParams,
    ) -> Result<()> {
        handle_wallet_un_stake(ctx, params).unwrap();
        Ok(())
    }

    /*-----account------*/
    pub fn deposit<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Deposit>,
        token_index: u16,
        amount: u128,
    ) -> Result<()> {
        handle_deposit(ctx, token_index, amount)
    }

    pub fn withdraw<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Withdraw>,
        amount: u128,
    ) -> Result<()> {
        handle_withdraw(ctx, amount)
    }

    /*-----order------*/
    pub fn place_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PlaceOrder<'c>>,
        order: PlaceOrderParams,
    ) -> Result<()> {
        handle_place_order(ctx, order)
    }

    pub fn execute_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ExecuteOrder<'c>>,
        order_id: u64,
        _user_key: Pubkey,
    ) -> Result<()> {
        handle_execute_order(ctx, order_id)
    }

    pub fn cancel_order(
        ctx: Context<CancelOrderCtx>,
        order_id: u64,
        _pool_index: u16,
    ) -> Result<()> {
        handle_cancel_order(ctx, order_id, _pool_index)
    }

    /*-----position------*/
    pub fn add_position_margin<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AddPositionMargin>,
        params: UpdatePositionMarginParams,
    ) -> Result<()> {
        handle_add_position_margin(ctx, params)
    }

    pub fn update_position_leverage<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdatePositionLeverage>,
        params: UpdatePositionLeverageParams,
    ) -> Result<()> {
        handle_update_position_leverage(ctx, params)
    }

    pub fn liquidate_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, LiquidatePosition>,
        position_key: Pubkey,
        liquidation_price: u128,
        _market_index: u16,
        _pool_index: u16,
        _stable_pool_index: u16,
        _user_authority_key: Pubkey,
    ) -> Result<()> {
        handle_liquidate_position(ctx, position_key, liquidation_price)
    }

    /*-----adl------*/
    pub fn adl<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ADL<'info>>,
        _pool_index: u16,
        _stable_pool_index: u16,
        _market_index: u16,
        _trade_token_index: u16,
        params: [ADLParams; 10],
    ) -> Result<()> {
        handle_adl(ctx, params)
    }

    pub fn update_user_status(
        ctx: Context<UpdateUserStatus>,
        user_status: UserStatus,
        user_authority_key: Pubkey,
    ) -> Result<()> {
        handle_update_user_status(ctx, user_status, user_authority_key)
    }

    /*-----reward------*/
    pub fn claim_rewards<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimRewards<'c>>,
        _pool_index: u16,
    ) -> Result<()> {
        handle_claim_rewards(ctx)
    }

    pub fn auto_compound<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AutoCompoundRewards<'c>>,
        pool_index: u16,
    ) -> Result<()> {
        handle_auto_compound(ctx, pool_index)
    }

    pub fn collect_rewards<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CollectRewards<'info>>,
        _pool_index: u16,
        _stable_pool_index: u16,
        _trade_token_index: u16,
        _stable_trade_token_index: u16,
    ) -> Result<()> {
        handle_collect_rewards(ctx)
    }
}
