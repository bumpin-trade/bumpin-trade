use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::market_funding_fee::MarketFundingFee;
use crate::state::oracle_map::OracleMap;
use crate::state::state::State;
use crate::traits::Size;
use anchor_lang::prelude::*;
use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Market {
    pub symbol: [u8; 32],
    pub long_open_interest: MarketPosition,
    pub short_open_interest: MarketPosition,
    pub funding_fee: MarketFundingFee,
    pub config: MarketConfig,
    pub pool_key: Pubkey,
    pub pool_mint_key: Pubkey,
    pub index_mint_oracle: Pubkey,
    pub stable_pool_key: Pubkey,
    pub stable_pool_mint_key: Pubkey,
    pub index: u16,
    pub market_status: MarketStatus,
    pub padding: [u8; 13],
    pub reserve_padding: [u8; 32],
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum MarketStatus {
    #[default]
    NORMAL,
    ReduceOnly,
    Pause,
}

impl Size for Market {
    const SIZE: usize = std::mem::size_of::<Market>() + 8;
}

impl Market {
    pub fn update_oi(&mut self, add: bool, params: UpdateOIParams) -> BumpResult<()> {
        if add {
            self.add_oi(params)
        } else {
            self.sub_oi(params)
        }
    }

    pub fn update_market_total_funding_fee(
        &mut self,
        amount: i128,
        is_long: bool,
    ) -> BumpResult<()> {
        if is_long {
            self.funding_fee.total_long_funding_fee =
                cal_utils::add_i128(self.funding_fee.total_long_funding_fee, amount)?;
        } else {
            self.funding_fee.total_short_funding_fee =
                cal_utils::add_i128(self.funding_fee.total_short_funding_fee, amount)?;
        }
        Ok(())
    }

    fn add_oi(&mut self, params: UpdateOIParams) -> BumpResult<()> {
        let mut market_position =
            if params.is_long { self.long_open_interest } else { self.short_open_interest };
        if market_position.open_interest == 0u128 {
            market_position.add_open_interest(params.size, params.entry_price)?;
        } else {
            let entry_price = cal_utils::compute_avg_entry_price(
                market_position.open_interest,
                market_position.entry_price,
                params.size,
                params.entry_price,
                self.config.tick_size,
                params.is_long,
            )?;

            market_position.add_open_interest(params.size, entry_price)?;
        }
        Ok(())
    }

    fn sub_oi(&mut self, params: UpdateOIParams) -> BumpResult<()> {
        let mut market_position =
            if params.is_long { self.long_open_interest } else { self.short_open_interest };

        market_position.sub_open_interest(params.size, params.entry_price)?;
        Ok(())
    }

    pub fn update_market_funding_fee_rate(
        &mut self,
        state: &State,
        price: u128,
        decimals: u16,
    ) -> BumpResult<()> {
        let long = &self.long_open_interest;
        let short = &self.short_open_interest;
        if (long.open_interest == 0u128 && short.open_interest == 0u128)
            || long.open_interest == short.open_interest
        {
            self.funding_fee.update_last_update()?;
            return Ok(());
        }
        let fee_durations = self.funding_fee.get_market_funding_fee_durations()?;
        if fee_durations > 0 {
            let funding_rate_per_second = long
                .open_interest
                .cast::<i128>()?
                .safe_sub(short.open_interest.cast()?)?
                .safe_div(
                    long.open_interest
                        .cast::<i128>()?
                        .safe_add(short.open_interest.cast::<i128>()?)?,
                )?
                .safe_mul(state.funding_fee_base_rate.cast()?)?;

            let funding_fee = long
                .open_interest
                .cast::<i128>()?
                .max(short.open_interest.cast::<i128>()?)
                .safe_mul(funding_rate_per_second)?
                .safe_mul(fee_durations.cast()?)?;

            let mut long_funding_fee_amount_per_size_delta = cal_utils::usd_to_token_i(
                funding_fee.safe_div(long.open_interest.cast::<i128>()?)?,
                decimals,
                price.cast()?,
            )?;

            long_funding_fee_amount_per_size_delta = long_funding_fee_amount_per_size_delta.min(
                state
                    .maximum_funding_base_rate
                    .cast::<i128>()?
                    .safe_mul(funding_rate_per_second.cast()?)?,
            );

            let mut short_funding_fee_amount_per_size_delta =
                funding_fee.safe_div(short.open_interest.cast::<i128>()?)?.safe_mul(-1i128)?;

            short_funding_fee_amount_per_size_delta = short_funding_fee_amount_per_size_delta.min(
                state
                    .maximum_funding_base_rate
                    .cast::<i128>()?
                    .safe_mul(funding_rate_per_second)?,
            );

            self.funding_fee.update_market_funding_fee_rate(
                short_funding_fee_amount_per_size_delta,
                long_funding_fee_amount_per_size_delta,
                fee_durations,
            )?;
        }
        self.funding_fee.update_last_update()
    }

    pub fn get_market_un_pnl(&self, is_long: bool, oracle_map: &mut OracleMap) -> BumpResult<i128> {
        let position = if is_long { &self.long_open_interest } else { &self.short_open_interest };
        let mark_price = oracle_map
            .get_price_data(&self.index_mint_oracle)
            .map_err(|_e| BumpErrorCode::OracleNotFound)?
            .price;
        if position.entry_price == 0u128 {
            return Ok(0i128);
        };
        if is_long {
            let pnl_in_usd = position
                .open_interest
                .cast::<i128>()?
                .safe_mul(
                    mark_price.cast::<i128>()?.safe_sub(position.entry_price.cast::<i128>()?)?,
                )?
                .safe_div(position.entry_price.cast::<i128>()?)?;
            Ok(-pnl_in_usd)
        } else {
            let pnl_in_usd = position
                .open_interest
                .cast::<i128>()?
                .safe_mul(
                    position.entry_price.cast::<i128>()?.safe_sub(mark_price.cast::<i128>()?)?,
                )?
                .safe_div(position.entry_price.cast::<i128>()?)?;
            Ok(-pnl_in_usd)
        }
    }
}

#[bumpin_zero_copy_unsafe]
pub struct MarketPosition {
    pub open_interest: u128,
    pub entry_price: u128,
}

impl MarketPosition {
    pub fn add_open_interest(&mut self, size: u128, price: u128) -> BumpResult<()> {
        self.open_interest = cal_utils::add_u128(self.open_interest, size)?;
        self.entry_price = price;
        Ok(())
    }
    pub fn sub_open_interest(&mut self, size: u128, price: u128) -> BumpResult<()> {
        if self.open_interest <= size {
            self.open_interest = 0u128;
            self.entry_price = 0u128;
        } else {
            self.entry_price = self
                .open_interest
                .safe_mul(self.entry_price)?
                .safe_sub(size.safe_mul(price)?)?
                .safe_div(self.open_interest.safe_sub(size)?)?;
            self.open_interest = cal_utils::sub_u128(self.open_interest, size)?;
        }
        Ok(())
    }
}

#[bumpin_zero_copy_unsafe]
pub struct MarketConfig {
    pub tick_size: u128,
    pub open_fee_rate: u128,
    pub close_fee_rate: u128,
    pub maximum_long_open_interest_cap: u128,
    pub maximum_short_open_interest_cap: u128,
    pub long_short_ratio_limit: u128,
    pub long_short_oi_bottom_limit: u128,
    pub maximum_leverage: u32,
    pub minimum_leverage: u32,
    pub padding: [u8; 8],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct UpdateOIParams {
    pub margin_token: Pubkey,
    pub size: u128,
    pub is_long: bool,
    pub entry_price: u128,
}
