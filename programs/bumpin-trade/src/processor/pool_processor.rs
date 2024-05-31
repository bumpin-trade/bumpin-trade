use crate::errors::BumpResult;
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::processor::market_processor::MarketProcessor;
use crate::state::market_map::MarketMap;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;

pub struct PoolProcessor<'a> {
    pub(crate) pool: &'a mut Pool,
}

impl<'a> PoolProcessor<'_> {
    pub fn collect_stake_fee(&mut self, state: &mut State, amount: u128) -> BumpResult<u128> {
        Ok(fee_processor::collect_stake_fee(&mut self.pool, state, amount)?)
    }
    pub fn collect_un_stake_fee(&mut self, state: &mut State, amount: u128) -> BumpResult<u128> {
        Ok(fee_processor::collect_un_stake_fee(&mut self.pool, state, amount)?)
    }
    pub fn stake(&mut self, mint_amount: u128, oracle_map: &mut OracleMap, market_vec: &MarketMap, trade_token: &TradeToken) -> BumpResult<u128> {
        let mut stake_amount = mint_amount;
        if self.pool.total_supply > 0 {
            let oracle_price_data = oracle_map.get_price_data(&self.pool.pool_mint)?;

            stake_amount = cal_utils::token_to_usd_u(mint_amount, trade_token.decimals, oracle_price_data.price)?.
                safe_div(self.get_pool_net_price(oracle_map, market_vec)?)?;
        }
        self.pool.add_supply(stake_amount)?;
        self.pool.add_amount(mint_amount)?;
        Ok(stake_amount)
    }
    pub fn un_stake(&mut self, un_stake_amount: u128, oracle_map: &mut OracleMap, pool_value: u128) -> BumpResult<u128> {
        let un_stake_usd = cal_utils::mul_div_u(un_stake_amount, pool_value, self.pool.total_supply)?;
        let pool_price = oracle_map.get_price_data(&self.pool.pool_mint)?;
        let token_amount = cal_utils::div_u128(un_stake_usd, pool_price.price)?;
        Ok(token_amount)
    }
    pub fn get_pool_net_price(&mut self, oracle_map: &mut OracleMap, market_vec: &MarketMap) -> BumpResult<u128> {
        let pool_value = self.get_pool_usd_value(oracle_map, market_vec)?;
        let net_price = self.pool.total_supply
            .safe_div(pool_value.cast()?)?;
        Ok(net_price)
    }
    pub fn get_pool_usd_value(&mut self, oracle_map: &mut OracleMap, market_vec: &MarketMap) -> BumpResult<u128> {
        let oracle_price_data = oracle_map.get_price_data(&self.pool.pool_mint)?;
        let mut pool_value = self.pool.pool_balance.amount.
            safe_add(self.pool.pool_balance.un_settle_amount)?
            .safe_mul(oracle_price_data.price)?;
        if !self.pool.stable {
            let markets = market_vec.get_all_market()?;
            for mut market in markets {
                if self.pool.pool_key.eq(&market.1.pool_key) {
                    let mut market_processor = MarketProcessor { market: &mut market.1 };
                    let long_market_un_pnl = market_processor.get_market_un_pnl(true, oracle_map)?;
                    pool_value = cal_utils::add_u128(pool_value, long_market_un_pnl)?;

                    let short_market_un_pnl = market_processor.get_market_un_pnl(false, oracle_map)?;
                    pool_value = cal_utils::add_u128(pool_value, short_market_un_pnl)?;
                }
            }

            let stable_amount = self.pool.stable_balance.amount.
                safe_add(self.pool.stable_balance.un_settle_amount)?.
                safe_sub(self.pool.stable_balance.loss_amount)?;

            let stable_price = oracle_map.get_price_data(&self.pool.stable_balance.pool_mint)?;
            let stable_usd_value = stable_amount.safe_mul(stable_price.price)?;
            pool_value = cal_utils::add_u128(pool_value, stable_usd_value)?;
        }
        Ok(if pool_value <= 0 { 0u128 } else { pool_value })
    }
    pub fn update_pool_borrowing_fee_rate(&mut self) -> BumpResult {
        self.pool.borrowing_fee.update_pool_borrowing_fee(&self.pool.pool_balance,
                                                          self.pool.pool_config.borrowing_interest_rate)?;
        Ok(())
    }

    pub fn update_pnl_and_un_hold_pool_amount(&mut self, amount: u128, token_pnl: i128, add_liability: u128) -> BumpResult {
        self.pool.un_hold_pool(amount)?;
        if token_pnl < 0i128 {
            self.pool.sub_amount(token_pnl.abs().cast::<u128>()?)?
        } else if add_liability == 0u128 {
            self.pool.add_amount(token_pnl.cast::<u128>()?)?
        } else {
            let u_token_pnl = token_pnl.abs().cast::<u128>()?;

            self.pool.add_amount(if u_token_pnl > add_liability { u_token_pnl.cast::<u128>()?.safe_sub(add_liability.cast::<u128>()?)?.cast::<u128>()? } else { 0u128 })?;
            self.pool.add_unsettle(if u_token_pnl > add_liability { add_liability } else { token_pnl.abs().cast::<u128>()? })?;
        }
        Ok(())
    }

    pub fn add_insurance_fund(&mut self, amount: u128) -> BumpResult<()> {
        self.pool.add_insurance_fund(amount)?;
        Ok(())
    }
}