use anchor_lang::prelude::*;
use num_traits::ToPrimitive;

use crate::errors::BumpResult;
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::pool::PoolBalance;

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
#[repr(C)]
pub struct BorrowingFee {
    pub total_borrowing_fee: u128,
    pub total_realized_borrowing_fee: u128,
    pub cumulative_borrowing_fee_per_token: u128,
    pub last_update: u128,
}

impl BorrowingFee {
    pub fn update_pool_borrowing_fee(
        &mut self,
        pool: &PoolBalance,
        borrowing_interest_rate: u128,
    ) -> BumpResult<()> {
        if pool.amount == 0 && pool.un_settle_amount == 0 {
            self.cumulative_borrowing_fee_per_token = 0;
        } else {
            let total_amount = pool.amount.safe_add(pool.un_settle_amount.cast()?)?;
            let utilization = pool.hold_amount.safe_div(total_amount)?;
            self.cumulative_borrowing_fee_per_token =
                utilization.safe_mul(borrowing_interest_rate.cast()?)?.cast::<u128>()?;
        }
        self.last_update = Clock::get().unwrap().unix_timestamp.to_u128().unwrap();
        Ok(())
    }

    pub fn update_total_borrowing_fee(
        &mut self,
        borrowing_fee: u128,
        is_borrowing_fee_add: bool,
        realized_borrowing_fee: u128,
        is_realized_borrowing_fee_add: bool,
    ) -> BumpResult<()> {
        if is_borrowing_fee_add {
            self.total_borrowing_fee =
                cal_utils::add_u128(self.total_borrowing_fee, borrowing_fee)?;
        } else {
            self.total_borrowing_fee =
                cal_utils::sub_u128(self.total_borrowing_fee, borrowing_fee)?;
        }

        if is_realized_borrowing_fee_add {
            self.total_realized_borrowing_fee =
                cal_utils::add_u128(self.total_realized_borrowing_fee, realized_borrowing_fee)?;
        } else {
            self.total_realized_borrowing_fee =
                cal_utils::sub_u128(self.total_realized_borrowing_fee, realized_borrowing_fee)?;
        }

        Ok(())
    }
    pub fn get_pool_borrowing_fee_durations(&self) -> BumpResult<u128> {
        if self.last_update > 0u128 {
            let clock = Clock::get().unwrap();
            Ok(clock.unix_timestamp.to_u128().unwrap().safe_sub(self.last_update)?)
        } else {
            Ok(0u128)
        }
    }
}
