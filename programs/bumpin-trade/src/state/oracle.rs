use anchor_lang::prelude::*;
use pyth_sdk_solana::state::SolanaPriceAccount;

use crate::errors::BumpErrorCode::{InvalidOracle, PythOffline};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::casting::Cast;
use crate::math::constants::PRICE_PRECISION;
use crate::math::safe_math::SafeMath;

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

pub fn get_oracle_price(price_oracle: &AccountInfo) -> BumpResult<OraclePriceData> {
    get_pyth_price(price_oracle, 1)
}

pub fn get_pyth_price(price_oracle: &AccountInfo, multiple: u128) -> BumpResult<OraclePriceData> {
    let price_feed =
        SolanaPriceAccount::account_info_to_feed(price_oracle).map_err(|_e| PythOffline)?;
    let current_timestamp =
        Clock::get().map_err(|_e| BumpErrorCode::TimestampNotFound)?.unix_timestamp;
    let price_data =
        price_feed.get_price_no_older_than(current_timestamp, 180).ok_or(PythOffline)?;
    let oracle_price = price_data.price;
    let oracle_conf = price_data.conf;

    let oracle_precision = 10_u128.pow(price_data.expo.unsigned_abs());

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
