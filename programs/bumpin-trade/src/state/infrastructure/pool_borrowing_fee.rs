use anchor_lang::prelude::*;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpResult;
use crate::instructions::cal_utils;
use crate::math::safe_math::SafeMath;
use crate::state::pool::PoolBalance;

#[bumpin_zero_copy_unsafe]
pub struct BorrowingFee {
    pub total_borrowing_fee: u128,
    pub total_realized_borrowing_fee: u128,
    // 当前这一刻，我借的每一个币产生的borrowing fee
    pub cumulative_borrowing_fee_per_token: u128,
    pub last_update: i64,
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
            let time_diff = self.get_pool_borrowing_fee_durations()?;
            let total_amount = pool.amount.safe_add(pool.un_settle_amount)?;
            let utilization = pool.hold_amount.safe_div_small_rate(total_amount)?;
            self.cumulative_borrowing_fee_per_token = utilization
                .safe_mul_small_rate(borrowing_interest_rate)?
                .safe_mul(time_diff as u128)?;
        }
        self.last_update = Clock::get().unwrap().unix_timestamp;
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
        if self.last_update > 0i64 {
            let clock = Clock::get().unwrap();
            clock.unix_timestamp.safe_sub(self.last_update)
        } else {
            Ok(0i64)
        }
    }
}
