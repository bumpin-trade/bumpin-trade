use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;
use crate::traits::{MarketIndexOffset, Size};

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug, Default)]
#[repr(C)]
pub struct TradeToken {
    pub mint: Pubkey,
    pub oracle: Pubkey,
    pub token_index: u16,
    pub discount: u128,
    pub liquidation_factor: u128,
    pub decimals: u8,
    pub trade_token_vault: Pubkey,
}

impl Size for TradeToken {
    const SIZE: usize = std::mem::size_of::<TradeToken>() + 8;
}

impl MarketIndexOffset for TradeToken {
    const MARKET_INDEX_OFFSET: usize = 8;
}