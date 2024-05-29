use anchor_lang::prelude::*;
use num_traits::ToPrimitive;
use crate::check;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{add_u128, cal_utils};
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::state::infrastructure::pool_borrowing_fee::BorrowingFee;
use crate::traits::Size;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Pool {
    pub pool_key: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_mint_vault: Pubkey,
    pub pool_rewards_vault: Pubkey,
    pub pool_fee_vault: Pubkey,
    pub pool_name: [u8; 32],
    pub pool_balance: PoolBalance,
    pub stable_balance: PoolBalance,
    pub borrowing_fee: BorrowingFee,
    pub fee_reward: FeeReward,
    pub pool_config: PoolConfig,
    pub total_supply: u128,
    pub pool_status: PoolStatus,
    pub stable: bool,
    pub decimals: u8,
    pub apr: u128,
    pub insurance_fund_amount: u128,
}

impl Size for Pool {
    const SIZE: usize = std::mem::size_of::<Pool>() + 8;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolStatus {
    NORMAL,
    StakePaused,
    UnStakePaused,
}

#[zero_copy(unsafe)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct PoolBalance {
    pub pool_mint: Pubkey,
    pub amount: u128,
    pub hold_amount: u128,
    pub un_settle_amount: u128,
    pub loss_amount: u128,
}


#[zero_copy(unsafe)]
#[derive(Eq, PartialEq, Debug, Default)]
#[repr(C)]
pub struct PoolConfig {
    pub mini_stake_amount: u128,
    pub mini_un_stake_amount: u128,
    pub pool_liquidity_limit: u128,
    pub stake_fee_rate: u128,
    pub un_stake_fee_rate: u128,
    pub un_settle_mint_ratio_limit: u128,
    pub borrowing_interest_rate: u128,
}


impl Default for Pool {
    fn default() -> Self {
        Pool {
            pool_key: Default::default(),
            pool_mint: Default::default(),
            pool_mint_vault: Pubkey::default(),
            pool_rewards_vault: Default::default(),
            pool_fee_vault: Default::default(),
            pool_name: [0; 32],
            pool_balance: PoolBalance::default(),
            stable_balance: Default::default(),
            borrowing_fee: BorrowingFee::default(),
            fee_reward: FeeReward::default(),
            pool_config: PoolConfig::default(),
            total_supply: 0u128,
            pool_status: PoolStatus::NORMAL,
            stable: false,
            apr: 0u128,
            insurance_fund_amount: 0,
        }
    }
}


impl Pool {
    pub fn add_amount(&mut self, amount: u128) {
        self.pool_balance.amount = self.pool_balance.amount.safe_add(amount)?;
    }

    pub fn sub_amount(&mut self, amount: u128) {
        check!(self.pool_balance.amount >= amount, BumpErrorCode::AmountNotEnough);
        self.pool_balance.amount = self.pool_balance.amount.safe_sub(amount)?;
    }

    pub fn add_supply(&mut self, stake_amount: u128) {
        self.total_supply = self.total_supply.safe_add(stake_amount)?;
    }
    pub fn hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        check!(self.check_hold_is_allowed(amount), BumpErrorCode::AmountNotEnough);
        Ok(self.pool_balance.hold_amount = add_u128(self.pool_balance.hold_amount, amount)?)
    }

    pub fn un_hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        check!(self.pool_balance.hold_amount>=amount, BumpErrorCode::AmountNotEnough);
        Ok(self.pool_balance.hold_amount = add_u128(self.pool_balance.hold_amount, amount)?)
    }

    pub fn add_unsettle(&mut self, amount: i128) -> BumpResult<()> {
        if amount < 0 {
            Ok(self.pool_balance.un_settle_amount = cal_utils::sub_u128(self.pool_balance.un_settle_amount?, amount.abs().to_u128()?)?)
        } else {
            Ok(self.pool_balance.un_settle_amount = add_u128(self.pool_balance.un_settle_amount?, amount.abs().to_u128()?)?)
        }
    }

    pub fn get_current_max_un_stake(&self) -> BumpResult<u128> {
        if self.pool_config.pool_liquidity_limit == 0 {
            let max_un_stake = cal_utils::sub_u128(self.pool_balance.amount, self.pool_balance.hold_amount);
            max_un_stake
        } else {
            let min_amount = self.pool_balance.hold_amount.safe_div(self.pool_config.pool_liquidity_limit)?;
            Ok(if self.pool_balance.amount > min_amount { self.pool_balance.amount.safe_sub(min_amount)? } else { 0u128 })
        }
    }

    pub fn add_insurance_fund(&mut self, amount: u128) -> BumpResult<()> {
        self.insurance_fund_amount = add_u128(self.insurance_fund_amount, amount)?;
        Ok(())
    }

    fn check_hold_is_allowed(&self, amount: u128) -> BumpResult<bool> {
        if self.pool_config.pool_liquidity_limit == 0 {
            return Ok(add_u128(self.pool_balance.amount, self.pool_balance.un_settle_amount)?.safe_sub(self.pool_balance.hold_amount)? >= amount);
        }
        return Ok(add_u128(self.pool_balance.amount, self.pool_balance.un_settle_amount)?.safe_sub(self.pool_balance.hold_amount)?
            .safe_mul(self.pool_config.pool_liquidity_limit)? >= amount);
    }
}