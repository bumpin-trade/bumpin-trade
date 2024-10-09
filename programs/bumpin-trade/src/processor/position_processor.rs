use std::ops::{Deref, DerefMut};

use anchor_lang::emit;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{calculator, UpdatePositionLeverageParams, UpdatePositionMarginParams};
use crate::math::casting::Cast;
use crate::math::constants::RATE_PRECISION;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::state::bump_events::{
    AddOrDecreaseMarginEvent, AddOrDeleteUserPositionEvent, UpdateUserPositionEvent,
};
use crate::state::infrastructure::user_order::{
    OrderSide, OrderType, PositionSide, StopType, UserOrder,
};
use crate::state::infrastructure::user_position::UserPosition;
use crate::state::market::{Market, UpdateOIParams};
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::pool_map::PoolMap;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::{User, UserTokenUpdateReason};
use crate::state::vault_map::VaultMap;
use crate::utils::{pda, token};
use crate::validate;

#[track_caller]
pub fn handle_execute_order<'info>(
    user: &mut User,
    market_map: &MarketMap,
    pool_map: &PoolMap,
    state_account: &Account<'info, State>,
    user_token_account: Option<&Account<'info, TokenAccount>>,
    vault_map: &VaultMap<'info>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    program_id: &Pubkey,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    user_order: &UserOrder,
) -> BumpResult<()> {
    msg!("===========handle_execute_order start");
    let user_key = user.key;
    let mut market = market_map.get_mut_ref(&user_order.symbol)?;
    let mut base_token_pool = pool_map.get_mut_ref(&market.pool_key)?;
    let mut stable_pool = pool_map.get_mut_ref(&market.stable_pool_key)?;
    let mut trade_token = trade_token_map.get_trade_token_by_mint_ref_mut(&market.pool_mint_key)?;
    let mut stable_trade_token =
        trade_token_map.get_trade_token_by_mint_ref_mut(&market.stable_pool_mint_key)?;

    //validate order
    validate_execute_order(&user_order, &market)?;
    let is_long = OrderSide::LONG == user_order.order_side;
    let execute_price = get_execution_price(
        oracle_map
            .get_price_data(&market.index_mint_oracle)
            .map_err(|_e| BumpErrorCode::OracleNotFound)?
            .price,
        &user_order,
    )?;

    let margin_token_price = oracle_map
        .get_price_data(match use_base_token(&user_order.position_side, &user_order.order_side)? {
            true => &trade_token.oracle_key,
            false => &stable_trade_token.oracle_key,
        })
        .map_err(|_e| BumpErrorCode::OracleNotFound)?
        .price;
    //update funding_fee_rate and borrowing_fee_rate
    market.deref_mut().update_market_funding_fee_rate(state_account, margin_token_price)?;
    match use_base_token(&user_order.position_side, &user_order.order_side)? {
        true => base_token_pool.deref_mut().update_pool_borrowing_fee_rate()?,
        false => stable_pool.deref_mut().update_pool_borrowing_fee_rate()?,
    }

    let position_key = pda::generate_position_key(
        &user_key,
        market.symbol,
        user_order.is_portfolio_margin,
        program_id,
    )?;
    msg!("===========handle_execute_order start1111");
    // //do execute order and change position
    match user_order.position_side {
        PositionSide::NONE => Err(BumpErrorCode::PositionSideNotSupport),
        PositionSide::INCREASE => {
            {
                msg!("===========INCREASE start");
                let margin_token =
                    match use_base_token(&user_order.position_side, &user_order.order_side)? {
                        true => market.pool_mint_key,
                        false => market.stable_pool_mint_key,
                    };
                let trade_token_decimals = trade_token.decimals;
                let stable_trade_token_decimals = stable_trade_token.decimals;
                drop(trade_token);
                drop(stable_trade_token);
                //calculate real order_margin with validation
                let (order_margin, order_margin_from_balance) = execute_increase_order_margin(
                    &user_order,
                    &margin_token,
                    match use_base_token(&user_order.position_side, &user_order.order_side)? {
                        true => trade_token_decimals,
                        false => stable_trade_token_decimals,
                    },
                    user,
                    margin_token_price,
                    oracle_map,
                    trade_token_map,
                    state_account,
                )?;

                let mut trade_token =
                    trade_token_map.get_trade_token_by_mint_ref_mut(&market.pool_mint_key)?;
                let mut stable_trade_token = trade_token_map
                    .get_trade_token_by_mint_ref_mut(&market.stable_pool_mint_key)?;
                //collect open fee
                let fee = fee_processor::collect_open_position_fee(
                    &market,
                    if user_order.order_side.eq(&OrderSide::LONG) {
                        base_token_pool.deref_mut()
                    } else {
                        stable_pool.deref_mut()
                    },
                    order_margin.safe_mul_rate(user_order.leverage.cast()?)?,
                    user_order.is_portfolio_margin,
                )?;

                //record fee in user
                if user_order.is_portfolio_margin {
                    user.un_use_token(&user_order.margin_mint_key, fee)?;
                    user.sub_token_with_liability(
                        &user_order.margin_mint_key,
                        if user_order.order_side.eq(&OrderSide::LONG) {
                            trade_token.deref_mut()
                        } else {
                            stable_trade_token.deref_mut()
                        },
                        fee,
                        &UserTokenUpdateReason::SettleFee,
                    )?;
                }
                drop(base_token_pool);
                drop(stable_pool);
                drop(market);
                drop(trade_token);
                drop(stable_trade_token);
                msg!("===========INCREASE start1111");
                //increase position
                increase_position(
                    &user_order.symbol,
                    user,
                    pool_map,
                    program_id,
                    &user_order,
                    order_margin,
                    order_margin_from_balance,
                    execute_price,
                    margin_token_price,
                    fee,
                    oracle_map,
                    trade_token_map,
                    market_map,
                    state_account,
                )?;
                Ok(())
            }
        },

        PositionSide::DECREASE => {
            {
                //decrease
                let pool_vault = base_token_pool.pool_vault_key;
                let stable_pool_vault = stable_pool.pool_vault_key;
                let token_vault =
                    match use_base_token(&user_order.position_side, &user_order.order_side)? {
                        true => vault_map.get_account(&trade_token.vault_key)?,
                        false => vault_map.get_account(&stable_trade_token.vault_key)?,
                    };
                let position = user.get_user_position_ref(&position_key)?;
                let position_side = position.is_long;
                if position.position_size == 0u128 {
                    return Err(BumpErrorCode::CouldNotFindUserPosition.into());
                }
                if position_side == is_long {
                    return Err(BumpErrorCode::InvalidParam.into());
                }
                msg!("===========handle_execute_order start2222");
                decrease_position(
                    DecreasePositionParams {
                        order_id: user_order.order_id,
                        is_liquidation: false,
                        is_portfolio_margin: false,
                        margin_token: user_order.margin_mint_key,
                        decrease_size: if position.position_size < user_order.order_size {
                            position.position_size
                        } else {
                            user_order.order_size
                        },
                        execute_price,
                    },
                    user,
                    market.deref_mut(),
                    base_token_pool.deref_mut(),
                    stable_pool.deref_mut(),
                    state_account,
                    user_token_account,
                    match use_base_token(&user_order.position_side, &user_order.order_side)? {
                        true => vault_map.get_account(&pool_vault)?,
                        false => vault_map.get_account(&stable_pool_vault)?,
                    },
                    match use_base_token(&user_order.position_side, &user_order.order_side)? {
                        true => trade_token.deref_mut(),
                        false => stable_trade_token.deref_mut(),
                    },
                    token_vault,
                    bump_signer,
                    token_program,
                    oracle_map,
                    &position_key,
                )?;
                Ok(())
            }
        },
    }?;
    //delete order
    user.delete_order(user_order.order_id)?;
    Ok(())
}

