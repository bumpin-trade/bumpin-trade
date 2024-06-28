use anchor_lang::prelude::AccountLoader;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::user_stake::UserStake;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::validate;

pub struct PoolProcessor<'a> {
    pub(crate) pool: &'a mut Pool,
}

impl<'a> PoolProcessor<'_> {
    pub fn portfolio_to_stake(
        &mut self,
        user_loader: &AccountLoader<User>,
        pool_loader: &AccountLoader<Pool>,
        mint_amount: u128,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
        market_map: &MarketMap,
    ) -> BumpResult<(u128, UserStake)> {
        let mut supply_amount = mint_amount;
        let user = &mut user_loader.load_mut().unwrap();
        let pool = pool_loader.load().unwrap();
        let trade_token = trade_token_map.get_trade_token(&pool.pool_mint)?;

        let user_token = user.get_user_token_ref(&pool.pool_mint)?;
        validate!(user_token.amount > mint_amount, BumpErrorCode::AmountNotEnough)?;

        user.sub_user_token_amount(&pool.pool_mint, mint_amount)?;
        validate!(
            user.get_available_value(oracle_map, trade_token_map)? > 0,
            BumpErrorCode::AmountNotEnough
        )?;
        if self.pool.total_supply > 0 {
            let oracle_price_data = oracle_map.get_price_data(&trade_token.oracle)?;

            supply_amount = cal_utils::token_to_usd_u(
                mint_amount,
                trade_token.decimals,
                oracle_price_data.price,
            )?
            .safe_div(self.pool.get_pool_net_price(
                trade_token_map,
                oracle_map,
                market_map,
            )?)?;
        }
        let user_stake = user.get_user_stake_mut_ref(&pool.pool_key)?;
        user_stake.add_user_stake(supply_amount)?;
        Ok((supply_amount, user_stake.clone()))
    }
    pub fn stake(
        &mut self,
        user_loader: &AccountLoader<User>,
        mint_amount: u128,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
        market_map: &MarketMap,
    ) -> BumpResult<(u128, UserStake)> {
        let mut supply_amount = mint_amount;
        let mut user = user_loader.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadUserData)?;
        let trade_token = trade_token_map.get_trade_token(&self.pool.pool_mint)?;
        if self.pool.total_supply > 0 {
            let oracle_price_data = oracle_map.get_price_data(&trade_token.oracle)?;

            supply_amount = cal_utils::token_to_usd_u(
                mint_amount,
                trade_token.decimals,
                oracle_price_data.price,
            )?
            .safe_div(self.pool.get_pool_net_price(
                trade_token_map,
                oracle_map,
                market_map,
            )?)?;
        }
        let user_stake = user.get_user_stake_mut_ref(&self.pool.pool_key)?;
        user_stake.add_user_stake(supply_amount)?;
        Ok((supply_amount, user_stake.clone()))
    }
    pub fn un_stake(
        &self,
        user_loader: &AccountLoader<User>,
        un_stake_amount: u128,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
        market_map: &MarketMap,
    ) -> BumpResult<u128> {
        let mut user = user_loader.load_mut().unwrap();

        let trade_token = trade_token_map.get_trade_token(&self.pool.pool_mint)?;
        let pool_value = self.pool.get_pool_usd_value(trade_token_map, oracle_map, market_map)?;

        let un_stake_usd =
            cal_utils::mul_div_u(un_stake_amount, pool_value, self.pool.total_supply)?;
        let pool_price = oracle_map.get_price_data(&trade_token.oracle)?;
        let token_amount = cal_utils::div_u128(un_stake_usd, pool_price.price)?;
        validate!(
            token_amount > self.pool.pool_config.mini_un_stake_amount,
            BumpErrorCode::UnStakeTooSmall
        )?;

        let max_un_stake_amount = self.pool.get_current_max_un_stake()?;
        validate!(token_amount < max_un_stake_amount, BumpErrorCode::UnStakeTooLarge)?;

        let user_stake = user.get_user_stake_mut_ref(&self.pool.pool_key)?;
        user_stake.sub_user_stake(un_stake_amount)?;

        Ok(token_amount)
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
