use anchor_lang::prelude::*;

use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::traits::{MarketIndexOffset, Size};

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug, Default)]
#[repr(C)]
pub struct TradeToken {
    pub discount: u128,
    pub liquidation_factor: u128,
    pub total_liability: u128,
    pub total_amount: u128,
    pub mint: Pubkey,
    pub oracle: Pubkey,
    pub trade_token_vault: Pubkey,
    pub token_index: u16,
    pub decimals: u16,
    pub mint_name: [u8; 32],
    pub padding: [u8; 12],
}

impl Size for TradeToken {
    const SIZE: usize = std::mem::size_of::<TradeToken>() + 8;
}

impl TradeToken {
    pub fn add_token(&mut self, amount: u128) -> BumpResult {
        self.total_amount = self.total_amount.safe_add(amount)?;
        Ok(())
    }
    pub fn sub_token(&mut self, amount: u128) -> BumpResult {
        self.total_amount = self.total_amount.safe_sub(amount)?;
        Ok(())
    }
    pub fn add_liability(&mut self, amount: u128) -> BumpResult {
        self.total_amount = self.total_liability.safe_add(amount)?;
        Ok(())
    }

    pub fn sub_liability(&mut self, amount: u128) -> BumpResult {
        self.total_amount = self.total_liability.safe_sub(amount)?;
        Ok(())
    }
}

impl MarketIndexOffset for TradeToken {
    const MARKET_INDEX_OFFSET: usize = 8;
}
