use anchor_lang::prelude::*;

use instructions::*;

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

// declare_id!("88ZPYBftFhJLJLXL2hBHkDcXGEW8MbpqhyCtzkCWyUry");

#[program]
pub mod bumpin_trade {
    use super::*;

    #[track_caller]
    pub fn initialize_state<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeState>,
        param: InitializeStateParams,
    ) -> Result<()> {
        handle_initialize_state(ctx, param).unwrap();
        Ok(())
    }

    #[track_caller]
    pub fn modify_state<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ModifyState>,
        param: ModifyStateParams,
    ) -> Result<()> {
        handle_modify_state(ctx, param)
    }

    #[track_caller]
    pub fn initialize_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializePool>,
        params: InitializePoolParams,
    ) -> Result<()> {
        handle_initialize_pool(ctx, params)
    }

    #[track_caller]
    pub fn initialize_user<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeUser>,
    ) -> Result<()> {
        handle_initialize_user(ctx)
    }

    #[track_caller]
    pub fn initialize_trade_token<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeTradeToken>,
        discount: u32,
        mint_name: [u8; 32],
        liquidation_factor: u32,
    ) -> Result<()> {
        handle_initialize_trade_token(ctx, discount, mint_name, liquidation_factor)
    }

    #[track_caller]
    pub fn initialize_market<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeMarket>,
        params: InitializeMarketParams,
    ) -> Result<()> {
        handle_initialize_market(ctx, params)
    }

    #[track_caller]
    pub fn initialize_rewards(ctx: Context<InitializePoolRewards>, _pool_index: u16) -> Result<()> {
        handle_initialize_rewards(ctx)
    }

    #[track_caller]
    pub fn portfolio_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PortfolioStake>,
        pool_index: u16,
        trade_token_index: u16,
        request_token_amount: u128,
    ) -> Result<()> {
        handle_portfolio_stake(ctx, pool_index, trade_token_index, request_token_amount)
    }

    #[track_caller]
    pub fn wallet_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WalletStake>,
        pool_index: u16,
        request_token_amount: u128,
    ) -> Result<()> {
        handle_wallet_stake(ctx, pool_index, request_token_amount)
    }

    #[track_caller]
    pub fn wallet_un_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WalletUnStake>,
        params: UnStakeParams,
    ) -> Result<()> {
        handle_wallet_un_stake(ctx, params)
    }

    #[track_caller]
    pub fn deposit<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Deposit>,
        token_index: u16,
        amount: u128,
    ) -> Result<()> {
        handle_deposit(ctx, token_index, amount)
    }

    #[track_caller]
    pub fn withdraw<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Withdraw>,
        token_index: u16,
        amount: u128,
    ) -> Result<()> {
        handle_withdraw(ctx, token_index, amount)
    }

    #[track_caller]
    pub fn place_portfolio_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PortfolioPlaceOrder<'c>>,
        order: PlaceOrderParams,
    ) -> Result<()> {
        handle_place_portfolio_order(ctx, order)
    }

    #[track_caller]
    pub fn place_wallet_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, WalletPlaceOrder<'c>>,
        order: PlaceOrderParams,
    ) -> Result<()> {
        handle_place_wallet_order(ctx, order)
    }

    #[track_caller]
    pub fn execute_wallet_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ExecuteWalletOrder<'c>>,
        params: ExecuteOrderParams,
    ) -> Result<()> {
        handle_execute_wallet_order(ctx, params)
    }

    #[track_caller]
    pub fn execute_portfolio_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ExecutePortfolioOrder<'c>>,
        params: ExecuteOrderParams,
    ) -> Result<()> {
        handle_execute_portfolio_order(ctx, params)
    }

    #[track_caller]
    pub fn portfolio_cancel_order(
        ctx: Context<PortfolioCancelOrder>,
        params: CancelOrderParams,
    ) -> Result<()> {
        handle_portfolio_cancel_order(ctx, params)
    }

    #[track_caller]
    pub fn wallet_cancel_order(
        ctx: Context<WalletCancelOrder>,
        params: CancelOrderParams,
    ) -> Result<()> {
        handle_wallet_cancel_order(ctx, params)
    }

    #[track_caller]
    pub fn add_position_margin<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AddPositionMargin>,
        params: UpdatePositionMarginParams,
    ) -> Result<()> {
        handle_add_position_margin(ctx, params)
    }

    #[track_caller]
    pub fn update_cross_position_leverage<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdateCrossPositionLeverage>,
        params: UpdatePositionLeverageParams,
    ) -> Result<()> {
        handle_update_cross_position_leverage(ctx, params)
    }

    #[track_caller]
    pub fn update_isolate_position_leverage<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdateIsolatePositionLeverage>,
        params: UpdatePositionLeverageParams,
    ) -> Result<()> {
        handle_update_isolate_position_leverage(ctx, params)
    }

    pub fn liquidate_isolate_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, LiquidateIsolatePosition>,
        params: LiquidateIsolatePositionParams,
    ) -> Result<()> {
        handle_liquidate_isolate_position(ctx, params)
    }

    pub fn liquidate_cross_position<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, LiquidateCrossPosition<'info>>,
        _user_authority_key: Pubkey,
    ) -> Result<()> {
        handle_liquidate_cross_position(ctx)
    }

    #[track_caller]
    pub fn adl_isolate<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ADLIsolate<'info>>,
        params: ADLParams,
    ) -> Result<()> {
        handle_adl_isolate(ctx, params)
    }

    #[track_caller]
    pub fn adl_cross<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ADLCross<'info>>,
        params: ADLParams,
    ) -> Result<()> {
        handle_adl_cross(ctx, params)
    }

    #[track_caller]
    pub fn claim_rewards<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, ClaimRewards<'c>>,
        params: ClaimRewardsParams,
    ) -> Result<()> {
        handle_claim_rewards(ctx, params)
    }

    #[track_caller]
    pub fn auto_compound<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AutoCompoundRewards<'c>>,
        pool_index: u16,
    ) -> Result<()> {
        handle_auto_compound(ctx, pool_index)
    }

    #[track_caller]
    pub fn collect_rewards<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CollectRewards<'info>>,
        _pool_index: u16,
        _stable_pool_index: u16,
        _trade_token_index: u16,
        _stable_trade_token_index: u16,
    ) -> Result<()> {
        handle_collect_rewards(ctx)
    }

    #[track_caller]
    pub fn auto_rebalance<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AutoRebalance<'info>>,
        _pool_index: u16,
        _trade_token_index: u16,
    ) -> Result<()> {
        handle_auto_rebalance(ctx)
    }

    #[track_caller]
    pub fn rebalance_market_stable_loss<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, RebalanceMarketStableLoss<'info>>,
        _params: RebalanceMarketStableLossParams,
    ) -> Result<()> {
        handle_rebalance_market_stable_loss(ctx)
    }
}
