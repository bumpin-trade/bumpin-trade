pub mod portfolio_place_order;
pub mod wallet_place_order;

use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::user_order::{OrderSide, OrderType, PositionSide, StopType};
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::*;
pub use portfolio_place_order::*;
pub use wallet_place_order::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct PlaceOrderParams {
    pub symbol: [u8; 32],
    pub size: u128,
    pub order_margin: u128,
    pub leverage: u32,
    pub trigger_price: u128,
    pub acceptable_price: u128,
    pub place_time: i64,
    pub is_portfolio_margin: bool,
    pub is_native_token: bool,
    pub order_side: OrderSide,
    pub position_side: PositionSide,
    pub order_type: OrderType,
    pub stop_type: StopType,
}

fn validate_place_order(
    order: &PlaceOrderParams,
    token: &Pubkey,
    market: &Market,
    pool: &Pool,
    state: &State,
    token_price: u128,
) -> BumpResult<bool> {
    match order.order_type {
        OrderType::NONE => Ok(false),
        _ => {
            if order.position_side.eq(&PositionSide::DECREASE) && order.size == 0u128 {
                Ok(false)
            } else if order.order_side.eq(&OrderSide::NONE) {
                Ok(false)
            } else if order.order_type.eq(&OrderType::LIMIT)
                && order.position_side.eq(&PositionSide::DECREASE)
            {
                Ok(false)
            } else if order.order_type.eq(&OrderType::STOP)
                && (order.stop_type.eq(&StopType::NONE) || order.trigger_price == 0u128)
            {
                Ok(false)
            } else if order.position_side.eq(&PositionSide::INCREASE) {
                if order.order_margin == 0u128 {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::LONG) && !token.eq(&market.pool_mint_key)
                {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::LONG)
                    && !market.pool_mint_key.eq(&pool.mint_key)
                {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::SHORT) && !pool.mint_key.eq(token) {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::SHORT)
                    && !market.stable_pool_mint_key.eq(&pool.mint_key)
                {
                    Ok(false)
                } else if order.is_portfolio_margin
                    && order.order_margin < state.minimum_order_margin_usd
                {
                    Ok(false)
                } else if !order.is_portfolio_margin
                    && order.order_margin.safe_mul(token_price)? < state.minimum_order_margin_usd
                {
                    Ok(false)
                } else {
                    Ok(true)
                }
            } else {
                Ok(true)
            }
        },
    }
}