pub fn use_base_token(position_side: &PositionSide, order_side: &OrderSide) -> BumpResult<bool> {
    match (position_side, order_side) {
        (PositionSide::INCREASE, OrderSide::LONG) => Ok(true),
        (PositionSide::INCREASE, OrderSide::SHORT) => Ok(false),
        (PositionSide::DECREASE, OrderSide::LONG) => Ok(false),
        (PositionSide::DECREASE, OrderSide::SHORT) => Ok(true),
        _ => return Err(BumpErrorCode::InvalidParam),
    }
}

#[track_caller]
fn execute_increase_order_margin(
    order: &UserOrder,
    margin_token: &Pubkey,
    decimals: u16,
    user: &mut User,
    margin_token_price: u128,
    oracle_map: &mut OracleMap,
    trade_token_map: &TradeTokenMap,
    state: &Account<State>,
) -> BumpResult<(u128, u128)> {
    let order_margin;
    let order_margin_from_balance;
    if order.is_portfolio_margin {
        let available_value = user.get_available_value(trade_token_map, oracle_map)?;
        let order_margin_temp;
        if available_value < 0i128 {
            let fix_order_margin_in_usd =
                order.order_margin.cast::<i128>()?.safe_add(available_value)?;
            validate!(fix_order_margin_in_usd > 0i128, BumpErrorCode::BalanceNotEnough)?;
            user.sub_order_hold_in_usd(order.order_margin)?;
            order_margin_temp = fix_order_margin_in_usd.abs().cast::<u128>()?;
        } else {
            order_margin_temp = order.order_margin;
            user.sub_order_hold_in_usd(order.order_margin)?;
        }
        order_margin = calculator::usd_to_token_u(order_margin_temp, decimals, margin_token_price)?;
        order_margin_from_balance = user.use_token(margin_token, order_margin, false)?;
    } else {
        let order_margin_in_usd =
            calculator::token_to_usd_u(order.order_margin, decimals, margin_token_price)?;
        validate!(
            order_margin_in_usd >= state.minimum_order_margin_usd,
            BumpErrorCode::OrderMarginUSDTooSmall
        )?;
        order_margin = order.order_margin;
        order_margin_from_balance = order.order_margin;
    }

    Ok((order_margin, order_margin_from_balance))
}

#[track_caller]
fn get_execution_price(index_price: u128, order: &UserOrder) -> BumpResult<u128> {
    if order.order_type.eq(&OrderType::MARKET) {
        if order.acceptable_price > 0 {
            if order.order_side.eq(&OrderSide::LONG) && index_price >= order.acceptable_price {
                return Err(BumpErrorCode::PriceIsNotAllowed);
            }
            if order.order_side.eq(&OrderSide::SHORT) && index_price <= order.acceptable_price {
                return Err(BumpErrorCode::PriceIsNotAllowed);
            }
        }
        return Ok(index_price);
    }

    let long = OrderSide::LONG == order.order_side;
    if order.order_type.eq(&OrderType::LIMIT)
        || (order.order_type.eq(&OrderType::STOP) && order.stop_type.eq(&StopType::TakeProfit))
    {
        if (long && order.trigger_price >= index_price)
            || (!long && order.trigger_price <= index_price)
        {
            return Ok(index_price);
        }
        return Err(BumpErrorCode::PriceIsNotAllowed);
    }
    if order.order_type.eq(&OrderType::STOP)
        && order.stop_type.eq(&StopType::StopLoss)
        && ((long && order.trigger_price <= index_price)
            || (!long && order.trigger_price >= index_price))
    {
        return Ok(index_price);
    }

    Err(BumpErrorCode::PriceIsNotAllowed)
}

#[track_caller]
fn validate_execute_order(order: &UserOrder, market: &Market) -> BumpResult<()> {
    if order.leverage > market.config.maximum_leverage
        || order.leverage < market.config.minimum_leverage
    {
        return Err(BumpErrorCode::LeverageIsNotAllowed);
    }
    Ok(())
}

#[track_caller]
pub fn update_funding_fee(
    position: &mut UserPosition,
    market: &mut Market,
    pool: &mut Pool,
    token_price: u128,
    token: &TradeToken,
) -> BumpResult<()> {
    let market_funding_fee_per_size = if position.is_long {
        market.funding_fee.long_funding_fee_amount_per_size
    } else {
        market.funding_fee.short_funding_fee_amount_per_size
    };

    let realized_funding_fee_delta = calculator::mul_per_token_rate_i(
        position.position_size.cast::<i128>()?,
        market_funding_fee_per_size
            .cast::<i128>()?
            .safe_sub(position.open_funding_fee_amount_per_size.cast::<i128>()?)?, //10^5
    )?;
    let realized_funding_fee;
    if position.is_long {
        realized_funding_fee = realized_funding_fee_delta;
        position.add_realized_funding_fee(realized_funding_fee_delta)?;
        position.add_realized_funding_fee_in_usd(calculator::token_to_usd_i(
            realized_funding_fee_delta,
            token.decimals,
            token_price,
        )?)?;
    } else {
        realized_funding_fee =
            calculator::usd_to_token_i(realized_funding_fee_delta, token.decimals, token_price)?;
        msg!("======update_funding_fee, realized_funding_fee_delta:{}", realized_funding_fee_delta);
        msg!("======update_funding_fee, realized_funding_fee:{}", realized_funding_fee);
        msg!("======update_funding_fee, token.decimals:{}", token.decimals);
        msg!("======update_funding_fee, token_price:{}", token_price);
        position.add_realized_funding_fee(realized_funding_fee)?;
        position.add_realized_funding_fee_in_usd(realized_funding_fee_delta)?
    }

    position.set_open_funding_fee_amount_per_size(market_funding_fee_per_size)?;
    market.update_market_total_funding_fee(realized_funding_fee, position.is_long)?;
    pool.update_pool_funding_fee(realized_funding_fee_delta)?;
    Ok(())
}

