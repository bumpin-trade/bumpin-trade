use anchor_lang::prelude::*;

use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::traits::{MarketIndexOffset, Size};

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug, Default)]
#[repr(C)]
pub struct TradeToken {
    pub total_liability: u128,
    pub total_amount: u128,
    pub mint_key: Pubkey,
    pub oracle_key: Pubkey,
    pub vault_key: Pubkey,
    pub name: [u8; 32],
    pub discount: u32,           // 10^5
    pub liquidation_factor: u32, // 10^5
    pub index: u16,
    pub decimals: u16,
    pub padding: [u8; 4],
}

impl Size for TradeToken {
    const SIZE: usize = std::mem::size_of::<TradeToken>() + 8;
}

impl TradeToken {
    pub fn add_amount(&mut self, amount: u128) -> BumpResult {
        self.total_amount = self.total_amount.safe_add(amount)?;
        Ok(())
    }
    pub fn sub_amount(&mut self, amount: u128) -> BumpResult {
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
