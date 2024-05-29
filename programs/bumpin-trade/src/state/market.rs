use anchor_lang::prelude::*;
use crate::errors::BumpResult;
use crate::instructions::cal_utils;
use crate::state::infrastructure::market_funding_fee::MarketFundingFee;
use crate::traits::Size;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Market {
    pub symbol: [u8; 32],
    pub ticker_size: u128,
    pub pool_key: Pubkey,
    pub pool_mint_key: Pubkey,
    pub index: u128,
    pub index_mint_key: Pubkey,
    pub index_mint_key_decimal: u128,
    pub stable_pool_key: Pubkey,
    pub stable_pool_mint_key: Pubkey,
    pub long_open_interest: MarketPosition,
    pub short_open_interest: MarketPosition,
    pub funding_fee: MarketFundingFee,
    pub market_trade_config: MarketConfig,
}

impl Default for Market {
    fn default() -> Self {
        Market {
            symbol: [0; 32],
            ticker_size: 0u128,
            pool_key: Pubkey::default(),
            pool_mint_key: Pubkey::default(),
            index_mint_key: Pubkey::default(),
            index: 0u128,
            stable_pool_key: Pubkey::default(),
            stable_pool_mint_key: Default::default(),
            long_open_interest: MarketPosition::default(),
            short_open_interest: MarketPosition::default(),
            funding_fee: MarketFundingFee::default(),
            market_trade_config: MarketConfig::default(),
            index_mint_key_decimal: 0,
        }
    }
}

impl Size for Market {
    const SIZE: usize = std::mem::size_of::<Market>() + 8;
}

#[zero_copy(unsafe)]
#[derive(Eq, Default, AnchorSerialize, AnchorDeserialize, PartialEq, Debug)]
#[repr(C)]
pub struct MarketPosition {
    pub open_interest: u128,
    pub entry_price: u128,
}

impl MarketPosition {
    pub fn add_open_interest(&mut self, size: u128, price: u128) -> BumpResult<()> {
        self.open_interest = cal_utils::add_u128(self.open_interest, size)?;
        self.entry_price = price;
        Ok(())
    }
    pub fn sub_open_interest(&mut self, size: u128) -> BumpResult<()> {
        if self.open_interest <= size {
            self.open_interest = 0u128;
            self.entry_price = 0u128;
        } else {
            self.open_interest = cal_utils::sub_u128(self.open_interest, size)?;
        }
        Ok(())
    }
}

#[zero_copy(unsafe)]
#[derive(Eq, PartialEq, AnchorSerialize, AnchorDeserialize, Default, Debug)]
#[repr(C)]
pub struct MarketConfig {
    pub max_leverage: u128,
    pub tick_size: u128,
    pub open_fee_rate: u128,
    pub close_fee_rate: u128,
    pub max_long_open_interest_cap: u128,
    pub max_short_open_interest_cap: u128,
    pub long_short_ratio_limit: u128,
    pub long_short_oi_bottom_limit: u128,
}