#[track_caller]
pub fn update_borrowing_fee(
    position: &mut UserPosition,
    pool: &mut Pool,
    token_price: u128,
    token: &TradeToken,
) -> BumpResult<()> {
    let realized_borrowing_fee = calculator::mul_per_token_rate_u(
        calculator::mul_rate_u(
            position.initial_margin,
            (position.leverage as u128).safe_sub(1u128.safe_mul(RATE_PRECISION)?)?,
        )?,
        pool.borrowing_fee
            .cumulative_borrowing_fee_per_token
            .safe_sub(position.open_borrowing_fee_per_token)?,
    )?;

    position.add_realized_borrowing_fee(realized_borrowing_fee)?;
    position.add_realized_borrowing_fee_in_usd(calculator::token_to_usd_u(
        realized_borrowing_fee,
        token.decimals,
        token_price,
    )?)?;
    position
        .set_open_borrowing_fee_per_token(pool.borrowing_fee.cumulative_borrowing_fee_per_token)?;
    pool.borrowing_fee.update_total_borrowing_fee(0u128, true, realized_borrowing_fee, true)?;
    Ok(())
}

#[track_caller]
pub fn decrease_position<'info>(
    params: DecreasePositionParams,
    user: &mut User,
    market: &mut Market,
    stake_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state_account: &Account<'info, State>,
    user_token_account: Option<&Account<'info, TokenAccount>>,
    pool_vault_account: &Account<'info, TokenAccount>,
    trade_token: &mut TradeToken,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    oracle_map: &mut OracleMap,
    position_key: &Pubkey,
) -> BumpResult<()> {
    let (is_long, position_deletion, pre_position, response) = {
        let position = user.get_user_position_mut_ref(position_key)?;
        let pre_position = *position;
        let position_un_pnl_usd = position.get_position_un_pnl_usd(params.execute_price)?;
        let margin_mint_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
        update_borrowing_fee(
            position,
            if position.is_long { stake_token_pool } else { stable_pool },
            params.execute_price,
            trade_token,
        )?;
        update_funding_fee(
            position,
            market,
            if position.is_long { stake_token_pool } else { stable_pool },
            params.execute_price,
            trade_token,
        )?;

        let response = calculate_decrease_position(
            params.decrease_size,
            params.is_liquidation,
            params.is_portfolio_margin,
            position_un_pnl_usd,
            margin_mint_token_price,
            market,
            state_account,
            &trade_token,
            position,
        )?;
        msg!("========decrease_position, calculate_decrease_position response:{:?}", response);

        if response.settle_margin < 0i128 && !params.is_liquidation && !position.is_portfolio_margin
        {
            return Err(BumpErrorCode::PositionShouldBeLiquidation);
        }
        let position_deletion = if params.decrease_size != position.position_size {
            update_decrease_position(position, params.decrease_size, &response)?;
            false
        } else {
            true
        };
        (position.is_long, position_deletion, pre_position, response)
    };

    if position_deletion {
        user.delete_position(position_key)?;
    }
    //collect fee
    collect_decrease_fee(
        stake_token_pool,
        stable_pool,
        market,
        pre_position.is_portfolio_margin,
        response.settle_close_fee,
        response.settle_borrowing_fee,
        response.settle_funding_fee,
        &pre_position.margin_mint_key,
        params.decrease_size,
        is_long,
        pre_position.entry_price,
        trade_token.decimals,
    )?;
    settle(
        &response,
        user,
        market,
        stake_token_pool,
        stable_pool,
        state_account,
        user_token_account,
        pool_vault_account,
        trade_token,
        trade_token_vault_account,
        bump_signer,
        token_program,
        &pre_position,
    )?;

    //cancel stop order
    user.cancel_stop_orders(
        params.order_id,
        pre_position.symbol,
        &pre_position.margin_mint_key,
        pre_position.is_portfolio_margin,
    )?;

    //add insurance fund

    if params.is_liquidation {
        add_insurance_fund(
            market,
            state_account,
            trade_token,
            &response,
            if pre_position.is_long { stake_token_pool } else { stable_pool },
            &pre_position,
        )?;
    }

    Ok(())
}

#[track_caller]
fn collect_decrease_fee(
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    market: &mut Market,
    is_portfolio_margin: bool,
    settle_close_fee: u128,
    settle_borrowing_fee: u128,
    settle_funding_fee: i128,
    margin_token: &Pubkey,
    decrease_size: u128,
    is_long: bool,
    entry_price: u128,
    token_decimal: u16,
) -> BumpResult {
    fee_processor::collect_close_position_fee(
        if is_long { base_token_pool } else { stable_pool },
        settle_close_fee,
        is_portfolio_margin,
    )?;
    fee_processor::collect_borrowing_fee(
        if is_long { base_token_pool } else { stable_pool },
        settle_borrowing_fee,
        is_portfolio_margin,
    )?;

    if is_long { base_token_pool } else { stable_pool }.borrowing_fee.update_total_borrowing_fee(
        settle_borrowing_fee,
        true,
        settle_borrowing_fee,
        false,
    )?;
    market.update_market_total_funding_fee(settle_funding_fee, is_long)?;
    market.update_oi(
        false,
        UpdateOIParams {
            margin_token: *margin_token,
            size: decrease_size,
            is_long,
            entry_price,
            token_decimal,
        },
    )?;
    Ok(())
}

#[track_caller]
fn update_decrease_position(
    position: &mut UserPosition,
    decrease_size: u128,
    response: &UpdateDecreaseResponse,
) -> BumpResult {
    let pre_position = *position;
    position.sub_position_size(decrease_size)?;
    position.sub_initial_margin(response.decrease_margin)?;
    position.sub_initial_margin_usd(response.decrease_margin_in_usd)?;
    position
        .sub_initial_margin_usd_from_portfolio(response.decrease_margin_in_usd_from_portfolio)?;
    position.sub_hold_pool_amount(response.un_hold_pool_amount)?;
    position.add_realized_pnl(response.user_realized_pnl)?;
    position.sub_realized_borrowing_fee(response.settle_borrowing_fee)?;
    position.sub_realized_borrowing_fee_usd(response.settle_borrowing_fee_in_usd)?;
    position.sub_realized_funding_fee(response.settle_funding_fee)?;
    position.sub_realized_funding_fee_usd(response.settle_funding_fee_in_usd)?;
    position.sub_close_fee_usd(response.settle_close_fee_in_usd)?;
    position.set_last_update(calculator::current_time())?;
    emit!(UpdateUserPositionEvent { pre_position, position: *position });
    Ok(())
}

