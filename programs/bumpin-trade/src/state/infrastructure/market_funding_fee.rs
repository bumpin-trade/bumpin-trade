use anchor_lang::prelude::*;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::constants::SMALL_RATE_TO_PER_TOKEN_PRECISION;
use crate::math::safe_math::SafeMath;

#[bumpin_zero_copy_unsafe]
pub struct MarketFundingFee {
    pub long_funding_fee_amount_per_size: i128,  //10^18
    pub short_funding_fee_amount_per_size: i128, //10^18
    pub total_long_funding_fee: i128,
    pub total_short_funding_fee: i128,
    pub long_funding_fee_rate: i128,
    pub short_funding_fee_rate: i128,
    pub updated_at: i64,
    pub padding: [u8; 8],
}

impl MarketFundingFee {
    pub fn update_last_update(&mut self) -> BumpResult {
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
    pub fn update_market_funding_fee_rate(
        &mut self,
        short_funding_fee_amount_per_size_delta: i128,
        long_funding_fee_amount_per_size_delta: i128,
        fee_durations: i64,
    ) -> BumpResult<()> {
        self.short_funding_fee_amount_per_size = short_funding_fee_amount_per_size_delta
            .safe_add(self.short_funding_fee_amount_per_size)?;

        self.long_funding_fee_amount_per_size = long_funding_fee_amount_per_size_delta
            .safe_add(self.long_funding_fee_amount_per_size)?;

        self.long_funding_fee_rate = long_funding_fee_amount_per_size_delta
            .safe_mul(3600i128)?
            .safe_div(fee_durations.cast::<i128>()?)?
            .safe_div(SMALL_RATE_TO_PER_TOKEN_PRECISION.cast::<i128>()?)?;

        self.short_funding_fee_rate = short_funding_fee_amount_per_size_delta
            .safe_mul(3600i128)?
            .safe_div(fee_durations.cast::<i128>()?)?
            .safe_div(SMALL_RATE_TO_PER_TOKEN_PRECISION.cast::<i128>()?)?;
        self.update_last_update()?;
        Ok(())
    }

    pub fn get_market_funding_fee_durations(&self) -> BumpResult<i64> {
        if self.updated_at > 0i64 {
            let clock = Clock::get().unwrap();
            clock.unix_timestamp.safe_sub(self.updated_at)
        } else {
            Ok(0i64)
        }
    }
}
