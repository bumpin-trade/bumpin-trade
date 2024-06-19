use anchor_lang::prelude::AccountLoader;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::optional_accounts::AccountMaps;
use crate::processor::user_processor::UserProcessor;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::validate;

pub struct PoolProcessor<'a> {
    pub(crate) pool: &'a mut Pool,
}

impl<'a> PoolProcessor<'_> {
    pub fn collect_stake_fee(&mut self, amount: u128) -> BumpResult<u128> {
        Ok(fee_processor::collect_stake_fee(&mut self.pool, amount)?)
    }
    pub fn collect_un_stake_fee(&mut self, amount: u128) -> BumpResult<u128> {
        Ok(fee_processor::collect_un_stake_fee(&mut self.pool, amount)?)
    }
    pub fn portfolio_to_stake(
        &mut self,
        user_loader: &AccountLoader<User>,
        pool_loader: &AccountLoader<Pool>,
        mint_amount: u128,
        trade_token: &TradeToken,
        account_maps: &mut AccountMaps,
    ) -> BumpResult<u128> {
        let mut stake_amount = mint_amount;
        let user = &mut user_loader.load_mut().unwrap();
        let pool = pool_loader.load().unwrap();

        let user_token = user
            .get_user_token_ref(&pool.pool_mint)?
            .ok_or(BumpErrorCode::CouldNotFindUserToken)?;
        validate!(user_token.amount > mint_amount, BumpErrorCode::AmountNotEnough)?;

        let mut user_processor = UserProcessor { user };
        user_processor.user.sub_user_token_amount(&pool.pool_mint, mint_amount)?;
        validate!(
            user_processor
                .get_available_value(&mut account_maps.oracle_map, &account_maps.trade_token_map)?
                > 0,
            BumpErrorCode::AmountNotEnough
        )?;
        if self.pool.total_supply > 0 {
            let oracle_price_data = account_maps.oracle_map.get_price_data(&self.pool.pool_mint)?;

            stake_amount = cal_utils::token_to_usd_u(
                mint_amount,
                trade_token.decimals,
                oracle_price_data.price,
            )?
            .safe_div(
                self.get_pool_net_price(&mut account_maps.oracle_map, &account_maps.market_map)?,
            )?;
        }
        self.pool.add_supply(stake_amount)?;
        self.pool.add_amount(mint_amount)?;
        let user_stake =
            user.get_user_stake_mut(&pool.pool_key)?.ok_or(BumpErrorCode::StakePaused)?;
        user_stake.add_user_stake(stake_amount)?;
        Ok(stake_amount)
    }
    pub fn stake(
        &mut self,
        user_loader: &AccountLoader<User>,
        pool_loader: &AccountLoader<Pool>,
        mint_amount: u128,
        trade_token: &TradeToken,
        account_maps: &mut AccountMaps,
    ) -> BumpResult<u128> {
        let mut stake_amount = mint_amount;
        let mut user = user_loader.load_mut().unwrap();
        let pool = pool_loader.load_mut().unwrap();
        if self.pool.total_supply > 0 {
            let oracle_price_data = account_maps.oracle_map.get_price_data(&self.pool.pool_mint)?;

            stake_amount = cal_utils::token_to_usd_u(
                mint_amount,
                trade_token.decimals,
                oracle_price_data.price,
            )?
            .safe_div(
                self.get_pool_net_price(&mut account_maps.oracle_map, &account_maps.market_map)?,
            )?;
        }
        self.pool.add_supply(stake_amount)?;
        self.pool.add_amount(mint_amount)?;
        let user_stake =
            user.get_user_stake_mut(&pool.pool_key)?.ok_or(BumpErrorCode::StakePaused)?;
        user_stake.add_user_stake(stake_amount)?;
        Ok(stake_amount)
    }
    pub fn un_stake(
        &self,
        pool_loader: &AccountLoader<Pool>,
        user_loader: &AccountLoader<User>,
        un_stake_amount: u128,
        oracle_map: &mut OracleMap,
        market_map: &MarketMap,
    ) -> BumpResult<u128> {
        let pool = pool_loader.load().unwrap();
        let mut user = user_loader.load_mut().unwrap();
        let pool_value = self.get_pool_usd_value(oracle_map, market_map)?;
        let un_stake_usd =
            cal_utils::mul_div_u(un_stake_amount, pool_value, self.pool.total_supply)?;
        let pool_price = oracle_map.get_price_data(&self.pool.pool_mint)?;
        let token_amount = cal_utils::div_u128(un_stake_usd, pool_price.price)?;
        validate!(
            token_amount > pool.pool_config.mini_un_stake_amount,
            BumpErrorCode::UnStakeNotEnough
        )?;

        let max_un_stake_amount = pool.get_current_max_un_stake()?;
        validate!(token_amount < max_un_stake_amount, BumpErrorCode::UnStakeNotEnough)?;

        let user_stake =
            user.get_user_stake_mut(&pool.pool_key)?.ok_or(BumpErrorCode::StakePaused)?;
        user_stake.sub_user_stake(un_stake_amount)?;

        Ok(token_amount)
    }
    pub fn get_pool_net_price(
        &self,
        oracle_map: &mut OracleMap,
        market_vec: &MarketMap,
    ) -> BumpResult<u128> {
        let pool_value = self.get_pool_usd_value(oracle_map, market_vec)?;
        let net_price = self.pool.total_supply.safe_div(pool_value.cast()?)?;
        Ok(net_price)
    }
    pub fn get_pool_usd_value(
        &self,
        oracle_map: &mut OracleMap,
        market_vec: &MarketMap,
    ) -> BumpResult<u128> {
        let oracle_price_data = oracle_map.get_price_data(&self.pool.pool_mint)?;
        let mut pool_value = self
            .pool
            .pool_balance
            .amount
            .safe_add(self.pool.pool_balance.un_settle_amount)?
            .safe_mul(oracle_price_data.price)?;
        if !self.pool.stable {
            let markets = market_vec.get_all_market()?;
            for mut market in markets {
                if self.pool.pool_key.eq(&market.1.pool_key) {
                    let mut market_processor = MarketProcessor { market: &mut market.1 };
                    let long_market_un_pnl =
                        market_processor.get_market_un_pnl(true, oracle_map)?;
                    pool_value = cal_utils::add_u128(pool_value, long_market_un_pnl)?;

                    let short_market_un_pnl =
                        market_processor.get_market_un_pnl(false, oracle_map)?;
                    pool_value = cal_utils::add_u128(pool_value, short_market_un_pnl)?;
                }
            }

            let stable_amount = self
                .pool
                .stable_balance
                .amount
                .safe_add(self.pool.stable_balance.un_settle_amount)?
                .safe_sub(self.pool.stable_balance.loss_amount)?;

            let stable_price = oracle_map.get_price_data(&self.pool.stable_balance.pool_mint)?;
            let stable_usd_value = stable_amount.safe_mul(stable_price.price)?;
            pool_value = cal_utils::add_u128(pool_value, stable_usd_value)?;
        }
        Ok(if pool_value <= 0 { 0u128 } else { pool_value })
    }
    pub fn update_pool_borrowing_fee_rate(&mut self) -> BumpResult {
        self.pool.borrowing_fee.update_pool_borrowing_fee(
            &self.pool.pool_balance,
            self.pool.pool_config.borrowing_interest_rate,
        )?;
        Ok(())
    }