fn settle<'info>(
    response: &UpdateDecreaseResponse,
    user: &mut User,
    market: &mut Market,
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state_account: &Account<'info, State>,
    user_token_account: Option<&Account<'info, TokenAccount>>,
    pool_vault_account: &Account<'info, TokenAccount>,
    trade_token: &mut TradeToken,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    position: &UserPosition,
) -> BumpResult<()> {
    fee_processor::settle_funding_fee(
        if position.is_long { base_token_pool } else { stable_pool },
        response.settle_funding_fee,
        position.is_portfolio_margin,
    )?;
    let mut add_liability = 0u128;
    if position.is_portfolio_margin {
        add_liability = settle_cross(
            response,
            user,
            state_account,
            pool_vault_account,
            trade_token,
            trade_token_vault_account,
            bump_signer,
            token_program,
            position,
        )?;
        let repay_amount = user
            .repay_liability(&position.margin_mint_key, UserTokenUpdateReason::DecreasePosition)?;
        trade_token.sub_total_liability(repay_amount)?;
    } else {
        settle_isolate(
            response,
            state_account,
            user_token_account.ok_or(BumpErrorCode::InvalidParam)?,
            pool_vault_account,
            bump_signer,
            token_program,
        )?;
    }
    if position.is_long {
        base_token_pool.update_pnl_and_un_hold_pool_amount(
            market,
            response.un_hold_pool_amount,
            response.pool_pnl_token,
            add_liability,
        )?;
    } else {
        stable_pool.update_pnl_and_un_hold_pool_amount(
            market,
            response.un_hold_pool_amount,
            response.pool_pnl_token,
            add_liability,
        )?;
    }
    Ok(())
}

#[track_caller]
fn settle_cross<'info>(
    response: &UpdateDecreaseResponse,
    user: &mut User,
    state_account: &Account<'info, State>,
    pool_vault_account: &Account<'info, TokenAccount>,
    trade_token: &mut TradeToken,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    position: &UserPosition,
) -> BumpResult<u128> {
    let mut add_liability = 0u128;
    //record pay fee
    if response.settle_fee > 0i128 {
        add_liability = user.sub_token_with_liability(
            &position.margin_mint_key,
            trade_token,
            response.settle_fee.abs().cast::<u128>()?,
            &UserTokenUpdateReason::SettleFee,
        )?;
    } else {
        user.add_user_token_amount(
            &position.margin_mint_key,
            response.settle_fee.abs().cast::<u128>()?,
            &UserTokenUpdateReason::SettleFee,
        )?;
    }

    // release token
    user.un_use_token(&position.margin_mint_key, response.decrease_margin)?;

    //deal user pnl
    if response.user_realized_pnl_token.safe_add(response.settle_fee)? > 0i128 {
        user.add_user_token_amount(
            &position.margin_mint_key,
            response.user_realized_pnl_token.safe_add(response.settle_fee)?.abs().cast::<u128>()?,
            &UserTokenUpdateReason::SettlePnl,
        )?;
    } else {
        add_liability = add_liability.safe_add(user.sub_token_with_liability(
            &position.margin_mint_key,
            trade_token,
            response.user_realized_pnl_token.safe_add(response.settle_fee)?.abs().cast::<u128>()?,
            &UserTokenUpdateReason::SettlePnl,
        )?)?;
    }

    if response.pool_pnl_token < 0i128 {
        token::send_from_program_vault(
            token_program,
            pool_vault_account,
            trade_token_vault_account,
            bump_signer,
            state_account.bump_signer_nonce,
            response.pool_pnl_token.abs().cast::<u128>()?,
        )
        .map_err(|_e| BumpErrorCode::TransferFailed)?;
        trade_token.add_total_amount(response.pool_pnl_token.abs().cast::<u128>()?)?;
    } else if response.pool_pnl_token.safe_sub(add_liability.cast::<i128>()?)? > 0i128 {
        token::send_from_program_vault(
            token_program,
            trade_token_vault_account,
            pool_vault_account,
            bump_signer,
            state_account.bump_signer_nonce,
            response.pool_pnl_token.safe_sub(add_liability.cast::<i128>()?)?.cast::<u128>()?,
        )
        .map_err(|_e| BumpErrorCode::TransferFailed)?;

        trade_token.sub_total_amount(
            response.pool_pnl_token.safe_sub(add_liability.cast::<i128>()?)?.cast::<u128>()?,
        )?;
    }

    if !response.is_liquidation {
        let change_token_amount = response
            .decrease_margin_in_usd_from_portfolio
            .safe_mul(position.initial_margin)?
            .safe_div(position.initial_margin_usd)?
            .cast::<i128>()?
            .safe_add(response.settle_margin.cast::<i128>()?)?
            .safe_sub(response.decrease_margin.cast::<i128>()?)?;

        user.update_all_position_from_portfolio_margin(
            change_token_amount,
            &position.margin_mint_key,
        )?;
    }
    Ok(add_liability)
}

fn settle_isolate<'info>(
    response: &UpdateDecreaseResponse,
    state_account: &Account<'info, State>,
    user_token_account: &Account<'info, TokenAccount>,
    pool_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
) -> BumpResult<()> {
    if response.is_liquidation {
        return Ok(());
    }
    token::send_from_program_vault(
        token_program,
        pool_vault_account,
        user_token_account,
        bump_signer,
        state_account.bump_signer_nonce,
        response.settle_margin.abs().cast::<u128>()?,
    )
    .map_err(|_e| BumpErrorCode::TransferFailed)?;
    Ok(())
}

#[track_caller]
fn add_insurance_fund(
    market: &Market,
    state: &State,
    trade_token: &TradeToken,
    response: &UpdateDecreaseResponse,
    pool: &mut Pool,
    position: &UserPosition,
) -> BumpResult<()> {
    if position.is_portfolio_margin {
        pool.add_insurance_fund(calculator::usd_to_token_u(
            position.get_position_mm(market, state)?,
            trade_token.decimals,
            response.margin_token_price,
        )?)?
    }

    let add_funds = if response.settle_fee >= 0i128 {
        if response.decrease_margin
            > (response.settle_fee.safe_add(response.pool_pnl_token)?.abs().cast::<u128>()?)
        {
            response.decrease_margin.safe_sub(
                response
                    .settle_fee
                    .safe_add(response.pool_pnl_token.abs().cast::<i128>()?)?
                    .cast::<u128>()?,
            )?
        } else {
            0u128
        }
    } else {
        response
            .decrease_margin
            .safe_add(response.settle_fee.abs().cast::<u128>()?)?
            .safe_sub(response.pool_pnl_token.abs().cast::<u128>()?)?
    };
    pool.add_insurance_fund(add_funds)
}

