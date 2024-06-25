use anchor_lang::prelude::*;
use num_traits::ToPrimitive;

use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub struct MarketFundingFee {
    pub long_funding_fee_amount_per_size: i128,
    pub short_funding_fee_amount_per_size: i128,
    pub total_long_funding_fee: i128,
    pub total_short_funding_fee: i128,
    pub long_funding_fee_rate: i128,
    pub short_funding_fee_rate: i128,
    pub last_update: u128,
}

impl MarketFundingFee {
    pub fn update_last_update(&mut self) -> BumpResult {
        self.last_update = Clock::get().unwrap().unix_timestamp.to_u128().unwrap();
        Ok(())
    }
    pub fn update_market_funding_fee_rate(
        &mut self,
        short_funding_fee_amount_per_size_delta: i128,
        long_funding_fee_amount_per_size_delta: i128,
        fee_durations: u128,
    ) -> BumpResult<()> {
        self.short_funding_fee_amount_per_size = short_funding_fee_amount_per_size_delta
            .safe_add(self.short_funding_fee_amount_per_size.cast::<i128>()?)?
            .cast::<i128>()?;

        self.long_funding_fee_amount_per_size = long_funding_fee_amount_per_size_delta
            .safe_add(self.long_funding_fee_amount_per_size.cast()?)?
            .cast::<i128>()?;

        self.long_funding_fee_rate = long_funding_fee_amount_per_size_delta
            .safe_div(fee_durations.cast::<i128>()?)?
            .safe_mul(3600i128)?;

        self.short_funding_fee_rate = short_funding_fee_amount_per_size_delta
            .safe_div(fee_durations.cast::<i128>()?)?
            .safe_mul(3600i128)?;
        self.update_last_update()?;
        Ok(())
    }

    pub fn get_market_funding_fee_durations(&self) -> BumpResult<u128> {
        if self.last_update > 0u128 {
            let clock = Clock::get().unwrap();
            Ok(clock.unix_timestamp.to_u128().unwrap().safe_sub(self.last_update)?)
        } else {
            Ok(0u128)
        }
    }
}
