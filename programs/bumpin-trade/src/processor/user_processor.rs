use anchor_lang::prelude::*;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::user_position::PositionStatus;
use crate::state::oracle_map::OracleMap;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::{User, UserTokenUpdateReason};
use crate::validate;

pub fn withdraw(
    user: &mut User,
    amount: u128,
    oracle: &Pubkey,
    trade_token: &TradeToken,
    token_mint: &Pubkey,
    oracle_map: &mut OracleMap,
    trade_token_map: &TradeTokenMap,
) -> BumpResult {
    let price = oracle_map.get_price_data(oracle)?.price;
    let withdraw_usd = cal_utils::token_to_usd_u(amount, trade_token.decimals, price)?;

    let available_value = user.get_available_value(oracle_map, trade_token_map)?;
    validate!(
        available_value.abs().cast::<u128>()? > withdraw_usd,
        BumpErrorCode::UserNotEnoughValue
    )?;
    user.sub_user_token_amount_ignore_used_amount(
        token_mint,
        amount,
        &UserTokenUpdateReason::WITHDRAW,
    )?;
    update_cross_position_balance(user, token_mint, amount, false)?;
    Ok(())
}

pub fn update_cross_position_balance(
    user: &mut User,
    mint: &Pubkey,
    amount: u128,
    add_amount: bool,
) -> BumpResult<()> {
    let mut reduce_amount = amount;
    for user_position in user.positions.iter_mut() {
        if user_position.status.eq(&PositionStatus::INIT) {
            continue;
        }
        if user_position.is_portfolio_margin
            && user_position.margin_mint_key.eq(mint)
            && reduce_amount > 0
        {
            if add_amount {
                let change_amount = user_position.add_position_portfolio_balance(reduce_amount)?;
                reduce_amount = reduce_amount.safe_sub(change_amount)?;
            } else {
                let change_amount =
                    user_position.reduce_position_portfolio_balance(reduce_amount)?;
                reduce_amount = reduce_amount.safe_sub(change_amount)?;
            }
        }

        if reduce_amount == 0u128 {
            break;
        }
    }
    Ok(())
}