#[track_caller]
pub fn execute_reduce_position_margin(
    params: &UpdatePositionMarginParams,
    need_update_leverage: bool,
    base_trade_token: &TradeToken,
    stable_trade_token: &TradeToken,
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    market: &Market,
    state: &State,
    position: &mut UserPosition,
    oracle_map: &mut OracleMap,
    trade_token_map: &TradeTokenMap,
    market_map: &MarketMap,
) -> BumpResult<u128> {
    let max_reduce_margin_in_usd = position.initial_margin_usd.safe_sub(
        calculator::div_rate_u(position.position_size, market.config.maximum_leverage as u128)?
            .max(state.minimum_order_margin_usd),
    )?;
    validate!(
        max_reduce_margin_in_usd > params.update_margin_amount,
        BumpErrorCode::AmountNotEnough
    )?;
    let user_key = position.user_key;
    let pre_position = *position;
    let reduce_margin_amount = calculator::usd_to_token_u(
        params.update_margin_amount,
        if position.is_long { base_trade_token.decimals } else { stable_trade_token.decimals },
        position.entry_price,
    )?;

    if position.is_portfolio_margin
        && position.initial_margin_usd.safe_sub(position.initial_margin_usd_from_portfolio)?
            < reduce_margin_amount
    {
        position.sub_initial_margin_usd_from_portfolio(
            reduce_margin_amount
                .safe_sub(
                    position
                        .initial_margin_usd
                        .safe_sub(position.initial_margin_usd_from_portfolio)?,
                )?
                .max(0u128),
        )?;
    }
    position.sub_initial_margin(reduce_margin_amount)?;
    position.sub_initial_margin_usd(params.update_margin_amount)?;

    if need_update_leverage {
        position.set_leverage(calculator::div_rate_u(
            position.position_size,
            position.initial_margin_usd,
        )? as u32)?; //TODO: Precision loss?
    }
    if !position.is_portfolio_margin {
        position.set_initial_margin_usd_from_portfolio(position.initial_margin_usd)?;
    }
    position.add_hold_pool_amount(reduce_margin_amount)?;

    //lock pool amount
    if position.is_long {
        base_token_pool.hold_pool_amount(
            reduce_margin_amount,
            market_map,
            oracle_map,
            trade_token_map,
            market.config.max_pool_liquidity_share_rate,
        )?
    } else {
        stable_pool.hold_pool_amount(
            reduce_margin_amount,
            market_map,
            oracle_map,
            trade_token_map,
            market.config.max_pool_liquidity_share_rate,
        )?
    }
    emit!(AddOrDecreaseMarginEvent { user_key, position: *position, pre_position, is_add: false });
    Ok(reduce_margin_amount)
}

#[track_caller]
pub fn calculate_decrease_position(
    decrease_size: u128,
    is_liquidation: bool,
    is_portfolio_margin: bool,
    pnl: i128,
    margin_mint_token_price: u128,
    market: &Market,
    state: &State,
    trade_token: &TradeToken,
    position: &UserPosition,
) -> BumpResult<UpdateDecreaseResponse> {
    let mut response = UpdateDecreaseResponse::default();
    response.is_liquidation = is_liquidation;
    response.margin_token_price = margin_mint_token_price;

    let (settle_borrowing_fee, settle_borrowing_fee_in_usd) =
        cal_decrease_borrowing_fee(position, decrease_size)?;
    let (settle_funding_fee, settle_funding_fee_in_usd) =
        cal_decrease_funding_fee(position, decrease_size)?;
    let (settle_close_fee, settle_close_fee_in_usd) = cal_decrease_close_fee(
        decrease_size,
        trade_token,
        margin_mint_token_price,
        market.config.close_fee_rate,
        position,
    )?;

    response.settle_borrowing_fee = settle_borrowing_fee;
    response.settle_borrowing_fee_in_usd = settle_borrowing_fee_in_usd;
    response.settle_funding_fee = settle_funding_fee;
    response.settle_funding_fee_in_usd = settle_funding_fee_in_usd;
    response.settle_close_fee = settle_close_fee;
    response.settle_close_fee_in_usd = settle_close_fee_in_usd;
    response.settle_fee = response
        .settle_close_fee
        .cast::<i128>()?
        .safe_add(response.settle_funding_fee)?
        .safe_add(response.settle_borrowing_fee.cast::<i128>()?)?
        .cast::<i128>()?;

    response.decrease_margin =
        position.initial_margin.safe_mul(decrease_size)?.safe_div(position.position_size)?;
    response.decrease_margin_in_usd =
        position.initial_margin_usd.safe_mul(decrease_size)?.safe_div(position.position_size)?;
    response.un_hold_pool_amount =
        position.hold_pool_amount.safe_mul(decrease_size)?.safe_div(position.position_size)?;

    if position.position_size == decrease_size && is_liquidation {
        response.settle_margin = if is_portfolio_margin {
            //(initial_margin_usd - pos_fee_usd + pnl - mm) * decimals / price
            calculator::usd_to_token_i(
                position
                    .initial_margin_usd
                    .cast::<i128>()?
                    .safe_sub(get_pos_fee_in_usd(
                        settle_funding_fee_in_usd,
                        settle_borrowing_fee_in_usd,
                        settle_close_fee_in_usd,
                    )?)?
                    .safe_add(pnl)?
                    .safe_sub(position.get_position_mm(market, state)?.cast::<i128>()?)?,
                trade_token.decimals,
                margin_mint_token_price,
            )?
        } else {
            0i128
        };
    } else {
        //(initial_margin_usd - pos_fee + pnl) * decrease_percent * decimals / price
        response.settle_margin = calculator::usd_to_token_i(
            position
                .initial_margin_usd
                .cast::<i128>()?
                .safe_add(pnl)?
                .safe_mul(decrease_size.cast()?)?
                .safe_div(position.position_size.cast()?)?
                .safe_sub(get_pos_fee_in_usd(
                    settle_funding_fee_in_usd,
                    settle_borrowing_fee_in_usd,
                    settle_close_fee_in_usd,
                )?)?,
            trade_token.decimals,
            margin_mint_token_price,
        )?;
    }

    response.user_realized_pnl_token =
        response.settle_margin.safe_sub(response.decrease_margin.cast::<i128>()?)?;
    //decrease_margin - (initial_margin_usd + pnl) * decrease_percent * decimals / price
    response.pool_pnl_token = response
        .decrease_margin
        .cast::<i128>()?
        .safe_sub(response.settle_margin)?
        .safe_sub(response.settle_fee)?;
    //(settle_margin - decrease_margin) * price / decimal
    response.user_realized_pnl = calculator::token_to_usd_i(
        response.user_realized_pnl_token,
        trade_token.decimals,
        margin_mint_token_price,
    )?;
    response.decrease_margin_in_usd_from_portfolio = if calculator::add_u128(
        response.decrease_margin_in_usd,
        position.initial_margin_usd_from_portfolio,
    )? > position.initial_margin_usd
    {
        calculator::sub_u128(
            calculator::add_u128(
                response.decrease_margin_in_usd,
                position.initial_margin_usd_from_portfolio,
            )?,
            position.initial_margin_usd,
        )?
    } else {
        0u128
    };
    Ok(response)
}

#[track_caller]
fn get_pos_fee_in_usd(
    funding_fee_in_usd: i128,
    borrowing_fee_in_usd: u128,
    close_fee_in_usd: u128,
) -> BumpResult<i128> {
    let result = funding_fee_in_usd
        .safe_add(borrowing_fee_in_usd.cast::<i128>()?)?
        .safe_add(close_fee_in_usd.cast::<i128>()?)?;
    Ok(result)
}

#[track_caller]
fn cal_decrease_borrowing_fee(
    position: &UserPosition,
    decrease_size: u128,
) -> BumpResult<(u128, u128)> {
    if position.position_size == decrease_size {
        return Ok((position.realized_borrowing_fee, position.realized_borrowing_fee_in_usd));
    }
    Ok((
        position
            .realized_borrowing_fee
            .safe_mul(decrease_size)?
            .safe_div(position.position_size)?,
        position
            .realized_borrowing_fee_in_usd
            .safe_mul(decrease_size)?
            .safe_div(position.position_size)?,
    ))
}

