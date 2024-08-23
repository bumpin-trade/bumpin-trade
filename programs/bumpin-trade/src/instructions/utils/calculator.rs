use anchor_lang::prelude::*;

use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::constants::{
    PER_TOKEN_PRECISION, PRICE_TO_USD_PRECISION, RATE_PRECISION, SMALL_RATE_PRECISION,
};
use crate::math::safe_math::SafeMath;

pub fn mul_div_i(a: i128, b: i128, denominator: i128) -> BumpResult<i128> {
    a.safe_mul(b)?.safe_div(denominator)
}

pub fn mul_div_u(a: u128, b: u128, denominator: u128) -> BumpResult<u128> {
    a.safe_mul(b)?.safe_div(denominator)
}

pub fn mul_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, rate, RATE_PRECISION)
}

pub fn mul_rate_i(value: i128, rate: i128) -> BumpResult<i128> {
    mul_div_i(value, rate, RATE_PRECISION.cast::<i128>()?)
}

pub fn mul_per_token_rate_i(value: i128, rate: i128) -> BumpResult<i128> {
    mul_div_i(value, rate, PER_TOKEN_PRECISION.cast::<i128>()?)
}

pub fn mul_per_token_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, rate, PER_TOKEN_PRECISION)
}

pub fn div_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, RATE_PRECISION, rate)
}

pub fn div_to_precision_u(a: u128, b: u128, precision: u128) -> BumpResult<u128> {
    mul_div_u(a, precision, b)
}

pub fn div_to_precision_i(a: i128, b: i128, precision: i128) -> BumpResult<i128> {
    mul_div_i(a, precision, b)
}

pub fn mul_small_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, rate, SMALL_RATE_PRECISION)
}

pub fn mul_small_rate_i(value: i128, rate: i128) -> BumpResult<i128> {
    mul_div_i(value, rate, SMALL_RATE_PRECISION.cast::<i128>()?)
}

pub fn div_small_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, SMALL_RATE_PRECISION, rate)
}

pub fn div_small_rate_i(value: i128, rate: i128) -> BumpResult<i128> {
    mul_div_i(value, SMALL_RATE_PRECISION.cast::<i128>()?, rate)
}

pub fn add_u128(x: u128, y: u128) -> BumpResult<u128> {
    x.safe_add(y)
}

pub fn sub_u128(x: u128, y: u128) -> BumpResult<u128> {
    x.safe_sub(y)
}

pub fn sub_i128(x: i128, y: i128) -> BumpResult<i128> {
    x.safe_sub(y)
}

pub fn add_i128(x: i128, y: i128) -> BumpResult<i128> {
    x.safe_add(y)
}

pub fn mul_u128(x: u128, y: u128) -> BumpResult<u128> {
    x.safe_mul(y)
}

pub fn div_u128(x: u128, y: u128) -> BumpResult<u128> {
    x.safe_div(y)
}

pub fn diff_u(x: u128, y: u128) -> BumpResult<u128> {
    if x == y {
        return Ok(0u128);
    }
    if x > y {
        Ok(x.safe_sub(y)?)
    } else {
        Ok(y.safe_sub(x)?)
    }
}

pub fn usd_to_token_u(usd_value: u128, decimals: u16, token_price: u128) -> BumpResult<u128> {
    mul_div_u(
        usd_value,
        10u128.pow(decimals.cast::<u32>()?),
        token_price.safe_mul(PRICE_TO_USD_PRECISION)?,
    )
}

pub fn usd_to_token_i(usd_value: i128, decimals: u16, token_price: u128) -> BumpResult<i128> {
    mul_div_i(
        usd_value,
        10i128.pow(decimals.cast::<u32>()?),
        token_price.cast::<i128>()?.safe_mul(PRICE_TO_USD_PRECISION.cast::<i128>()?)?,
    )
}

pub fn token_to_usd_u(token_amount: u128, decimals: u16, token_price: u128) -> BumpResult<u128> {
    token_amount
        .safe_mul(token_price)?
        .safe_mul(PRICE_TO_USD_PRECISION)?
        .safe_div(10u128.pow(decimals.cast::<u32>()?))
}

pub fn token_to_usd_i(token_amount: i128, decimals: u16, token_price: u128) -> BumpResult<i128> {
    token_amount
        .safe_mul(token_price.cast::<i128>()?)?
        .safe_mul(PRICE_TO_USD_PRECISION.cast::<i128>()?)?
        .safe_div(10i128.pow(decimals.cast::<u32>()?))
}

pub fn current_time() -> i64 {
    let clock = Clock::get().unwrap();
    clock.unix_timestamp
}

pub fn compute_avg_entry_price(
    origin_size: u128,
    origin_entry_price: u128,
    increase_size: u128,
    token_price: u128,
    ticker_size: u128,
    decimal: u16,
    up: bool,
) -> BumpResult<u128> {
    let origin_token_amount = usd_to_token_u(origin_size, decimal, origin_entry_price)?;
    let increase_amount = usd_to_token_u(increase_size, decimal, token_price)?;
    let total_size = origin_size.safe_add(increase_size)?;
    let entry_price = total_size
        .safe_mul(10u128.pow(decimal.cast()?))?
        .safe_div(origin_token_amount.safe_add(increase_amount)?)?
        .safe_div(PRICE_TO_USD_PRECISION)?;
    format_to_ticker_size(entry_price, ticker_size, up)
}

pub fn compute_decrease_avg_entry_price(
    origin_size: u128,
    origin_entry_price: u128,
    decrease_size: u128,
    token_price: u128,
    ticker_size: u128,
    decimal: u16,
    up: bool,
) -> BumpResult<u128> {
    let origin_token_amount = usd_to_token_u(origin_size, decimal, origin_entry_price)?;
    let decrease_amount = usd_to_token_u(decrease_size, decimal, token_price)?;
    let total_size = origin_size.safe_sub(decrease_size)?;
    if total_size == 0u128 {
        return Ok(0u128);
    }
    let entry_price = total_size
        .safe_mul(10u128.pow(decimal.cast()?))?
        .safe_div(origin_token_amount.safe_sub(decrease_amount)?)?
        .safe_div(PRICE_TO_USD_PRECISION)?;
    format_to_ticker_size(entry_price, ticker_size, up)
}

pub fn format_to_ticker_size(price: u128, ticker_size: u128, up: bool) -> BumpResult<u128> {
    let remainder = price % ticker_size;
    if remainder == 0u128 {
        Ok(price)
    } else {
        Ok(price.safe_div(ticker_size)?.safe_add(if up { 0 } else { 1 })?.safe_mul(ticker_size)?)
    }
}

pub fn get_mm(size: u128, leverage: u32, max_mm_rate: u32) -> BumpResult<u128> {
    Ok(size
        .safe_div_rate(leverage.safe_mul(2)? as u128)?
        .min(size.safe_mul_rate(max_mm_rate as u128)?))
}

pub fn get_mm_rate(leverage: u32, max_mm_rate: u32) -> BumpResult<u128> {
    Ok(div_rate_u(RATE_PRECISION, leverage.safe_mul(2)? as u128)?.min(max_mm_rate as u128))
}
