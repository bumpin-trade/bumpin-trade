use anchor_lang::prelude::*;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpErrorCode::PoolSubUnsettleNotEnough;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{add_u128, sub_u128};
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::PoolUpdateEvent;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::state::infrastructure::pool_borrowing_fee::BorrowingFee;
use crate::traits::Size;
use crate::validate;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Pool {
    pub pnl: i128,
    pub apr: u128,
    pub insurance_fund_amount: u128,
    pub total_supply: u128,
    pub pool_balance: PoolBalance, //16
    pub stable_balance: PoolBalance,
    pub borrowing_fee: BorrowingFee, //16
    pub fee_reward: FeeReward,       //16
    pub stable_fee_reward: FeeReward,
    pub pool_config: PoolConfig, //16
    pub pool_mint_vault: Pubkey,
    pub pool_key: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_index: u16,
    pub pool_status: PoolStatus,
    pub stable: bool,
    pub pool_name: [u8; 32],
    pub padding: [u8; 12],
}

impl Size for Pool {
    const SIZE: usize = std::mem::size_of::<Pool>() + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolStatus {
    NORMAL,
    StakePaused,
    UnStakePaused,
}

#[bumpin_zero_copy_unsafe]
pub struct PoolBalance {
    pub amount: u128,
    pub hold_amount: u128,
    pub un_settle_amount: u128,
    pub settle_funding_fee_amount: u128,
    pub loss_amount: u128,
    pub pool_mint: Pubkey,
}

#[bumpin_zero_copy_unsafe]
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
            pool_index: Default::default(),
            pool_key: Default::default(),
            pool_mint: Default::default(),
            pool_mint_vault: Pubkey::default(),
            pool_name: [0; 32],
            pool_balance: PoolBalance::default(),
            stable_balance: Default::default(),
            borrowing_fee: BorrowingFee::default(),
            fee_reward: FeeReward::default(),
            stable_fee_reward: Default::default(),
            pool_config: PoolConfig::default(),
            total_supply: 0u128,
            pool_status: PoolStatus::NORMAL,
            stable: false,
            pnl: 0,
            apr: 0u128,
            insurance_fund_amount: 0,
            padding: [0; 12],
        }
    }
}

impl Pool {
    pub fn add_pnl(&mut self, pool_pnl: i128) -> BumpResult<()> {
        self.pnl = self.pnl.safe_add(pool_pnl)?;
        Ok(())
    }
    pub fn add_amount(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        self.pool_balance.amount = self.pool_balance.amount.safe_add(amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_amount(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.pool_balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.pool_balance.amount = self.pool_balance.amount.safe_sub(amount)?;
        Ok(())
    }

    pub fn add_amount_and_supply(&mut self, amount: u128, supply_amount: u128) -> BumpResult<()> {
        validate!(self.pool_balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        validate!(self.total_supply > amount, BumpErrorCode::AmountNotEnough.into())?;
        let pre_pool = self.clone();
        self.pool_balance.amount = self.pool_balance.amount.safe_add(amount)?;
        self.total_supply = self.total_supply.safe_add(supply_amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_amount_and_supply(&mut self, amount: u128, supply_amount: u128) -> BumpResult<()> {
        validate!(self.pool_balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        validate!(self.total_supply > amount, BumpErrorCode::AmountNotEnough.into())?;
        let pre_pool = self.clone();
        self.pool_balance.amount = self.pool_balance.amount.safe_sub(amount)?;
        self.total_supply = self.total_supply.safe_sub(supply_amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_loss_amount(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.pool_balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.pool_balance.loss_amount = self.pool_balance.loss_amount.safe_sub(amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.check_hold_is_allowed(amount)?, BumpErrorCode::AmountNotEnough.into())?;
        self.pool_balance.hold_amount = add_u128(self.pool_balance.hold_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn un_hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.pool_balance.hold_amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.pool_balance.hold_amount = add_u128(self.pool_balance.hold_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn add_unsettle(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        self.pool_balance.un_settle_amount = add_u128(self.pool_balance.un_settle_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_unsettle(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.pool_balance.un_settle_amount >= amount, PoolSubUnsettleNotEnough)?;
        self.pool_balance.un_settle_amount = sub_u128(self.pool_balance.un_settle_amount, amount)?;
        self.pool_balance.amount = add_u128(self.pool_balance.amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn get_current_max_un_stake(&self) -> BumpResult<u128> {
        Ok(self.pool_balance.amount.safe_sub(self.pool_balance.hold_amount)?)
    }

    pub fn add_insurance_fund(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        self.insurance_fund_amount = add_u128(self.insurance_fund_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn add_stable_amount(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        self.stable_balance.amount = add_u128(self.stable_balance.amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn add_stable_loss_amount(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        self.stable_balance.loss_amount = add_u128(self.stable_balance.loss_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn update_pool_borrowing_fee_rate(&mut self) -> BumpResult {
        self.borrowing_fee.update_pool_borrowing_fee(
            &self.pool_balance,
            self.pool_config.borrowing_interest_rate,
        )?;
        Ok(())
    }

    fn check_hold_is_allowed(&self, amount: u128) -> BumpResult<bool> {
        if self.pool_config.pool_liquidity_limit == 0 {
            return Ok(add_u128(self.pool_balance.amount, self.pool_balance.un_settle_amount)?
                .safe_sub(self.pool_balance.hold_amount)?
                >= amount);
        }
        return Ok(add_u128(self.pool_balance.amount, self.pool_balance.un_settle_amount)?
            .safe_sub(self.pool_balance.hold_amount)?
            .safe_mul(self.pool_config.pool_liquidity_limit)?
            >= amount);
    }

    fn emit_pool_update_event(&self, pre_pool: &Pool) {
        emit!(PoolUpdateEvent {
            pool_key: self.pool_key,
            pool_mint: self.pool_mint,
            pool_index: self.pool_index,
            pool_balance: self.pool_balance,
            stable_balance: self.stable_balance,
            borrowing_fee: self.borrowing_fee,
            fee_reward: self.fee_reward,
            stable_fee_reward: self.stable_fee_reward,
            total_supply: self.total_supply,
            pnl: self.pnl,
            apr: self.apr,
            insurance_fund_amount: self.insurance_fund_amount,
            pre_pool_balance: pre_pool.pool_balance,
            pre_stable_balance: pre_pool.stable_balance,
            pre_borrowing_fee: pre_pool.borrowing_fee,
            pre_fee_reward: pre_pool.fee_reward,
            pre_stable_fee_reward: pre_pool.stable_fee_reward,
            pre_total_supply: pre_pool.total_supply,
            pre_pnl: pre_pool.pnl,
            pre_apr: pre_pool.apr,
            pre_insurance_fund_amount: pre_pool.insurance_fund_amount,
        });
    }
}

#[cfg(test)]
mod test {
    use crate::state::pool::Pool;

    #[test]
    pub fn size_of_pool() {
        println!("size of pool: {}", std::mem::size_of::<Pool>());
        assert_eq!(std::mem::size_of::<Pool>(), 528);
    }
}
