use anchor_lang::prelude::*;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpResult;
use crate::instructions::cal_utils;
use crate::math::constants::SMALL_RATE_PRECISION;
use crate::math::safe_math::SafeMath;
use crate::state::pool::PoolBalance;

#[bumpin_zero_copy_unsafe]
pub struct BorrowingFee {
    pub total_borrowing_fee: u128,
    pub total_realized_borrowing_fee: u128,
    pub cumulative_borrowing_fee_per_token: u128,
    pub updated_at: i64,
    pub padding: [u8; 8],
}

impl BorrowingFee {
    pub fn update_pool_borrowing_fee(
        &mut self,
        pool_balance: &PoolBalance,
        borrowing_interest_rate: u128,
    ) -> BumpResult<()> {
        if pool_balance.amount == 0 && pool_balance.un_settle_amount == 0 {
            self.cumulative_borrowing_fee_per_token = 0;
        } else {
            let time_diff = self.get_pool_borrowing_fee_durations()?;
            let total_amount = pool_balance.amount.safe_add(pool_balance.un_settle_amount)?;
            let hold_rate = cal_utils::div_to_precision_u(
                pool_balance.hold_amount,
                total_amount,
                SMALL_RATE_PRECISION,
            )?;
            self.cumulative_borrowing_fee_per_token =
                self.cumulative_borrowing_fee_per_token.safe_add(cal_utils::mul_small_rate_u(
                    hold_rate.safe_mul(time_diff as u128)?,
                    borrowing_interest_rate,
                )?)?;
        }
        self.updated_at = Clock::get().unwrap().unix_timestamp;
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
    pub fn get_pool_borrowing_fee_durations(&self) -> BumpResult<i64> {
        if self.updated_at > 0i64 {
            let clock = Clock::get().unwrap();
            clock.unix_timestamp.safe_sub(self.updated_at)
        } else {
            Ok(0i64)
        }
    }
}