#[track_caller]
fn cal_decrease_funding_fee(
    position: &UserPosition,
    decrease_size: u128,
) -> BumpResult<(i128, i128)> {
    if position.position_size == decrease_size {
        return Ok((position.realized_funding_fee, position.realized_funding_fee_in_usd));
    }
    Ok((
        position
            .realized_funding_fee
            .safe_mul(decrease_size.cast()?)?
            .safe_div(position.position_size.cast()?)?,
        position
            .realized_funding_fee_in_usd
            .safe_mul(decrease_size.cast()?)?
            .safe_div(position.position_size.cast()?)?,
    ))
}

#[track_caller]
fn cal_decrease_close_fee(
    decrease_size: u128,
    trade_token: &TradeToken,
    token_price: u128,
    close_fee_rate: u128,
    position: &UserPosition,
) -> BumpResult<(u128, u128)> {
    if position.position_size == decrease_size {
        return Ok((
            calculator::usd_to_token_u(
                position.close_fee_in_usd,
                trade_token.decimals,
                token_price,
            )
            .unwrap(),
            position.close_fee_in_usd,
        ));
    }

    let mut close_fee_in_usd = calculator::mul_rate_u(decrease_size, close_fee_rate).unwrap();
    if close_fee_in_usd > position.close_fee_in_usd {
        close_fee_in_usd = position.close_fee_in_usd;
    }

    Ok((
        calculator::usd_to_token_u(close_fee_in_usd, trade_token.decimals, token_price)?
            .safe_mul(decrease_size)?
            .safe_div(position.position_size)?,
        close_fee_in_usd.safe_mul(decrease_size)?.safe_div(position.position_size)?,
    ))
}

#[track_caller]
pub fn execute_add_position_margin(
    params: &UpdatePositionMarginParams,
    trade_token: &TradeToken,
    pool: &mut Pool,
    market: &mut Market,
    position: &mut UserPosition,
) -> BumpResult<()> {
    let user_key = position.user_key;
    validate!(
        params.update_margin_amount
            < calculator::usd_to_token_u(
                position.position_size.safe_sub(position.initial_margin_usd)?,
                trade_token.decimals,
                position.entry_price
            )?,
        BumpErrorCode::AmountNotEnough
    )?;

    let mut add_or_decrease_margin_event = AddOrDecreaseMarginEvent {
        user_key,
        position: Default::default(),
        pre_position: *position,
        is_add: true,
    };

    position.add_initial_margin(params.update_margin_amount)?;
    if position.is_portfolio_margin {
        position.set_initial_margin_usd(calculator::div_rate_u(
            position.position_size,
            position.leverage as u128,
        )?)?;
        position.add_initial_margin_usd_from_portfolio(params.add_initial_margin_from_portfolio)?;
    } else {
        position.add_initial_margin_usd(calculator::token_to_usd_u(
            params.update_margin_amount,
            trade_token.decimals,
            position.margin_token_entry_price,
        )?)?;
        position.set_leverage(calculator::div_rate_u(
            position.position_size,
            position.initial_margin_usd,
        )? as u32)?; // TODO: Precision loss?
        position.set_initial_margin_usd_from_portfolio(position.initial_margin)?;
    }

    let sub_amount = params.update_margin_amount.min(position.hold_pool_amount);
    position.sub_hold_pool_amount(sub_amount)?;
    pool.update_pnl_and_un_hold_pool_amount(market, sub_amount, 0i128, 0u128)?;

    add_or_decrease_margin_event.position = *position;
    emit!(add_or_decrease_margin_event);
    Ok(())
}

