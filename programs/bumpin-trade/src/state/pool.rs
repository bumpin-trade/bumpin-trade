use anchor_lang::prelude::*;
use std::panic::Location;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpErrorCode::PoolSubUnsettleNotEnough;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{add_i128, add_u128, calculator, sub_i128, sub_u128};
use crate::math::casting::Cast;
use crate::math::constants::PRICE_TO_USD_PRECISION;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::PoolUpdateEvent;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::state::infrastructure::pool_borrowing_fee::BorrowingFee;
use crate::state::market::Market;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
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
    pub borrowing_fee: BorrowingFee,
    pub fee_reward: FeeReward,
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
    pub settle_funding_fee: i128, // we should receive funding fee
    pub amount: u128,
    pub hold_amount: u128,
    pub un_settle_amount: u128,
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
    pub fn set_apr(&mut self, apr: u128) -> BumpResult<()> {
        self.apr = apr;
        Ok(())
    }
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
        validate!(
            self.balance.amount >= amount,
            BumpErrorCode::SubPoolAmountBiggerThanAmount.into()
        )?;
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
        validate!(
            self.balance.amount >= amount,
            BumpErrorCode::SubPoolAmountBiggerThanAmount.into()
        )?;
        validate!(
            self.total_supply >= amount,
            BumpErrorCode::SubPoolAmountBiggerThanAmount.into()
        )?;
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
        market_map: &MarketMap,
        oracle_map: &mut OracleMap,
        trade_token_map: &TradeTokenMap,
        max_pool_liquidity_share_rate: u32,
    ) -> BumpResult<()> {
        let available_liquidity = self
            .get_pool_available_liquidity(market_map, oracle_map, trade_token_map)?
            .safe_mul_rate(max_pool_liquidity_share_rate.cast::<u128>()?)?;
        validate!(amount < available_liquidity, BumpErrorCode::PoolAvailableLiquidityNotEnough)?;
        self.hold_pool(amount)?;
        Ok(())
    }

    pub fn un_hold_pool(&mut self, amount: u128) -> BumpResult<()> {
        let pre_pool = self.clone();
        validate!(
            self.balance.hold_amount >= amount,
            BumpErrorCode::SubHoldPoolBiggerThanHold.into()
        )?;
        self.balance.hold_amount = sub_u128(self.balance.hold_amount, amount)?;
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

    pub fn update_pool_borrowing_fee_rate(&mut self) -> BumpResult {
        self.borrowing_fee
            .update_pool_borrowing_fee_rate(&self.balance, self.config.borrowing_interest_rate)?;
        Ok(())
    }

    pub fn update_pool_funding_fee(&mut self, amount: i128) -> BumpResult {
        self.balance.settle_funding_fee = self.balance.settle_funding_fee.safe_add(amount)?;
        Ok(())
    }

    pub fn settle_pool_funding_fee(&mut self, amount: i128) -> BumpResult<()> {
        if amount == 0i128 {
            return Ok(());
        }
        self.balance.settle_funding_fee = self.balance.settle_funding_fee.safe_sub(amount)?;
        Ok(())
    }

    pub fn get_pool_available_liquidity(
        &self,
        market_map: &MarketMap,
        oracle_map: &mut OracleMap,
        trade_token_map: &TradeTokenMap,
    ) -> BumpResult<u128> {
        let mut base_token_amount = self
            .balance
            .amount
            .cast::<i128>()?
            .safe_add(self.balance.un_settle_amount.cast::<i128>()?)?;
        let markets = market_map.get_all_market()?;
        let mut market_loaded = vec![];
        for market_loader in markets {
            let market = market_loader.load().map_err(|e| {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!("Could not load trade_token at {}:{}", caller.file(), caller.line());
                BumpErrorCode::CouldNotLoadMarketData
            })?;
            if self.key.eq(&market.pool_key) || self.key.eq(&market.stable_pool_key) {
                market_loaded.push(market);
            }
        }
        validate!(
            self.market_number == market_loaded.len() as u16,
            BumpErrorCode::MarketNumberNotEqual2Pool
        )?;

        for market in market_loaded {
            if !market.share_short {
                continue;
            }
            let stable_trade_token =
                trade_token_map.get_trade_token_by_mint_ref(&market.stable_pool_mint_key)?;
            let stable_trade_token_price =
                oracle_map.get_price_data(&stable_trade_token.oracle_key)?.price;
            let stable_loss_value = calculator::token_to_usd_i(
                market.stable_loss.safe_add(market.stable_unsettle_loss.cast()?)?,
                stable_trade_token.decimals,
                stable_trade_token_price,
            )?;
            let trade_token = trade_token_map.get_trade_token_by_mint_ref(&self.mint_key)?;
            let trade_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;

            let stable_loss_token_amount = calculator::usd_to_token_i(
                stable_loss_value,
                trade_token.decimals,
                trade_token_price,
            )?;
            if !self.stable {
                base_token_amount = base_token_amount.safe_add(stable_loss_token_amount)?
            } else {
                base_token_amount = base_token_amount.safe_sub(stable_loss_token_amount)?
            }
        }

        if base_token_amount <= 0i128 {
            return Ok(0u128);
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
            BumpErrorCode::PoolAvailableLiquidityNotEnough
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
        if pool_liquidity_limit == 0u128 {
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
        }
    }

    fn emit_pool_update_event(&self, pre_pool: &Pool) {
        emit!(PoolUpdateEvent {
            pool_key: self.key,
            pool_mint: self.mint_key,
            pool_index: self.index,
            pool_balance: self.balance,
            borrowing_fee: self.borrowing_fee,
            fee_reward: self.fee_reward,
            total_supply: self.total_supply,
            pnl: self.pnl,
            apr: self.apr,
            insurance_fund_amount: self.insurance_fund_amount,
            pre_pool_balance: pre_pool.balance,
            pre_borrowing_fee: pre_pool.borrowing_fee,
            pre_fee_reward: pre_pool.fee_reward,
            pre_total_supply: pre_pool.total_supply,
            pre_pnl: pre_pool.pnl,
            pre_apr: pre_pool.apr,
            pre_insurance_fund_amount: pre_pool.insurance_fund_amount,
        });
    }

    #[track_caller]
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
                .cast::<i128>()?
                .safe_add(self.balance.settle_funding_fee)?,
            trade_token.decimals,
            trade_token_price,
        )?;

        let markets = market_map.get_all_market()?;
        let mut market_loaded = vec![];
        for market_loader in markets {
            let market =
                market_loader.load().map_err(|_e| BumpErrorCode::CouldNotLoadMarketData)?;
            if self.key.eq(&market.pool_key) || self.key.eq(&market.stable_pool_key) {
                market_loaded.push(market);
            }
        }
        validate!(
            self.market_number == market_loaded.len() as u16,
            BumpErrorCode::MarketNumberNotEqual2Pool
        )?;

        for market in market_loaded {
            if !self.stable {
                let long_market_un_pnl = market.get_market_un_pnl(true, oracle_map)?;
                pool_value = add_i128(pool_value, long_market_un_pnl)?;

                if market.share_short {
                    let short_market_un_pnl = market.get_market_un_pnl(false, oracle_map)?;
                    pool_value = add_i128(pool_value, short_market_un_pnl)?;

                    let stable_trade_token =
                        trade_token_map.get_trade_token_by_mint_ref(&self.stable_mint_key)?;
                    let stable_trade_token_price =
                        oracle_map.get_price_data(&stable_trade_token.oracle_key)?.price;

                    let stable_loss_value = calculator::token_to_usd_i(
                        market.stable_loss.safe_add(market.stable_unsettle_loss.cast()?)?,
                        stable_trade_token.decimals,
                        stable_trade_token_price,
                    )?;
                    pool_value = add_i128(pool_value, stable_loss_value)?;
                }
            } else {
                if market.share_short {
                    let stable_trade_token =
                        trade_token_map.get_trade_token_by_mint_ref(&self.stable_mint_key)?;
                    let stable_trade_token_price =
                        oracle_map.get_price_data(&stable_trade_token.oracle_key)?.price;

                    let stable_loss_value = calculator::token_to_usd_i(
                        market.stable_loss.safe_add(market.stable_unsettle_loss.cast()?)?,
                        stable_trade_token.decimals,
                        stable_trade_token_price,
                    )?;
                    pool_value = sub_i128(pool_value, stable_loss_value.cast()?)?;
                } else {
                    let short_market_un_pnl = market.get_market_un_pnl(false, oracle_map)?;
                    pool_value = add_i128(pool_value, short_market_un_pnl)?;
                }
            }
        }

        msg!("========get_pool_usd_value, pool_value: {}", pool_value);
        Ok(if pool_value <= 0i128 { 0u128 } else { pool_value.abs().cast::<u128>()? })
    }

    pub fn get_pool_net_price(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
        market_vec: &MarketMap,
    ) -> BumpResult<u128> {
        let pool_token = trade_token_map.get_trade_token_by_mint_ref(&self.mint_key)?;
        let pool_value = self.get_pool_usd_value(trade_token_map, oracle_map, market_vec)?;
        pool_value
            .safe_mul(10u128.pow(pool_token.decimals.cast::<u32>()?))?
            .safe_div(self.total_supply)?
            .safe_div(PRICE_TO_USD_PRECISION)
    }
    //TODO fuck logic
    pub fn update_pnl_and_un_hold_pool_amount(
        &mut self,
        market: &mut Market,
        hold_amount: u128,
        token_pnl: i128,
        user_liability: u128,
    ) -> BumpResult {
        self.un_hold_pool(hold_amount)?;
        if token_pnl < 0i128 {
            self.sub_amount(token_pnl.abs().cast::<u128>()?)?;
            if self.stable && market.share_short {
                msg!("======update_pnl_and_un_hold_pool_amount+token_pnl:{}", token_pnl);
                market.add_stable_loss(token_pnl)?;
            }
        } else if user_liability == 0u128 {
            self.add_amount(token_pnl.cast::<u128>()?)?;
            if self.stable && market.share_short {
                market.add_stable_loss(token_pnl)?;
            }
        } else {
            let u_token_pnl = token_pnl.abs().cast::<u128>()?;
            self.add_amount(if u_token_pnl > user_liability {
                u_token_pnl.safe_sub(user_liability)?
            } else {
                0u128
            })?;
            if self.stable && market.share_short && u_token_pnl > user_liability {
                market.add_stable_loss(u_token_pnl.safe_sub(user_liability)?.cast()?)?;
            }
            self.add_unsettle(if u_token_pnl > user_liability {
                user_liability
            } else {
                u_token_pnl
            })?;
            if self.stable && market.share_short {
                market.add_unsettle_stable_loss(if u_token_pnl > user_liability {
                    user_liability
                } else {
                    u_token_pnl
                })?;
            }
        }
        Ok(())
    }
}
