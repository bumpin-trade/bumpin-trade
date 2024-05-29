use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpResult;
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::market::Market;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;

pub struct MarketProcessor<'a> {
    pub(crate) market: &'a mut Market,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct UpdateOIParams {
    pub margin_token: Pubkey,
    pub size: u128,
    pub is_long: bool,
    pub entry_price: u128,
}

impl<'a> MarketProcessor<'_> {
    pub fn get_market_un_pnl(&mut self, is_long: bool, oracle_map: &OracleMap) -> BumpResult<u128> {
        //todo
        Ok(0u128)
    }
    pub fn update_oi(&mut self, add: bool, params: UpdateOIParams) -> BumpResult<()> {
        if add {
            self.add_oi(params)?
        } else {
            self.sub_oi(params)?
        }
        Ok(())
    }

    fn add_oi(&self, params: UpdateOIParams) -> BumpResult<()> {
        let mut market_position = if params.is_long { self.market.long_open_interest } else { self.market.short_open_interest };
        if market_position.open_interest == 0u128 {
            market_position.add_open_interest(params.size, params.entry_price);
        } else {
            let entry_price = cal_utils::compute_avg_entry_price(market_position.open_interest,
                                                                 market_position.entry_price,
                                                                 params.size,
                                                                 params.entry_price,
                                                                 self.market.ticker_size,
                                                                 params.is_long)?;

            market_position.add_open_interest(params.size, entry_price);
        }
        Ok(())
    }

    fn sub_oi(&self, params: UpdateOIParams) -> BumpResult<()> {
        let mut market_position = if params.is_long { self.market.long_open_interest } else { self.market.short_open_interest };
        market_position.sub_open_interest(params.size);
        Ok(())
    }
    pub fn update_market_funding_fee_rate(&mut self, state: &State, oracle_price: &mut OracleMap) -> BumpResult<()> {
        let oracle_price_data = oracle_price.get_price_data(&self.market.pool_mint_key)?;
        let long = self.market.long_open_interest;
        let short = self.market.short_open_interest;

        let fee_durations = self.market.funding_fee.get_market_funding_fee_durations()?;
        if fee_durations > 0 {
            let funding_rate_per_second = long.open_interest.cast::<i128>()?.safe_sub(short.open_interest.cast()?)?.
                safe_div(long.open_interest.cast::<i128>()?.
                    safe_add(short.open_interest.cast::<i128>()?)?)?.
                safe_mul(state.funding_fee_base_rate.cast()?)?;

            let funding_fee = long.open_interest.cast::<i128>()?.max(short.open_interest.cast::<i128>()?).
                safe_mul(funding_rate_per_second)?.
                safe_mul(fee_durations.cast()?)?;

            let mut long_funding_fee_amount_per_size_delta = funding_fee.
                safe_div(long.open_interest.cast::<i128>()?.
                    safe_mul(oracle_price_data.price.cast()?)?)?;

            long_funding_fee_amount_per_size_delta = long_funding_fee_amount_per_size_delta.
                min(state.max_funding_base_rate.cast::<i128>()?.
                    safe_mul(funding_rate_per_second.cast()?)?);

            let mut short_funding_fee_amount_per_size_delta = funding_fee.
                safe_div(short.open_interest.cast::<i128>()?.
                    safe_mul(oracle_price_data.price.cast::<i128>()?)?)?.safe_mul(-1i128)?;

            short_funding_fee_amount_per_size_delta = short_funding_fee_amount_per_size_delta.
                min(state.max_funding_base_rate.cast::<i128>()?.
                    safe_mul(funding_rate_per_second.cast()?)?);

            self.market.funding_fee.update_market_funding_fee_rate(short_funding_fee_amount_per_size_delta, long_funding_fee_amount_per_size_delta, fee_durations)?;
        }

        self.market.funding_fee.update_last_update();
        Ok(())
    }

    pub fn update_market_total_funding_fee(&mut self,
                                           amount: i128,
                                           update_unsettle: bool,
                                           is_long: bool,
                                           is_add: bool,
                                           pool: &mut Pool) -> BumpResult<()> {
        if is_long {
            if is_add {
                self.market.funding_fee.total_long_funding_fee = cal_utils::add_i128(self.market.funding_fee.total_long_funding_fee, amount)?;
            } else {
                self.market.funding_fee.total_long_funding_fee = cal_utils::sub_i128(self.market.funding_fee.total_long_funding_fee, amount)?;
            }
        } else {
            if is_add {
                self.market.funding_fee.short_funding_fee_rate = cal_utils::add_i128(self.market.funding_fee.short_funding_fee_rate, amount)?;
            } else {
                self.market.funding_fee.short_funding_fee_rate = cal_utils::sub_i128(self.market.funding_fee.short_funding_fee_rate, amount)?;
            }
        }

        if update_unsettle {
            pool.add_unsettle(amount)?
        }

        Ok(())
    }
}