#[track_caller]
pub fn increase_position(
    symbol: &[u8; 32],
    user: &mut User,
    pool_map: &PoolMap,
    program_id: &Pubkey,
    order: &UserOrder,
    order_margin: u128,
    order_margin_from_balance: u128,
    execute_price: u128,
    margin_token_price: u128,
    fee: u128,
    oracle_map: &mut OracleMap,
    trade_token_map: &TradeTokenMap,
    market_map: &MarketMap,
    state: &Account<State>,
) -> BumpResult<()> {
    msg!("===========increase_position start");
    let mut market = market_map.get_mut_ref(symbol)?;
    let mut base_token_pool = pool_map.get_mut_ref(&market.pool_key)?;
    let mut stable_pool = pool_map.get_mut_ref(&market.stable_pool_key)?;
    let trade_token = trade_token_map.get_trade_token_by_mint_ref_mut(&market.pool_mint_key)?;
    let stable_trade_token =
        trade_token_map.get_trade_token_by_mint_ref_mut(&market.stable_pool_mint_key)?;
    let position_key = pda::generate_position_key(
        &user.key,
        market.symbol,
        order.is_portfolio_margin,
        program_id,
    )?;

    let position_index = user
        .get_user_position_index(&position_key)
        .or_else(|_| user.add_user_position(&position_key))?;
    let position = &mut user.positions[position_index];

    let is_long = order.order_side.eq(&OrderSide::LONG);
    if position.position_size != 0u128 && position.leverage != order.leverage {
        return Err(BumpErrorCode::LeverageIsNotAllowed.into());
    }
    let increase_margin = calculator::sub_u128(order_margin, fee)?;
    let increase_margin_from_balance = if order_margin_from_balance > fee {
        calculator::sub_u128(order_margin_from_balance, fee)?
    } else {
        0u128
    };
    let decimal = if is_long { trade_token.decimals } else { stable_trade_token.decimals };
    let increase_size = calculator::token_to_usd_u(
        calculator::mul_rate_u(increase_margin, order.leverage as u128)?,
        decimal,
        margin_token_price,
    )?;
    let increase_hold = calculator::mul_rate_u(
        increase_margin,
        calculator::sub_u128(order.leverage as u128, 1u128.safe_mul(RATE_PRECISION)?)?,
    )?;

    if position.position_size == 0u128 {
        //new position
        msg!("===========increase_position start1111");
        position.set_index_mint(market.index_mint_oracle)?;
        position.set_symbol(order.symbol)?;
        position.set_margin_mint(order.margin_mint_key)?;
        position.set_leverage(order.leverage)?;
        position.set_is_long(order.order_side.eq(&OrderSide::LONG))?;
        position.set_portfolio_margin(order.is_portfolio_margin)?;
        position.set_margin_mint(order.margin_mint_key)?;
        position.set_entry_price(execute_price)?;
        position.set_margin_token_entry_price(margin_token_price)?;
        position.set_user_token_account(order.user_token_account)?;
        position.add_initial_margin(increase_margin)?;
        position.add_initial_margin_usd(calculator::token_to_usd_u(
            increase_margin,
            decimal,
            margin_token_price,
        )?)?;
        position.add_initial_margin_usd_from_portfolio(calculator::token_to_usd_u(
            increase_margin_from_balance,
            decimal,
            margin_token_price,
        )?)?;
        position.add_close_fee_in_usd(calculator::mul_rate_u(
            increase_size,
            market.config.close_fee_rate,
        )?)?;
        position.add_open_fee(fee)?;
        position.add_open_fee_in_usd(calculator::token_to_usd_u(
            fee,
            decimal,
            margin_token_price,
        )?)?;
        position.add_position_size(increase_size)?;
        position.set_leverage(order.leverage)?;
        position.set_realized_pnl(-calculator::token_to_usd_i(
            fee.cast::<i128>()?,
            decimal,
            margin_token_price,
        )?)?;
        position.set_open_borrowing_fee_per_token(if order.order_side.eq(&OrderSide::LONG) {
            base_token_pool.borrowing_fee.cumulative_borrowing_fee_per_token
        } else {
            stable_pool.borrowing_fee.cumulative_borrowing_fee_per_token
        })?;
        position.set_open_funding_fee_amount_per_size(if is_long {
            market.funding_fee.long_funding_fee_amount_per_size
        } else {
            market.funding_fee.short_funding_fee_amount_per_size
        })?;
        position.set_last_update(calculator::current_time())?;
        position.add_hold_pool_amount(increase_hold)?;
        position.set_mm_usd(position.get_position_mm(market.deref(), state)?)?;
        emit!(AddOrDeleteUserPositionEvent { position: position.clone(), is_add: true });
    } else {
        msg!("===========increase_position start222");
        validate!(is_long.eq(&position.is_long), BumpErrorCode::OnlyOneDirectionPositionIsAllowed)?;
        //increase position
        let pre_position = *position;
        update_borrowing_fee(
            position,
            if is_long { base_token_pool.deref_mut() } else { stable_pool.deref_mut() },
            margin_token_price,
            if is_long { &trade_token } else { &stable_trade_token },
        )?;
        update_funding_fee(
            position,
            market.deref_mut(),
            if is_long { base_token_pool.deref_mut() } else { stable_pool.deref_mut() },
            margin_token_price,
            if is_long { &trade_token } else { &stable_trade_token },
        )?;
        position.set_entry_price(calculator::compute_avg_entry_price(
            position.position_size,
            position.entry_price,
            increase_size,
            execute_price,
            market.config.tick_size,
            decimal,
            position.is_long,
        )?)?;

        position.set_margin_token_entry_price(calculator::compute_avg_entry_price(
            position.position_size,
            position.margin_token_entry_price,
            increase_size,
            margin_token_price,
            market.config.tick_size,
            decimal,
            position.is_long,
        )?)?;
        position.add_initial_margin(increase_margin)?;
        position.add_initial_margin_usd(calculator::token_to_usd_u(
            increase_margin,
            decimal,
            margin_token_price,
        )?)?;
        position.add_initial_margin_usd_from_portfolio(calculator::token_to_usd_u(
            increase_margin_from_balance,
            decimal,
            margin_token_price,
        )?)?;
        position.add_position_size(increase_size)?;
        position.add_close_fee_in_usd(calculator::mul_rate_u(
            increase_size,
            market.config.close_fee_rate,
        )?)?;
        position.add_open_fee(fee)?;
        position.add_open_fee_in_usd(calculator::token_to_usd_u(
            fee,
            decimal,
            margin_token_price,
        )?)?;
        position.add_realized_pnl(
            -calculator::token_to_usd_u(fee, decimal, margin_token_price)?.cast::<i128>()?,
        )?;
        position.set_last_update(calculator::current_time())?;
        position.add_hold_pool_amount(increase_hold)?;
        position.set_mm_usd(position.get_position_mm(market.deref(), state)?)?;
        emit!(UpdateUserPositionEvent { pre_position, position: position.clone() });
    }
    msg!("===========increase_position start333");
    // update market io
    market.update_oi(
        true,
        UpdateOIParams {
            margin_token: order.margin_mint_key,
            size: increase_size,
            is_long,
            entry_price: execute_price,
            token_decimal: decimal,
        },
    )?;

    msg!("===========increase_position start444");
    let max_pool_liquidity_share_rate = market.config.max_pool_liquidity_share_rate;
    drop(market);
    drop(trade_token);
    drop(stable_trade_token);
    //lock pool amount
    if is_long {
        base_token_pool.hold_pool_amount(
            increase_hold,
            market_map,
            oracle_map,
            trade_token_map,
            max_pool_liquidity_share_rate,
        )?;
    } else {
        stable_pool.hold_pool_amount(
            increase_hold,
            market_map,
            oracle_map,
            trade_token_map,
            max_pool_liquidity_share_rate,
        )?;
    }
    Ok(())
}

#[track_caller]
pub fn update_cross_leverage<'info>(
    params: UpdatePositionLeverageParams,
    position_key: &Pubkey,
    user: &mut User,
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state: &State,
    market: &mut Market,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
) -> BumpResult<()> {
    let position = *user.get_user_position_ref(position_key)?;
    let base_trade_token = trade_token_map.get_trade_token_by_mint_ref(&market.pool_mint_key)?;
    let stable_trade_token =
        trade_token_map.get_trade_token_by_mint_ref(&market.stable_pool_mint_key)?;

    if position.position_size != 0u128 {
        if position.leverage > params.leverage {
            let add_margin_amount;
            let (position_leverage, position_size, position_entry_price) = {
                let position = user.get_user_position_mut_ref(position_key)?;
                position.set_leverage(params.leverage)?;
                (position.leverage, position.position_size, position.margin_token_entry_price)
            };
            msg!("=========update_cross_leverage,available_amount, position_leverage:{}, position_size:{}, position_entry_price:{}",position_leverage,position_size,position_entry_price);
            let available_amount = user
                .get_user_token_ref(&if position.is_long {
                    base_trade_token.mint_key
                } else {
                    stable_trade_token.mint_key
                })?
                .get_token_available_amount()?;
            msg!("=========update_cross_leverage,available_amount: {}", available_amount);
            let new_initial_margin_in_usd =
                calculator::div_rate_u(position_size, position_leverage as u128)?;
            msg!(
                "=========update_cross_leverage,new_initial_margin_in_usd: {}",
                new_initial_margin_in_usd
            );
            let add_margin_in_usd = if new_initial_margin_in_usd > position.initial_margin_usd {
                new_initial_margin_in_usd.safe_sub(position.initial_margin_usd)?
            } else {
                0u128
            };
            msg!("=========update_cross_leverage,add_margin_in_usd: {}", add_margin_in_usd);
            let cross_available_value = user.get_available_value(trade_token_map, oracle_map)?;
            msg!("=========update_cross_leverage,cross_available_value: {}", cross_available_value);
            validate!(
                add_margin_in_usd.cast::<i128>()? < cross_available_value,
                BumpErrorCode::UserAvailableValueNotEnough.into()
            )?;

            add_margin_amount = calculator::usd_to_token_u(
                add_margin_in_usd,
                if position.is_long {
                    base_trade_token.decimals
                } else {
                    stable_trade_token.decimals
                },
                position_entry_price,
            )?;
            msg!("=========update_cross_leverage,add_margin_amount: {}", add_margin_amount);

            let add_initial_margin_from_portfolio = calculator::token_to_usd_u(
                add_margin_amount.min(available_amount),
                if position.is_long {
                    base_trade_token.decimals
                } else {
                    stable_trade_token.decimals
                },
                position_entry_price,
            )?;
            msg!(
                "=========update_cross_leverage,add_initial_margin_from_portfolio: {}",
                add_initial_margin_from_portfolio
            );

            user.use_token(
                &if position.is_long {
                    base_trade_token.mint_key
                } else {
                    stable_trade_token.mint_key
                },
                add_margin_amount,
                false,
            )?;

            let position = user.get_user_position_mut_ref(position_key)?;
            execute_add_position_margin(
                &UpdatePositionMarginParams {
                    position_key: *position_key,
                    is_add: true,
                    update_margin_amount: add_margin_amount,
                    add_initial_margin_from_portfolio,
                    ..UpdatePositionMarginParams::default()
                },
                if position.is_long { &base_trade_token } else { &stable_trade_token },
                if position.is_long { base_token_pool } else { stable_pool },
                market,
                position,
            )?;
        } else {
            let position = user.get_user_position_mut_ref(position_key)?;
            position.set_leverage(params.leverage)?;
            let reduce_margin = position.initial_margin_usd.safe_sub(calculator::div_rate_u(
                position.position_size,
                position.leverage as u128,
            )?)?;
            let reduce_margin_amount = execute_reduce_position_margin(
                &UpdatePositionMarginParams {
                    position_key: *position_key,
                    is_add: false,
                    update_margin_amount: reduce_margin,
                    add_initial_margin_from_portfolio: 0,
                    ..UpdatePositionMarginParams::default()
                },
                false,
                &base_trade_token,
                &stable_trade_token,
                base_token_pool,
                stable_pool,
                market,
                state,
                position,
                oracle_map,
                trade_token_map,
                market_map,
            )?;
            let margin_mint_key = position.margin_mint_key;
            user.un_use_token(&margin_mint_key, reduce_margin_amount)?;
        }
    }
    Ok(())
}

