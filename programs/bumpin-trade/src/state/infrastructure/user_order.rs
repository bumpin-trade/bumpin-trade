use anchor_lang::prelude::*;
use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

#[bumpin_zero_copy_unsafe]
pub struct UserOrder {
    pub order_margin: u128,
    pub order_size: u128,
    pub trigger_price: u128,
    pub acceptable_price: u128,
    pub created_at: i64,
    pub order_id: u64,
    pub margin_mint_key: Pubkey,
    pub authority: Pubkey,
    pub user_token_account: Pubkey,
    pub symbol: [u8; 32],
    pub leverage: u32,
    pub order_side: OrderSide,
    pub position_side: PositionSide,
    pub order_type: OrderType,
    pub stop_type: StopType,
    pub status: OrderStatus,
    pub is_portfolio_margin: bool,
    pub padding: [u8; 6],
    pub reserve_padding: [u8; 16],
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

    pub fn print(&self) {
        let symbol_str = std::str::from_utf8(&self.symbol).unwrap_or("Invalid UTF-8");
        msg!(
            "Order Margin: {}, Order Size: {}, Trigger Price: {}, Acceptable Price: {}, Created At: {}, Order ID: {}, Margin Mint Key: {}, Authority: {}, Symbol: {}, Leverage: {}, Order Side: {:?}, Position Side: {:?}, Order Type: {:?}, Stop Type: {:?}, Status: {:?}, Is Portfolio Margin: {}, Padding: {:?}, Reserve Padding: {:?}",
            self.order_margin,
            self.order_size,
            self.trigger_price,
            self.acceptable_price,
            self.created_at,
            self.order_id,
            self.margin_mint_key,
            self.authority,
            symbol_str,
            self.leverage,
            self.order_side,
            self.position_side,
            self.order_type,
            self.stop_type,
            self.status,
            self.is_portfolio_margin,
            self.padding,
            self.reserve_padding,
        );
    }
}
