use anchor_lang::prelude::*;
use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

#[bumpin_zero_copy_unsafe]
pub struct UserOrder {
    pub order_margin: u128,
    pub leverage: u32,
    pub order_size: u128,
    pub trigger_price: u128,
    pub acceptable_price: u128,
    pub created_at: i64,
    pub order_id: u64,
    pub order_side: OrderSide,
    pub position_side: PositionSide,
    pub order_type: OrderType,
    pub stop_type: StopType,
    pub status: OrderStatus,
    pub margin_mint_key: Pubkey,
    pub authority: Pubkey,
    pub symbol: [u8; 32],
    pub cross_margin: bool,
    pub padding: [u8; 10],
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub enum OrderSide {
    #[default]
    NONE,
    LONG,
    SHORT,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub enum OrderStatus {
    #[default]
    INIT,
    USING,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub enum PositionSide {
    #[default]
    NONE,
    INCREASE,
    DECREASE,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub enum OrderType {
    #[default]
    NONE,
    MARKET,
    LIMIT,
    STOP,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub enum StopType {
    #[default]
    NONE,
    StopLoss,
    TakeProfit,
}

impl UserOrder {
    pub fn set_leverage(&mut self, leverage: u32) {
        self.leverage = leverage;
    }
}
