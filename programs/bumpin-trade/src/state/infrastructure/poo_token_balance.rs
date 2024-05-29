use anchor_lang::zero_copy;
use solana_program::pubkey::Pubkey;
use crate::{validate};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::safe_math::SafeMath;
use crate::state::pool::PoolConfig;
use solana_program::msg;

#[zero_copy(unsafe)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct PoolTokenBalance {
    pub mint_key: Pubkey,
    pub amount: u128,
    pub liability: u128,
    pub hold_amount: u128,
    pub un_settle_amount: u128,
    pub loss_amount: u128,
}

impl PoolTokenBalance {
    pub fn hold_pool(&mut self, pool_config: PoolConfig, amount: u128) -> BumpResult<()> {
        validate!(self.check_hold_is_allowed(amount,pool_config), BumpErrorCode::AmountNotEnough.into());
        self.hold_amount = cal_utils::add_u128(self.hold_amount, amount)?;
        Ok(())
    }

    fn check_hold_is_allowed(&self, amount: u128, pool_config: PoolConfig) -> BumpResult<bool> {
        if pool_config.pool_liquidity_limit == 0 {
            return Ok(cal_utils::add_u128(self.amount, self.un_settle_amount)?.safe_sub(self.hold_amount)? >= amount);
        }
        return Ok(cal_utils::add_u128(self.amount, self.un_settle_amount)?.safe_mul(pool_config.pool_liquidity_limit)? >= amount);
    }
}