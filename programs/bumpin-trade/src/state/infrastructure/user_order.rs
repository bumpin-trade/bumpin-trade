use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub struct UserOrder {
    pub authority: Pubkey,
    pub order_id: u128,
    pub symbol: [u8; 32],
    pub order_side: OrderSide,
    pub position_side: PositionSide,
    pub order_type: OrderType,
    pub stop_type: StopType,
    pub cross_margin: bool,
    pub margin_mint: Pubkey,
    pub order_margin: u128,
    pub leverage: u128,
    pub order_size: u128,
    pub trigger_price: u128,
    pub acceptable_price: u128,
    pub time: u128,
    pub status: OrderStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub enum OrderSide {
    #[default]
    NONE,
    LONG,
    SHORT,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub enum OrderStatus {
    #[default]
    INIT,
    USING,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub enum PositionSide {
    #[default]
    NONE,
    INCREASE,
    DECREASE,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub enum OrderType {
    #[default]
    NONE,
    MARKET,
    LIMIT,
    STOP,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub enum StopType {
    #[default]
    NONE,
    StopLoss,
    TakeProfit,
}

impl UserOrder {
    pub fn set_leverage(&mut self, leverage: u128) {
        self.leverage = leverage;
    }
}
