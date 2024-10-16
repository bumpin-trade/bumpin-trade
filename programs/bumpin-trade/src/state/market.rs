use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::calculator;
use crate::math::casting::Cast;
use crate::math::constants::{PRICE_PRECISION, SMALL_RATE_TO_PER_TOKEN_PRECISION};
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
    pub stable_loss: i128, // short profit mean +/ otherwise mean -
    pub pool_key: Pubkey,
    pub pool_mint_key: Pubkey,
    pub index_mint_oracle: Pubkey,
    pub stable_pool_key: Pubkey,
    pub stable_pool_mint_key: Pubkey,
    pub stable_unsettle_loss: u128,
    pub index: u16,
    pub market_status: MarketStatus,
    pub share_short: bool,
    pub padding: [u8; 12],
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
    pub fn add_stable_loss(&mut self, amount: i128) -> BumpResult<()> {
        self.stable_loss = calculator::add_i128(self.stable_loss, amount)?;
        Ok(())
    }
    pub fn add_unsettle_stable_loss(&mut self, amount: u128) -> BumpResult<()> {
        self.stable_unsettle_loss = calculator::add_u128(self.stable_unsettle_loss, amount)?;
        Ok(())
    }

    pub fn sub_unsettle_stable_loss(&mut self, amount: u128) -> BumpResult<u128> {
        if self.stable_unsettle_loss > amount {
            let gap = calculator::sub_u128(amount, self.stable_unsettle_loss)?;
            self.stable_unsettle_loss = 0u128;
            return Ok(gap)
        }
        self.stable_unsettle_loss = calculator::sub_u128(self.stable_unsettle_loss, amount)?;
        Ok(0u128)
    }
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
                calculator::add_i128(self.funding_fee.total_long_funding_fee, amount)?;
        } else {
            self.funding_fee.total_short_funding_fee =
                calculator::add_i128(self.funding_fee.total_short_funding_fee, amount)?;
        }
        Ok(())
    }

    fn add_oi(&mut self, params: UpdateOIParams) -> BumpResult<()> {
        let market_position = if params.is_long {
            &mut self.long_open_interest
        } else {
            &mut self.short_open_interest
        };
        if market_position.open_interest == 0u128 {
            market_position.add_open_interest(params.size, params.entry_price)?;
        } else {
            let entry_price = calculator::compute_avg_entry_price(
                market_position.open_interest,
                market_position.entry_price,
                params.size,
                params.entry_price,
                self.config.tick_size,
                params.token_decimal,
                params.is_long,
            )?;

            market_position.add_open_interest(params.size, entry_price)?;
        }
        Ok(())
    }

    fn sub_oi(&mut self, params: UpdateOIParams) -> BumpResult<()> {
        let market_position = if params.is_long {
            &mut self.long_open_interest
        } else {
            &mut self.short_open_interest
        };
        let entry_price = calculator::compute_decrease_avg_entry_price(
            market_position.open_interest,
            market_position.entry_price,
            params.size,
            params.entry_price,
            self.config.tick_size,
            params.token_decimal,
            params.is_long,
        )?;

        market_position.sub_open_interest(params.size, entry_price)?;
        Ok(())
    }

    pub fn update_market_funding_fee_rate(&mut self, state: &State, price: u128) -> BumpResult<()> {
        let (
            update_time_only,
            long_funding_fee_per_qty_delta,
            short_funding_fee_per_qty_delta,
            funding_fee_duration_in_seconds,
        ) = {
            let funding_fee_duration_in_seconds =
                self.funding_fee.get_market_funding_fee_durations()?;
            let mut long_funding_fee_per_qty_delta = 0i128;
            let mut short_funding_fee_per_qty_delta = 0i128;
            let long = &self.long_open_interest;
            let short = &self.short_open_interest;
            if (long.open_interest == 0u128 && short.open_interest == 0u128)
                || long.open_interest == short.open_interest
                || funding_fee_duration_in_seconds == 0
            {
                (
                    true,
                    long_funding_fee_per_qty_delta,
                    short_funding_fee_per_qty_delta,
                    funding_fee_duration_in_seconds,
                )
            } else {
                let long_pay_short = long.open_interest > short.open_interest;
                let funding_rate_per_second = {
                    let long_position_interest = long.open_interest;
                    let short_position_interest = short.open_interest;
                    let diff = calculator::diff_u(long_position_interest, short_position_interest)?;
                    let open_interest = long_position_interest.safe_add(short_position_interest)?;
                    if diff == 0u128 || open_interest == 0u128 {
                        0u128;
                    }
                    calculator::mul_div_u(diff, state.funding_fee_base_rate, open_interest)?
                };
                let total_funding_fee = long
                    .open_interest //^10
                    .max(short.open_interest)
                    .safe_mul(funding_fee_duration_in_seconds.abs().cast::<u128>()?)?
                    .safe_mul(funding_rate_per_second)?; //^10
                if long.open_interest > 0 {
                    let current_long_funding_fee_per_qty = if long_pay_short {
                        total_funding_fee
                            .safe_div(long.open_interest)?
                            .safe_mul(SMALL_RATE_TO_PER_TOKEN_PRECISION)?
                    } else {
                        calculator::div_to_precision_u(
                            state
                                .maximum_funding_base_rate
                                .safe_mul(funding_fee_duration_in_seconds.abs().cast::<u128>()?)?
                                .min(total_funding_fee.safe_div(long.open_interest)?),
                            1u128,
                            SMALL_RATE_TO_PER_TOKEN_PRECISION,
                        )? //^18
                    };
                    long_funding_fee_per_qty_delta = calculator::mul_div_u(
                        current_long_funding_fee_per_qty, //^18
                        PRICE_PRECISION,
                        price,
                    )?
                    .cast::<i128>()?;

                    long_funding_fee_per_qty_delta = if long_pay_short {
                        long_funding_fee_per_qty_delta
                    } else {
                        -long_funding_fee_per_qty_delta
                    };
                }
                if short.open_interest > 0 {
                    short_funding_fee_per_qty_delta = if long_pay_short {
                        calculator::div_to_precision_i(
                            -state
                                .maximum_funding_base_rate
                                .safe_mul(funding_fee_duration_in_seconds.abs().cast::<u128>()?)?
                                .min(total_funding_fee.safe_div(short.open_interest)?)
                                .cast::<i128>()?,
                            1i128,
                            SMALL_RATE_TO_PER_TOKEN_PRECISION.cast::<i128>()?,
                        )?
                    } else {
                        calculator::div_to_precision_i(
                            total_funding_fee.safe_div(short.open_interest)?.cast::<i128>()?,
                            1i128,
                            SMALL_RATE_TO_PER_TOKEN_PRECISION.cast::<i128>()?,
                        )?
                    }
                }
                (
                    false,
                    long_funding_fee_per_qty_delta,
                    short_funding_fee_per_qty_delta,
                    funding_fee_duration_in_seconds,
                )
            }
        };
        if update_time_only {
            self.funding_fee.update_last_update()?;
            return Ok(());
        }

        self.funding_fee.update_market_funding_fee_rate(
            short_funding_fee_per_qty_delta,
            long_funding_fee_per_qty_delta,
            funding_fee_duration_in_seconds,
        )?;
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
        self.open_interest = calculator::add_u128(self.open_interest, size)?;
        self.entry_price = price;
        Ok(())
    }
    pub fn sub_open_interest(&mut self, size: u128, price: u128) -> BumpResult<()> {
        if self.open_interest <= size {
            self.open_interest = 0u128;
            self.entry_price = 0u128;
        } else {
            self.entry_price = price;
            self.open_interest = calculator::sub_u128(self.open_interest, size)?;
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
    pub max_pool_liquidity_share_rate: u32,
    pub padding: [u8; 4],
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Default, Copy)]
pub struct UpdateOIParams {
    pub margin_token: Pubkey,
    pub size: u128,
    pub is_long: bool,
    pub entry_price: u128,
    pub token_decimal: u16,
}