    pub fn update_pnl_and_un_hold_pool_amount(
        &mut self,
        amount: u128,
        token_pnl: i128,
        add_liability: u128,
        base_token_pool: Option<&mut Pool>,
    ) -> BumpResult {
        self.pool.un_hold_pool(amount)?;
        if token_pnl < 0i128 {
            self.pool.sub_amount(token_pnl.abs().cast::<u128>()?)?;
            if self.pool.stable && base_token_pool.is_some() {
                // need count loss on base_token_pool
                self.pool.add_unsettle(token_pnl.abs().cast::<u128>()?)?;
                base_token_pool.unwrap().add_stable_loss_amount(token_pnl.abs().cast::<u128>()?)?;
            }
        } else if add_liability == 0u128 {
            self.pool.add_amount(token_pnl.cast::<u128>()?)?
        } else {
            let u_token_pnl = token_pnl.abs().cast::<u128>()?;
            self.pool.add_amount(if u_token_pnl > add_liability {
                u_token_pnl.safe_sub(add_liability)?
            } else {
                0u128
            })?;
            self.pool.add_unsettle(if u_token_pnl > add_liability {
                add_liability
            } else {
                u_token_pnl
            })?;
        }
        Ok(())
    }

    pub fn add_insurance_fund(&mut self, amount: u128) -> BumpResult<()> {
        self.pool.add_insurance_fund(amount)?;
        Ok(())
    }
}
