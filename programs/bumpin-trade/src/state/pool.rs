use anchor_lang::prelude::*;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpErrorCode::PoolSubUnsettleNotEnough;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{add_i128, add_u128, calculator, sub_u128};
use crate::math::casting::Cast;
use crate::math::constants::PRICE_PRECISION;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::PoolUpdateEvent;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::state::infrastructure::pool_borrowing_fee::BorrowingFee;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::traits::Size;
use crate::validate;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Default, Debug)]
#[repr(C)]
pub struct Pool {
    pub key: Pubkey,
    pub name: [u8; 32],
    pub pnl: i128,
    pub apr: u128,
    pub insurance_fund_amount: u128,
    pub total_supply: u128,
    pub balance: PoolBalance,
    pub stable_balance: PoolBalance,
    pub borrowing_fee: BorrowingFee,
    pub fee_reward: FeeReward,
    pub stable_fee_reward: FeeReward,
    pub config: PoolConfig,
    pub pool_vault_key: Pubkey,
    pub stable_mint_key: Pubkey,
    pub mint_key: Pubkey,
    pub index: u16,
    pub status: PoolStatus,
    pub stable: bool,
    pub market_number: u16,
    pub padding: [u8; 8],
    pub reserve_padding: [u8; 32],
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
    pub settle_funding_fee: i128,
    pub amount: u128,
    pub hold_amount: u128,
    pub un_settle_amount: u128,
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
    pub padding: [u8; 4],
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

    pub fn hold_pool_amount(
        &mut self,
        amount: u128,
        oracle_map: &mut OracleMap,
        base_trade_token: &TradeToken,
        stable_trade_token: &TradeToken,
    ) -> BumpResult<()> {
        let available_liquidity =
            self.get_pool_available_liquidity(oracle_map, base_trade_token, stable_trade_token)?;
        validate!(amount < available_liquidity, BumpErrorCode::AmountNotEnough)?;
        self.hold_pool(amount)?;
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

    pub fn add_stable_balance_unsettle(&mut self, amount: u128) -> BumpResult<()> {
        self.stable_balance.un_settle_amount =
            add_u128(self.stable_balance.un_settle_amount, amount)?;
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

    pub fn sub_stable_amount(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.stable_balance.amount >= amount, BumpErrorCode::AmountNotEnough)?;
        let pre_pool = self.clone();
        self.stable_balance.amount = sub_u128(self.stable_balance.amount, amount)?;
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
            .update_pool_borrowing_fee(&self.balance, self.config.borrowing_interest_rate)?;
        Ok(())
    }

    pub fn update_pool_funding_fee(&mut self, amount: i128, is_add: bool) -> BumpResult {
        self.balance.settle_funding_fee = if is_add {
            self.balance.settle_funding_fee.safe_add(amount)?
        } else {
            self.balance.settle_funding_fee.safe_sub(amount)?
        };
        Ok(())
    }

    pub fn get_pool_available_liquidity(
        &self,
        oracle_map: &mut OracleMap,
        base_trade_token: &TradeToken,
        stable_trade_token: &TradeToken,
    ) -> BumpResult<u128> {
        let mut base_token_amount = self
            .balance
            .amount
            .cast::<i128>()?
            .safe_add(self.balance.un_settle_amount.cast::<i128>()?)?
            .safe_sub(self.balance.hold_amount.cast::<i128>()?)?;
        if base_token_amount <= 0i128 {
            return Ok(0u128);
        }
        if Self::get_token_amount(&self.stable_balance)? < 0i128 {
            let token_usd = calculator::token_to_usd_i(
                Self::get_token_amount(&self.stable_balance)?,
                stable_trade_token.decimals,
                oracle_map
                    .get_price_data(&stable_trade_token.oracle_key)
                    .map_err(|_e| BumpErrorCode::OracleNotFound)?
                    .price,
            )?;
            let stable_to_base_token = calculator::usd_to_token_i(
                token_usd,
                base_trade_token.decimals,
                oracle_map
                    .get_price_data(&base_trade_token.oracle_key)
                    .map_err(|_e| BumpErrorCode::OracleNotFound)?
                    .price,
            )?;
            if base_token_amount > stable_to_base_token {
                base_token_amount = base_token_amount.safe_sub(stable_to_base_token)?;
            } else {
                base_token_amount = 0i128
            }
        }
        let available_token_amount = calculator::mul_rate_i(
            base_token_amount,
            self.config.pool_liquidity_limit.cast::<i128>()?,
        )?;
        if available_token_amount > self.balance.hold_amount.cast::<i128>()? {
            Ok(available_token_amount
                .safe_sub(self.balance.hold_amount.cast::<i128>()?)?
                .cast::<u128>()?)
        } else {
            Ok(0u128)
        }
    }

    fn hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = *self;
        if amount == 0u128 {
            return Ok(());
        };
        validate!(
            Self::check_hold_allowed(&self.balance, self.config.pool_liquidity_limit, amount)?,
            BumpErrorCode::AmountNotEnough
        )?;
        self.balance.hold_amount = self.balance.hold_amount.safe_add(amount)?;
        self.emit_pool_update_event(&pre_pool);
        Ok(())
    }

