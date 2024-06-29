use anchor_lang::prelude::*;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpErrorCode::PoolSubUnsettleNotEnough;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{add_u128, cal_utils, sub_u128};
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::state::bump_events::PoolUpdateEvent;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::state::infrastructure::pool_borrowing_fee::BorrowingFee;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::trade_token_map::TradeTokenMap;
use crate::traits::Size;
use crate::validate;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Default, Debug)]
#[repr(C)]
pub struct Pool {
    pub pnl: i128,
    pub apr: u128,
    pub insurance_fund_amount: u128,
    pub total_supply: u128,
    pub balance: PoolBalance,
    pub stable_balance: PoolBalance,
    pub borrowing_fee: BorrowingFee,
    pub fee_reward: FeeReward,
    pub stable_fee_reward: FeeReward,
    pub pool_config: PoolConfig,
    pub pool_mint_vault: Pubkey,
    pub key: Pubkey,
    pub stable_key: Pubkey,
    pub mint_key: Pubkey,
    pub index: u16,
    pub status: PoolStatus,
    pub settle_funding_fee: i128,
    pub stable: bool,
    pub name: [u8; 32],
    pub padding: [u8; 12],
}

impl Size for Pool {
    const SIZE: usize = std::mem::size_of::<Pool>() + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum PoolStatus {
    #[default]
    NORMAL,
    StakePaused,
    UnStakePaused,
}

#[bumpin_zero_copy_unsafe]
pub struct PoolBalance {
    pub amount: u128,
    pub hold_amount: u128,
    pub un_settle_amount: u128,
    // 剩下还未跟用户结算的金额，已经计算了，但是还未结算
    pub settle_funding_fee_amount: u128,
    pub loss_amount: u128,
}

#[bumpin_zero_copy_unsafe]
pub struct PoolConfig {
    pub minimum_stake_amount: u128,
    pub minimum_un_stake_amount: u128,
    pub pool_liquidity_limit: u128,
    pub borrowing_interest_rate: u128, //10^18
    pub stake_fee_rate: u32,
    pub un_stake_fee_rate: u32,
    pub un_settle_mint_ratio_limit: u32,
}

impl Pool {
    pub fn add_pnl(&mut self, pool_pnl: i128) -> BumpResult<()> {
        self.pnl = self.pnl.safe_add(pool_pnl)?;
        Ok(())
    }
    pub fn add_amount(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        self.balance.amount = self.balance.amount.safe_add(amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_amount(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.balance.amount = self.balance.amount.safe_sub(amount)?;
        Ok(())
    }

    pub fn add_amount_and_supply(&mut self, amount: u128, supply_amount: u128) -> BumpResult<()> {
        validate!(self.balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        validate!(self.total_supply > amount, BumpErrorCode::AmountNotEnough.into())?;
        let pre_pool = self.clone();
        self.balance.amount = self.balance.amount.safe_add(amount)?;
        self.total_supply = self.total_supply.safe_add(supply_amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_amount_and_supply(&mut self, amount: u128, supply_amount: u128) -> BumpResult<()> {
        validate!(self.balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        validate!(self.total_supply > amount, BumpErrorCode::AmountNotEnough.into())?;
        let pre_pool = self.clone();
        self.balance.amount = self.balance.amount.safe_sub(amount)?;
        self.total_supply = self.total_supply.safe_sub(supply_amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_loss_amount(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.balance.amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.balance.loss_amount = self.balance.loss_amount.safe_sub(amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.check_hold_is_allowed(amount)?, BumpErrorCode::AmountNotEnough.into())?;
        self.balance.hold_amount = add_u128(self.balance.hold_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn un_hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.balance.hold_amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        self.balance.hold_amount = add_u128(self.balance.hold_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn add_unsettle(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        self.balance.un_settle_amount = add_u128(self.balance.un_settle_amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn sub_unsettle(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(self.balance.un_settle_amount >= amount, PoolSubUnsettleNotEnough)?;
        self.balance.un_settle_amount = sub_u128(self.balance.un_settle_amount, amount)?;
        self.balance.amount = add_u128(self.balance.amount, amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    pub fn get_current_max_un_stake(&self) -> BumpResult<u128> {
        Ok(self.balance.amount.safe_sub(self.balance.hold_amount)?)
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
        self.borrowing_fee
            .update_pool_borrowing_fee(&self.balance, self.pool_config.borrowing_interest_rate)?;
        Ok(())
    }

    pub fn update_pool_funding_fee(&mut self, amount: i128, is_add: bool) -> BumpResult {
        self.settle_funding_fee = if is_add {
            self.settle_funding_fee.safe_add(amount)?
        } else {
            self.settle_funding_fee.safe_sub(amount)?
        };
        Ok(())
    }

    fn check_hold_is_allowed(&self, amount: u128) -> BumpResult<bool> {
        if self.pool_config.pool_liquidity_limit == 0 {
            return Ok(add_u128(self.balance.amount, self.balance.un_settle_amount)?
                .safe_sub(self.balance.hold_amount)?
                >= amount);
        }
        return Ok(add_u128(self.balance.amount, self.balance.un_settle_amount)?
            .safe_sub(self.balance.hold_amount)?
            .safe_mul(self.pool_config.pool_liquidity_limit)?
            >= amount);
    }

    fn emit_pool_update_event(&self, pre_pool: &Pool) {
        emit!(PoolUpdateEvent {
            pool_key: self.key,
            pool_mint: self.mint_key,
            pool_index: self.index,
            pool_balance: self.balance,
            stable_balance: self.stable_balance,
            borrowing_fee: self.borrowing_fee,
            fee_reward: self.fee_reward,
            stable_fee_reward: self.stable_fee_reward,
            total_supply: self.total_supply,
            pnl: self.pnl,
            apr: self.apr,
            insurance_fund_amount: self.insurance_fund_amount,
            pre_pool_balance: pre_pool.balance,
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

    pub fn get_pool_usd_value(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
        market_vec: &MarketMap,
    ) -> BumpResult<u128> {
        let trade_token = trade_token_map.get_trade_token(&self.mint_key)?;
        let trade_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
        let mut pool_value = cal_utils::token_to_usd_u(
            self.balance.amount.safe_add(self.balance.un_settle_amount)?,
            trade_token.decimals,
            trade_token_price,
        )?;
        if !self.stable {
            let markets = market_vec.get_all_market()?;
            for mut market in markets {
                if self.key.eq(&market.pool_key) {
                    let mut market_processor = MarketProcessor { market: &mut market };
                    let long_market_un_pnl =
                        market_processor.get_market_un_pnl(true, oracle_map)?;
                    pool_value = add_u128(pool_value, long_market_un_pnl)?;

                    let short_market_un_pnl =
                        market_processor.get_market_un_pnl(false, oracle_map)?;
                    pool_value = add_u128(pool_value, short_market_un_pnl)?;
                }
            }

            let stable_amount = self
                .stable_balance
                .amount
                .safe_add(self.stable_balance.un_settle_amount)?
                .safe_sub(self.stable_balance.loss_amount)?;
            if stable_amount > 0u128 {
                let stable_trade_token = trade_token_map.get_trade_token(&self.stable_key)?;
                let stable_trade_token_price =
                    oracle_map.get_price_data(&stable_trade_token.oracle_key)?.price;
                let stable_usd_value = cal_utils::token_to_usd_u(
                    stable_amount,
                    stable_trade_token.decimals,
                    stable_trade_token_price,
                )?;
                pool_value = add_u128(pool_value, stable_usd_value)?;
            }
        }
        Ok(if pool_value <= 0 { 0u128 } else { pool_value })
    }

    pub fn get_pool_net_price(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
        market_vec: &MarketMap,
    ) -> BumpResult<u128> {
        let pool_value = self.get_pool_usd_value(trade_token_map, oracle_map, market_vec)?;
        let net_price = self.total_supply.safe_div(pool_value)?;
        Ok(net_price)
    }
}
