pub mod macros;
pub mod errors;
pub mod state;
pub mod instructions;
pub mod ids;
pub mod math;
pub mod traits;
pub mod utils;
pub mod processor;

use anchor_lang::prelude::*;
use instructions::*;
declare_id!("GhzHdLjZ1qLLPnPq6YdeqJAszuBRN8WnLnK455yBbig6");

#[program]
pub mod bumpin_trade {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    /*-----pool pool------*/
    pub fn pool_stake(ctx: Context<PoolStake>, pool_index: usize, trade_index: u16, params: StakeParams) -> Result<()> {
        handle_pool_stake(ctx, pool_index, trade_index, params)
    }

    pub fn pool_un_stake(ctx: Context<PoolUnStake>, pool_index: usize, params: UnStakeParams) -> Result<()> {
        handle_pool_un_stake(ctx, pool_index, params)
    }

    /*-----account------*/
    pub fn deposit(ctx: Context<Deposit>, amount: u128) -> Result<()> { Ok(()) }

    pub fn withdraw(ctx: Context<Deposit>, amount: u128) -> Result<()> { Ok(()) }

    /*-----order------*/
    pub fn place_order(ctx: Context<PlaceOrder>, params: PlaceOrderParams,
    ) -> Result<()> {
        handle_place_order(ctx, params)
    }

    pub fn execute_order(ctx: Context<PlaceOrder>, order_id: u128,
    ) -> Result<()> {
        handle_execute_order(ctx, state::infrastructure::user_order::UserOrder::default(), order_id, true)
    }

    pub fn cancel_order(ctx: Context<CancelOrderCtx>, order_id: u128, reason_code: u128) -> Result<()> {
        handle_cancel_order(ctx, order_id, reason_code)
    }

    /*-----position------*/
    pub fn add_position_margin(ctx: Context<AddPositionMargin>, params: UpdatePositionMarginParams) -> Result<()> {
        handle_add_position_margin(ctx, params)
    }

    pub fn update_position_leverage(ctx: Context<UpdatePositionLeverage>, params: UpdatePositionLeverageParams) -> Result<()> {
        handle_update_position_leverage(ctx, params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