    fn check_hold_allowed(
        token_balance: &PoolBalance,
        pool_liquidity_limit: u128,
        amount: u128,
    ) -> BumpResult<bool> {
        return if pool_liquidity_limit == 0u128 {
            Ok(token_balance
                .amount
                .safe_add(token_balance.un_settle_amount)?
                .safe_sub(token_balance.hold_amount)?
                >= amount)
        } else {
            Ok(calculator::mul_rate_u(
                token_balance.amount.safe_add(token_balance.un_settle_amount)?,
                pool_liquidity_limit,
            )?
            .safe_sub(token_balance.hold_amount)?
                >= amount)
        };
    }

    fn get_token_amount(pool_balance: &PoolBalance) -> BumpResult<i128> {
        Ok(pool_balance
            .amount
            .safe_add(pool_balance.un_settle_amount)?
            .cast::<i128>()?
            .safe_add(pool_balance.settle_funding_fee)?
            .safe_sub(pool_balance.loss_amount.cast()?)?)
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
        market_map: &MarketMap,
    ) -> BumpResult<u128> {
        let trade_token = trade_token_map.get_trade_token_by_mint_ref(&self.mint_key)?;
        let trade_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
        let mut pool_value = calculator::token_to_usd_i(
            self.balance
                .amount
                .safe_add(self.balance.un_settle_amount)?
                .safe_add(self.balance.settle_funding_fee_amount)?
                .cast::<i128>()?,
            trade_token.decimals,
            trade_token_price,
        )?;
        if !self.stable {
            let markets = market_map.get_all_market(self.market_number)?;
            for market_loader in markets {
                let market =
                    market_loader.load().map_err(|_e| BumpErrorCode::CouldNotLoadMarketData)?;
                validate!(self.key.eq(&market.pool_key), BumpErrorCode::CouldNotFindMarket)?;
                let long_market_un_pnl = market.get_market_un_pnl(true, oracle_map)?;
                pool_value = add_i128(pool_value, long_market_un_pnl)?;

                let short_market_un_pnl = market.get_market_un_pnl(false, oracle_map)?;
                pool_value = add_i128(pool_value, short_market_un_pnl)?;
            }

            let stable_amount = self
                .stable_balance
                .amount
                .safe_add(self.stable_balance.un_settle_amount)?
                .safe_add(self.stable_balance.settle_funding_fee_amount)?
                .safe_sub(self.stable_balance.loss_amount)?;
            if stable_amount > 0u128 {
                let stable_trade_token =
                    trade_token_map.get_trade_token_by_mint_ref(&self.stable_mint_key)?;
                let stable_trade_token_price =
                    oracle_map.get_price_data(&stable_trade_token.oracle_key)?.price;
                let stable_usd_value = calculator::token_to_usd_i(
                    stable_amount.cast::<i128>()?,
                    stable_trade_token.decimals,
                    stable_trade_token_price,
                )?;
                pool_value = add_i128(pool_value, stable_usd_value)?;
            }
        }
        Ok(if pool_value <= 0i128 { 0u128 } else { pool_value.abs().cast::<u128>()? })
    }

    pub fn get_pool_net_price(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
        market_vec: &MarketMap,
    ) -> BumpResult<u128> {
        let pool_value = self.get_pool_usd_value(trade_token_map, oracle_map, market_vec)?;
        calculator::div_to_precision_u(pool_value, self.total_supply, PRICE_PRECISION)
    }

    pub fn update_pnl_and_un_hold_pool_amount(
        &mut self,
        amount: u128,
        token_pnl: i128,
        add_liability: u128,
        base_token_pool: Option<&mut Pool>,
    ) -> BumpResult {
        self.un_hold_pool(amount)?;

        Ok(match base_token_pool {
            //long
            None => {
                if token_pnl < 0i128 {
                    if self.stable {
                        Err(BumpErrorCode::InvalidParam)?
                    }
                    self.sub_amount(token_pnl.abs().cast::<u128>()?)?;
                } else if add_liability == 0u128 {
                    self.add_amount(token_pnl.cast::<u128>()?)?
                } else {
                    let u_token_pnl = token_pnl.abs().cast::<u128>()?;
                    self.add_amount(if u_token_pnl > add_liability {
                        u_token_pnl.safe_sub(add_liability)?
                    } else {
                        0u128
                    })?;
                    self.add_unsettle(if u_token_pnl > add_liability {
                        add_liability
                    } else {
                        u_token_pnl
                    })?;
                }
            },
            Some(base_token_pool) => {
                //short
                if token_pnl < 0i128 {
                    if self.stable {
                        // need count loss on base_token_pool
                        self.add_stable_balance_unsettle(token_pnl.abs().cast::<u128>()?)?;
                        base_token_pool.add_stable_loss_amount(token_pnl.abs().cast::<u128>()?)?;
                    }
                    self.sub_stable_amount(token_pnl.abs().cast::<u128>()?)?;
                } else if add_liability == 0u128 {
                    base_token_pool.add_amount(token_pnl.cast::<u128>()?)?
                } else {
                    let u_token_pnl = token_pnl.abs().cast::<u128>()?;
                    base_token_pool.add_stable_amount(if u_token_pnl > add_liability {
                        u_token_pnl.safe_sub(add_liability)?
                    } else {
                        0u128
                    })?;
                    base_token_pool.add_stable_balance_unsettle(
                        if u_token_pnl > add_liability { add_liability } else { u_token_pnl },
                    )?;
                }
            },
        })
    }
}
