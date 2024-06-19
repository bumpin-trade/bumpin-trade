use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::add_u128;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::state::infrastructure::pool_borrowing_fee::BorrowingFee;
use crate::traits::Size;
use crate::validate;
use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Pool {
    pub pool_key: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_index: u16,
    pub pool_mint_vault: Pubkey,
    pub pool_name: [u8; 32],
    pub pool_balance: PoolBalance,
    pub stable_balance: PoolBalance,
    pub borrowing_fee: BorrowingFee,
    pub fee_reward: FeeReward,
    pub stable_fee_reward: FeeReward,
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
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
            decimals: 0,
            apr: 0u128,
            insurance_fund_amount: 0,
        }
    }
}

impl Pool {
    pub fn add_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.pool_balance.amount = self.pool_balance.amount.safe_add(amount)?;
        Ok(())
    }

    pub fn sub_amount(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.pool_balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.pool_balance.amount = self.pool_balance.amount.safe_sub(amount)?;
        Ok(())
    }

    pub fn add_supply(&mut self, stake_amount: u128) -> BumpResult<()> {
        self.total_supply = self.total_supply.safe_add(stake_amount)?;
        Ok(())
    }
    pub fn hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.check_hold_is_allowed(amount)?, BumpErrorCode::AmountNotEnough.into())?;
        self.pool_balance.hold_amount = add_u128(self.pool_balance.hold_amount, amount)?;
        Ok(())
    }

    pub fn un_hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.pool_balance.hold_amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.pool_balance.hold_amount = add_u128(self.pool_balance.hold_amount, amount)?;
        Ok(())
    }

    pub fn add_unsettle(&mut self, amount: u128) -> BumpResult<()> {
        self.pool_balance.un_settle_amount = add_u128(self.pool_balance.un_settle_amount, amount)?;
        Ok(())
    }

    pub fn get_current_max_un_stake(&self) -> BumpResult<u128> {
        Ok(self.pool_balance.amount.safe_sub(self.pool_balance.hold_amount)?)
    }

    pub fn add_insurance_fund(&mut self, amount: u128) -> BumpResult<()> {
        self.insurance_fund_amount = add_u128(self.insurance_fund_amount, amount)?;
        Ok(())
    }

    pub fn add_stable_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.stable_balance.amount = add_u128(self.stable_balance.amount, amount)?;
        Ok(())
    }

    pub fn add_stable_loss_amount(&mut self, amount: u128) -> BumpResult<()> {
        self.stable_balance.loss_amount = add_u128(self.stable_balance.loss_amount, amount)?;
        Ok(())
    }

    pub fn reset_stable_loss_amount(&mut self) -> BumpResult<()> {
        self.stable_balance.loss_amount = 0u128;
        Ok(())
    }

    pub fn reset_stable_amount(&mut self) -> BumpResult<()> {
        self.stable_balance.loss_amount = 0u128;
        Ok(())
    }

    pub fn settle_funding_fee(
        base_token_pool: &mut Pool,
        stable_token_pool: &mut Pool,
        fee_amount_usd: i128,
        fee_amount: i128,
        is_long: bool,
        is_cross: bool,
    ) -> BumpResult<()> {
        if !is_long {
            if fee_amount_usd <= 0i128 {
                //stable_pool should pay to user, count loss on base_token_pool
                base_token_pool.add_stable_loss_amount(fee_amount_usd.cast::<u128>()?)?;
                stable_token_pool.add_unsettle(fee_amount_usd.cast::<u128>()?)?;
            } else {
                if is_cross {
                    stable_token_pool.add_unsettle(fee_amount_usd.cast::<u128>()?)?;
                } else {
                    //user should pay to stable_pool, count amount on base_token_pool
                    base_token_pool.add_stable_amount(fee_amount_usd.cast::<u128>()?)?;
                }
            }
        } else {
            if fee_amount_usd <= 0i128 {
                //base_token_pool should pay to user, count amount on base_token_pool
                base_token_pool.sub_amount(fee_amount.cast::<u128>()?)?;
            } else {
                if is_cross {
                    //user should pay to base_token_pool, count amount on base_token_pool
                    base_token_pool.add_unsettle(fee_amount.cast::<u128>()?)?;
                } else {
                    base_token_pool.add_amount(fee_amount.cast::<u128>()?)?;
                }
            }
        }
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
