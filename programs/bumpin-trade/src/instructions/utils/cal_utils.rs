use num_traits::ToPrimitive;
use solana_program::clock::Clock;
use solana_program::sysvar::Sysvar;
use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::constants::{PRICE_TO_LAMPORT, RATE_PRECISION, SMALL_RATE_PRECISION};
use crate::math::safe_math::SafeMath;


pub fn mul_div_i(a: i128, b: i128, denominator: i128) -> BumpResult<i128> {
    Ok(a.safe_mul(b)?.safe_div(denominator)?)
}

pub fn mul_div_u(a: u128, b: u128, denominator: u128) -> BumpResult<u128> {
    Ok(a.safe_mul(b)?.safe_div(denominator)?)
}

pub fn mul_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, rate, RATE_PRECISION)
}

pub fn mul_rate_i(value: i128, rate: i128) -> BumpResult<i128> {
    mul_div_i(value, rate, RATE_PRECISION.cast::<i128>()?)
}

pub fn div_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, RATE_PRECISION, rate)
}

pub fn div_to_precision_u(a: u128, b: u128, precision: u128) -> BumpResult<u128> {
    mul_div_u(a, precision, b)
}

pub fn mul_small_rate_u(value: u128, rate: u128) -> BumpResult<u128> {
    mul_div_u(value, rate, SMALL_RATE_PRECISION)
}

pub fn mul_small_rate_i(value: i128, rate: i128) -> BumpResult<i128> {
    mul_div_i(value, rate, SMALL_RATE_PRECISION.cast::<i128>()?)
}

pub fn div_rate_i(value: i128, rate: i128) -> BumpResult<i128> {
    mul_div_i(value, SMALL_RATE_PRECISION.cast::<i128>()?, rate)
}


pub fn add_u128(x: u128, y: u128) -> BumpResult<u128> {
    Ok(x.safe_add(y)?)
}

pub fn sub_u128(x: u128, y: u128) -> BumpResult<u128> {
    Ok(x.safe_sub(y)?)
}

pub fn sub_i128(x: i128, y: i128) -> BumpResult<i128> {
    Ok(x.safe_sub(y)?)
}

pub fn add_i128(x: i128, y: i128) -> BumpResult<i128> {
    Ok(x.safe_add(y)?)
}

pub fn mul_u128(x: u128, y: u128) -> BumpResult<u128> {
    Ok(x.safe_mul(y)?)
}

pub fn div_u128(x: u128, y: u128) -> BumpResult<u128> {
    Ok(x.safe_div(y)?)
}

pub fn usd_to_token_u(usd_value: u128, decimals: u8, token_price: u128) -> BumpResult<u128> {
    mul_div_u(usd_value, 10u128.pow(decimals.cast::<u32>()?), token_price.safe_mul(PRICE_TO_LAMPORT)?)
}

pub fn usd_to_token_i(usd_value: i128, decimals: u8, token_price: u128) -> BumpResult<i128> {
    mul_div_i(usd_value, 10i128.pow(decimals.cast::<u32>()?), token_price.cast::<i128>()?.safe_mul(PRICE_TO_LAMPORT.cast::<i128>()?)?)
}

pub fn token_to_usd_u(token_amount: u128, decimals: u8, token_price: u128) -> BumpResult<u128> {
    token_amount.safe_mul(token_price)?.safe_mul(PRICE_TO_LAMPORT)?.safe_div(10u128.pow(decimals.cast::<u32>()?))
}

pub fn token_to_usd_i(token_amount: i128, decimals: u8, token_price: u128) -> BumpResult<i128> {
    token_amount.safe_mul(token_price.cast::<i128>()?)?.safe_mul(PRICE_TO_LAMPORT.cast::<i128>()?)?.safe_div(10i128.pow(decimals.cast::<u32>()?))
}


pub fn current_time() -> u128 {
    let clock = Clock::get().unwrap();
    clock.unix_timestamp.to_u128().unwrap()
}

pub fn compute_avg_entry_price(amount: u128, entry_price: u128, increase_amount: u128, token_price: u128, ticker_size: u128, up: bool) -> BumpResult<u128> {
    let origin_entry_price = amount
        .safe_mul(entry_price)?
        .safe_add(increase_amount
            .safe_mul(token_price)?)?
        .safe_mul(amount
            .safe_add(increase_amount)?)?;
    return format_to_ticker_size(origin_entry_price, ticker_size, up);
}

fn format_to_ticker_size(price: u128, ticker_size: u128, up: bool) -> BumpResult<u128> {
    let remainder = price % ticker_size;
    return if remainder == 0u128 {
        Ok(price)
    } else {
        Ok(price.safe_div(ticker_size)?
            .safe_add(if up { 0 } else { 1 })?
            .safe_mul(ticker_size)?)
    };
}