#[track_caller]
pub fn update_isolate_leverage<'info>(
    params: UpdatePositionLeverageParams,
    position_key: &Pubkey,
    user: &mut User,
    authority: &Signer<'info>,
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state: &State,
    market: &mut Market,
    user_token_account: &Account<'info, TokenAccount>,
    pool_vault: &Account<'info, TokenAccount>,
    stable_pool_vault: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    market_map: &MarketMap,
) -> BumpResult<()> {
    let position = *user.get_user_position_ref(position_key)?;
    let base_trade_token = trade_token_map.get_trade_token_by_mint_ref(&market.pool_mint_key)?;
    let stable_trade_token =
        trade_token_map.get_trade_token_by_mint_ref(&market.stable_pool_mint_key)?;

    if position.position_size != 0u128 {
        if position.leverage > params.leverage {
            let (position_leverage, position_size, position_entry_price) = {
                let position = user.get_user_position_mut_ref(position_key)?;
                position.set_leverage(params.leverage)?;
                (position.leverage, position.position_size, position.margin_token_entry_price)
            };
            let new_initial_margin_in_usd =
                calculator::div_rate_u(position_size, position_leverage as u128)?;
            let add_margin_in_usd = if new_initial_margin_in_usd > position.initial_margin_usd {
                new_initial_margin_in_usd.safe_sub(position.initial_margin_usd)?
            } else {
                0u128
            };
            let add_margin_amount = calculator::usd_to_token_u(
                add_margin_in_usd,
                if position.is_long {
                    base_trade_token.decimals
                } else {
                    stable_trade_token.decimals
                },
                position_entry_price,
            )?;

            let position = user.get_user_position_mut_ref(position_key)?;
            execute_add_position_margin(
                &UpdatePositionMarginParams {
                    position_key: *position_key,
                    is_add: true,
                    update_margin_amount: add_margin_amount,
                    add_initial_margin_from_portfolio: add_margin_amount,
                    ..UpdatePositionMarginParams::default()
                },
                if position.is_long { &base_trade_token } else { &stable_trade_token },
                if position.is_long { base_token_pool } else { stable_pool },
                market,
                position,
            )?;
            token::receive(
                token_program,
                user_token_account,
                if position.is_long { pool_vault } else { stable_pool_vault },
                authority,
                add_margin_amount,
            )
            .map_err(|_e| BumpErrorCode::TransferFailed)?;
        } else {
            let position = user.get_user_position_mut_ref(position_key)?;
            position.set_leverage(params.leverage)?;
            let reduce_margin = position.initial_margin_usd.safe_sub(calculator::div_rate_u(
                position.position_size,
                position.leverage as u128,
            )?)?;
            let reduce_margin_amount = execute_reduce_position_margin(
                &UpdatePositionMarginParams {
                    position_key: *position_key,
                    is_add: false,
                    update_margin_amount: reduce_margin,
                    add_initial_margin_from_portfolio: 0,
                    ..UpdatePositionMarginParams::default()
                },
                false,
                &base_trade_token,
                &stable_trade_token,
                base_token_pool,
                stable_pool,
                market,
                state,
                position,
                oracle_map,
                trade_token_map,
                market_map,
            )?;
            token::send_from_program_vault(
                token_program,
                if position.is_long { pool_vault } else { stable_pool_vault },
                user_token_account,
                bump_signer,
                state.bump_signer_nonce,
                reduce_margin_amount,
            )
            .map_err(|_e| BumpErrorCode::TransferFailed)?
        }
    }
    Ok(())
}

#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct IncreasePositionParams {
    pub margin_token: Pubkey,
    pub increase_margin: u128,
    pub increase_margin_from_balance: u128,
    pub margin_token_price: u128,
    pub index_token_price: u128,
    pub leverage: u128,
    pub is_long: bool,
    pub is_portfolio_margin: bool,
    pub decimals: u16,
}

#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct DecreasePositionParams {
    pub order_id: u64,
    pub is_liquidation: bool,
    pub is_portfolio_margin: bool,
    pub margin_token: Pubkey,
    pub decrease_size: u128,
    pub execute_price: u128,
}

#[derive(Eq, Default, PartialEq, Debug)]
#[repr(C)]
pub struct UpdateDecreaseResponse {
    pub margin_token_price: u128,
    pub decrease_margin: u128,
    pub decrease_margin_in_usd: u128,
    pub un_hold_pool_amount: u128,
    pub settle_borrowing_fee: u128,
    pub settle_borrowing_fee_in_usd: u128,
    pub settle_funding_fee: i128,
    pub settle_funding_fee_in_usd: i128,
    pub settle_close_fee: u128,
    pub settle_close_fee_in_usd: u128,
    pub settle_fee: i128,
    pub settle_margin: i128,
    pub user_realized_pnl_token: i128,
    pub pool_pnl_token: i128,
    pub decrease_margin_in_usd_from_portfolio: u128,
    pub user_realized_pnl: i128,
    pub is_liquidation: bool,
}
