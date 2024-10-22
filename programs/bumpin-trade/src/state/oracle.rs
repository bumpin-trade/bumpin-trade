use crate::errors::BumpErrorCode::{InvalidOracle, PythOffline};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::casting::Cast;
use crate::math::constants::PRICE_PRECISION;
use crate::math::safe_math::SafeMath;
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{FeedId, PriceUpdateV2};

#[derive(Default, Clone, Copy, Debug)]
pub struct OraclePriceData {
    pub price: u128,
    pub confidence: u128,
}

impl OraclePriceData {
    pub fn default_usd() -> Self {
        OraclePriceData { price: PRICE_PRECISION, confidence: 1 }
    }
}

pub fn get_oracle_price(
    feed_id: &FeedId,
    price_oracle: &PriceUpdateV2,
) -> BumpResult<OraclePriceData> {
    get_pyth_price(feed_id, price_oracle, 1)
}

pub fn get_pyth_price(
    feed_id: &FeedId,
    price_oracle: &PriceUpdateV2,
    multiple: u128,
) -> BumpResult<OraclePriceData> {
    let clock = Clock::get().map_err(|_e| BumpErrorCode::TimestampNotFound)?;

    let price_data = price_oracle.get_price_no_older_than(&clock, 180, feed_id).map_err(|_e| {
        msg!("=====get_pyth_price:{} ", _e.to_string());
        PythOffline
    })?;
    let oracle_price = price_data.price;
    let oracle_conf = price_data.conf;

    let oracle_precision = 10_u128.pow(price_data.exponent.unsigned_abs());

    if oracle_precision <= multiple {
        msg!("Multiple larger than oracle precision");
        return Err(InvalidOracle);
    }

    let oracle_precision = oracle_precision.safe_div(multiple)?;

    let mut oracle_scale_mult = 1;
    let mut oracle_scale_div = 1;

    if oracle_precision > PRICE_PRECISION {
        oracle_scale_div = oracle_precision.safe_div(PRICE_PRECISION)?;
    } else {
        oracle_scale_mult = PRICE_PRECISION.safe_div(oracle_precision)?;
    }

    let oracle_price_scaled = (oracle_price)
        .cast::<i128>()?
        .safe_mul(oracle_scale_mult.cast()?)?
        .safe_div(oracle_scale_div.cast()?)?
        .cast::<u128>()?;

    let oracle_conf_scaled = (oracle_conf)
        .cast::<u128>()?
        .safe_mul(oracle_scale_mult)?
        .safe_div(oracle_scale_div)?
        .cast::<u128>()?;

    Ok(OraclePriceData { price: oracle_price_scaled, confidence: oracle_conf_scaled })
}
