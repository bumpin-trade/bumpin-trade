use std::cell::Ref;

use anchor_lang::prelude::*;
use anchor_lang::prelude::{Account, AccountLoader, Program, Signer};
use anchor_lang::{emit, ToAccountInfo};
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{cal_utils, UpdatePositionLeverageParams, UpdatePositionMarginParams};
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::processor::market_processor::{MarketProcessor, UpdateOIParams};
use crate::processor::pool_processor::PoolProcessor;
use crate::state::bump_events::{
    AddOrDecreaseMarginEvent, AddOrDeleteUserPositionEvent, UpdateUserPositionEvent,
};
use crate::state::infrastructure::user_order::{OrderSide, UserOrder};
use crate::state::infrastructure::user_position::{PositionStatus, UserPosition};
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::{User, UserTokenUpdateReason};
use crate::utils::{pda, token};
use crate::{position_mut, validate};

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

    let realized_funding_fee = cal_utils::mul_small_rate_i(
        position.position_size.cast::<i128>()?,
        market_funding_fee_per_size
            .cast::<i128>()?
            .safe_sub(position.open_funding_fee_amount_per_size.cast::<i128>()?)?,
    )?;

    let mut market_processor = MarketProcessor { market };
    position.add_realized_funding_fee(realized_funding_fee)?;
    position.add_realized_funding_fee_in_usd(cal_utils::token_to_usd_i(
        realized_funding_fee,
        token.decimals,
        token_price,
    )?)?;
    position.set_open_funding_fee_amount_per_size(market_funding_fee_per_size)?;
    market_processor.update_market_total_funding_fee(realized_funding_fee, position.is_long)?;
    pool.update_pool_funding_fee(realized_funding_fee, true)?;
    Ok(())
}

pub fn update_borrowing_fee(
    position: &mut UserPosition,
    pool: &mut Pool,
    token_price: u128,
    token: &TradeToken,
) -> BumpResult<()> {
    pool.borrowing_fee.cumulative_borrowing_fee_per_token;
    let realized_borrowing_fee = position.initial_margin.safe_mul(position.leverage)?.safe_mul(
        pool.borrowing_fee
            .cumulative_borrowing_fee_per_token
            .safe_sub(position.open_borrowing_fee_per_token)?,
    )?;

    position.add_realized_borrowing_fee(realized_borrowing_fee)?;
    position.add_realized_borrowing_fee_in_usd(cal_utils::token_to_usd_u(
        realized_borrowing_fee,
        token.decimals,
        token_price,
    )?)?;
    position
        .set_open_borrowing_fee_per_token(pool.borrowing_fee.cumulative_borrowing_fee_per_token)?;
    pool.borrowing_fee.update_total_borrowing_fee(0u128, true, realized_borrowing_fee, true)?;
    Ok(())
}

pub fn decrease_position1<'info>(
    params: DecreasePositionParams,
    user: &mut User,
    market: &mut Market,
    stake_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state_account: &Account<'info, State>,
    user_token_account: Option<&Account<'info, TokenAccount>>,
    pool_vault_account: &Account<'info, TokenAccount>,
    trade_token: Ref<TradeToken>,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    oracle_map: &mut OracleMap,
    position_key: &Pubkey,
) -> BumpResult<()> {
    let (is_long, position_deletion, pre_position, response) = {
        let position = position_mut!(&mut user.user_positions, position_key)?;
        let pre_position = position.clone();
        let position_un_pnl_usd = position.get_position_un_pnl_usd(params.execute_price)?;
        let margin_mint_token_price = oracle_map.get_price_data(&trade_token.oracle)?.price;
        update_borrowing_fee(
            position,
            if position.is_long { stake_token_pool } else { stable_pool },
            params.execute_price,
            &trade_token,
        )?;
        update_funding_fee(
            position,
            market,
            if position.is_long { stake_token_pool } else { stable_pool },
            params.execute_price,
            &trade_token,
        )?;

        let response = calculate_decrease_position(
            params.decrease_size,
            params.is_liquidation,
            params.is_cross_margin,
            position_un_pnl_usd,
            trade_token.decimals,
            margin_mint_token_price,
            market,
            state_account,
            &trade_token,
            position,
        )?;

        if response.settle_margin < 0i128 && !params.is_liquidation && !position.cross_margin {
            return Err(BumpErrorCode::AmountNotEnough);
        }
        let position_deletion = if params.decrease_size != position.position_size {
            false
        } else {
            update_decrease_position(position, params.decrease_size, &response)?;
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
        state_account,
        pre_position.cross_margin,
        response.settle_close_fee,
        response.settle_borrowing_fee,
        response.settle_funding_fee,
        &pre_position.margin_mint,
        params.decrease_size,
        is_long,
        pre_position.entry_price,
    )?;
    //TODO: 适配对应参数
    // settle(
    //     &response,
    //     user_account_loader,
    //     pool_account_loader,
    //     stable_pool_account_loader,
    //     state_account,
    //     user_token_account,
    //     pool_vault_account,
    //     trade_token_loader,
    //     trade_token_vault_account,
    //     bump_signer,
    //     token_program,
    //     &pre_position,
    // )?;

    //cancel stop order
    user.cancel_stop_orders(
        params.order_id,
        pre_position.symbol,
        &pre_position.margin_mint,
        pre_position.cross_margin,
    )?;

    //add insurance fund

    if params.is_liquidation {
        add_insurance_fund(
            market,
            state_account,
            &trade_token,
            &response,
            if pre_position.is_long { stake_token_pool } else { stable_pool },
            &pre_position,
        )?;
    }

    Ok(())
}

fn collect_decrease_fee(
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    market: &mut Market,
    state_account: &Account<State>,
    is_cross_margin: bool,
    settle_close_fee: u128,
    settle_borrowing_fee: u128,
    settle_funding_fee: i128,
    margin_token: &Pubkey,
    decrease_size: u128,
    is_long: bool,
    entry_price: u128,
) -> BumpResult {
    if is_long {
        fee_processor::collect_long_close_position_fee(
            if is_long { base_token_pool } else { stable_pool },
            settle_close_fee,
            is_cross_margin,
        )?;
    } else {
        fee_processor::collect_short_close_position_fee(
            stable_pool,
            base_token_pool,
            state_account,
            settle_close_fee,
            is_cross_margin,
        )?;
    }
    fee_processor::collect_borrowing_fee(
        if is_long { base_token_pool } else { stable_pool },
        settle_borrowing_fee,
        is_cross_margin,
    )?;

    if is_long { base_token_pool } else { stable_pool }.borrowing_fee.update_total_borrowing_fee(
        settle_borrowing_fee,
        true,
        settle_borrowing_fee,
        false,
    )?;
    let mut market_processor = MarketProcessor { market };
    market_processor.update_market_total_funding_fee(settle_funding_fee, is_long)?;
    market_processor.update_oi(
        false,
        UpdateOIParams { margin_token: *margin_token, size: decrease_size, is_long, entry_price },
    )?;
    Ok(())
}

fn update_decrease_position(
    position: &mut UserPosition,
    decrease_size: u128,
    response: &UpdateDecreaseResponse,
) -> BumpResult {
    let pre_position = position.clone();
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
    position.set_last_update(cal_utils::current_time())?;
    emit!(UpdateUserPositionEvent { pre_position, position: position.clone() });
    Ok(())
}

fn settle<'info>(
    response: &UpdateDecreaseResponse,
    user_account_loader: &AccountLoader<'info, User>,
    pool_account_loader: &AccountLoader<'info, Pool>,
    stable_pool_account_loader: &AccountLoader<'info, Pool>,
    state_account: &Account<'info, State>,
    user_token_account: Option<&Account<'info, TokenAccount>>,
    pool_vault_account: &Account<'info, TokenAccount>,
    trade_token_account: &AccountLoader<'info, TradeToken>,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    position: &UserPosition,
) -> BumpResult<()> {
    let mut base_token_pool =
        pool_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadPoolData)?;
    let mut stable_pool =
        stable_pool_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadPoolData)?;
    fee_processor::settle_funding_fee(
        &mut base_token_pool,
        &mut stable_pool,
        response.settle_funding_fee_in_usd,
        response.settle_funding_fee,
        position.is_long,
        position.cross_margin,
    )?;
    let mut pool = if position.is_long { base_token_pool } else { stable_pool };
    let mut pool_processor = PoolProcessor { pool: &mut pool };

    let base_token_pool = &mut pool_account_loader.load_mut().unwrap();
    if position.cross_margin {
        let add_liability = settle_cross(
            response,
            user_account_loader,
            state_account,
            pool_vault_account,
            trade_token_account,
            trade_token_vault_account,
            bump_signer,
            token_program,
            position,
        )?;
        let mut user =
            user_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadUserData)?;
        let repay_amount =
            user.repay_liability(&position.margin_mint, UserTokenUpdateReason::DecreasePosition)?;
        let mut trade_token = trade_token_account
            .load_mut()
            .map_err(|_| BumpErrorCode::CouldNotLoadTradeTokenData)?;
        trade_token.sub_liability(repay_amount)?;

        pool_processor.update_pnl_and_un_hold_pool_amount(
            response.un_hold_pool_amount,
            response.pool_pnl_token,
            add_liability,
            Some(base_token_pool),
        )?;
    } else {
        settle_isolate(
            response,
            state_account,
            user_token_account.ok_or(BumpErrorCode::InvalidParam)?,
            pool_vault_account,
            bump_signer,
            token_program,
        )?;

        let base_token_pool = &mut pool_account_loader.load_mut().unwrap();
        pool_processor.update_pnl_and_un_hold_pool_amount(
            response.un_hold_pool_amount,
            response.pool_pnl_token,
            0u128,
            Some(base_token_pool),
        )?;
    }
    Ok(())
}

fn settle_cross<'info>(
    response: &UpdateDecreaseResponse,
    user_account_loader: &AccountLoader<'info, User>,
    state_account: &Account<'info, State>,
    pool_vault_account: &Account<'info, TokenAccount>,
    trade_token_account: &AccountLoader<'info, TradeToken>,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    position: &UserPosition,
) -> BumpResult<u128> {
    let user = &mut user_account_loader.load_mut().unwrap();

    let mut add_liability = 0u128;
    //record pay fee
    if response.settle_fee > 0i128 {
        add_liability = user.sub_token_with_liability(
            &position.margin_mint,
            &mut *trade_token_account
                .load_mut()
                .map_err(|_e| BumpErrorCode::CouldNotLoadTradeTokenData)?,
            response.settle_fee.abs().cast::<u128>()?,
            &UserTokenUpdateReason::SettleFee,
        )?;
    } else {
        user.add_user_token_amount(
            &position.margin_mint,
            response.settle_fee.abs().cast::<u128>()?,
            &UserTokenUpdateReason::SettleFee,
        )?;
    }

    // release token
    user.un_use_token(&position.margin_mint, response.decrease_margin)?;

    //deal user pnl
    if response.user_realized_pnl_token.safe_add(response.settle_fee)? > 0i128 {
        user.add_user_token_amount(
            &position.margin_mint,
            response.user_realized_pnl_token.safe_add(response.settle_fee)?.abs().cast::<u128>()?,
            &UserTokenUpdateReason::SettlePnl,
        )?;
    } else {
        add_liability = add_liability.safe_add(
            user.sub_token_with_liability(
                &position.margin_mint,
                &mut *trade_token_account
                    .load_mut()
                    .map_err(|_e| BumpErrorCode::CouldNotLoadTradeTokenData)?,
                response
                    .user_realized_pnl_token
                    .safe_add(response.settle_fee)?
                    .abs()
                    .cast::<u128>()?,
                &UserTokenUpdateReason::SettlePnl,
            )?,
        )?;
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
    } else if response.pool_pnl_token.safe_sub(add_liability.cast::<i128>()?)? > 0i128 {
        token::receive(
            token_program,
            trade_token_vault_account,
            pool_vault_account,
            bump_signer,
            response.pool_pnl_token.safe_sub(add_liability.cast::<i128>()?)?.cast::<u128>()?,
        )
        .map_err(|_e| BumpErrorCode::TransferFailed)?;
    }

    if !response.is_liquidation {
        let change_token_amount = response
            .decrease_margin_in_usd_from_portfolio
            .safe_mul(position.initial_margin)?
            .safe_div(position.initial_margin_usd)?
            .cast::<i128>()?
            .safe_add(response.settle_margin.cast::<i128>()?)?
            .safe_sub(response.decrease_margin.cast::<i128>()?)?;

        user.update_all_position_from_portfolio_margin(change_token_amount, &position.margin_mint)?;
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

fn add_insurance_fund(
    market: &Market,
    state: &State,
    trade_token: &TradeToken,
    response: &UpdateDecreaseResponse,
    pool: &mut Pool,
    position: &UserPosition,
) -> BumpResult<()> {
    let mut pool_processor = PoolProcessor { pool };
    if position.cross_margin {
        pool_processor.add_insurance_fund(cal_utils::usd_to_token_u(
            position.get_position_mm(market, state)?,
            trade_token.decimals,
            response.margin_token_price,
        )?)?;
        return Ok(());
    }

    let add_funds;
    if response.settle_fee >= 0i128 {
        add_funds = if response.decrease_margin
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
        add_funds = response
            .decrease_margin
            .safe_add(response.settle_fee.abs().cast::<u128>()?)?
            .safe_sub(response.pool_pnl_token.abs().cast::<u128>()?)?
    }
    pool_processor.add_insurance_fund(add_funds)?;
    Ok(())
}

pub fn execute_reduce_position_margin(
    params: &UpdatePositionMarginParams,
    need_update_leverage: bool,
    trade_token: &TradeToken,
    oracle_map: &mut OracleMap,
    pool: &mut Pool,
    market: &Market,
    state: &State,
    position: &mut UserPosition,
) -> BumpResult<u128> {
    let token_price = oracle_map.get_price_data(&trade_token.oracle)?.price;
    let max_reduce_margin_in_usd = position.initial_margin_usd.safe_sub(
        cal_utils::div_rate_u(position.position_size, market.market_trade_config.max_leverage)?
            .max(state.min_order_margin_usd),
    )?;
    validate!(
        max_reduce_margin_in_usd > params.update_margin_amount,
        BumpErrorCode::AmountNotEnough.into()
    )?;
    let user_key = position.user_key;
    let pre_position = position.clone();
    let reduce_margin_amount =
        cal_utils::usd_to_token_u(params.update_margin_amount, trade_token.decimals, token_price)?;

    if position.cross_margin
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
        position.set_leverage(cal_utils::div_rate_u(
            position.position_size,
            position.initial_margin_usd,
        )?)?;
    }
    if !position.cross_margin {
        position.set_initial_margin_usd_from_portfolio(position.initial_margin_usd)?;
    }
    position.add_hold_pool_amount(reduce_margin_amount)?;
    pool.hold_pool(reduce_margin_amount)?;
    emit!(AddOrDecreaseMarginEvent {
        user_key,
        position: position.clone(),
        pre_position,
        is_add: false,
    });
    Ok(reduce_margin_amount)
}

pub fn calculate_decrease_position(
    decrease_size: u128,
    is_liquidation: bool,
    is_cross_margin: bool,
    pnl: i128,
    decimals: u16,
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
        market.market_trade_config.close_fee_rate,
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
        response.settle_margin = if is_cross_margin {
            //(initial_margin_usd - pos_fee_usd + pnl - mm) * decimals / price
            cal_utils::usd_to_token_i(
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
                decimals,
                margin_mint_token_price,
            )?
        } else {
            0i128
        };
    } else {
        //(initial_margin_usd - pos_fee + pnl) * decrease_percent * decimals / price
        response.settle_margin = cal_utils::usd_to_token_i(
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
            decimals,
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
    response.user_realized_pnl = cal_utils::token_to_usd_i(
        response.user_realized_pnl_token,
        decimals,
        margin_mint_token_price,
    )?;
    response.decrease_margin_in_usd_from_portfolio = if cal_utils::add_u128(
        response.decrease_margin_in_usd,
        position.initial_margin_usd_from_portfolio,
    )? > position.initial_margin_usd
    {
        cal_utils::sub_u128(
            cal_utils::add_u128(
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

fn get_pos_fee_in_usd(
    funding_fee_in_usd: i128,
    borrowing_fee_in_usd: u128,
    close_fee_in_usd: u128,
) -> BumpResult<i128> {
    Ok(funding_fee_in_usd
        .safe_add(borrowing_fee_in_usd.cast::<i128>()?)?
        .safe_add(close_fee_in_usd.cast::<i128>()?)?
        .cast::<i128>()?)
}

fn cal_decrease_borrowing_fee(
    position: &UserPosition,
    decrease_size: u128,
) -> BumpResult<(u128, u128)> {
    if position.position_size == decrease_size {
        return Ok((position.realized_borrowing_fee, position.realized_borrowing_fee_in_usd));
    }
    return Ok((
        position
            .realized_borrowing_fee
            .safe_mul(decrease_size)?
            .safe_div(position.position_size)?,
        position
            .realized_borrowing_fee_in_usd
            .safe_mul(decrease_size)?
            .safe_div(position.position_size)?,
    ));
}

fn cal_decrease_funding_fee(
    position: &UserPosition,
    decrease_size: u128,
) -> BumpResult<(i128, i128)> {
    if position.position_size == decrease_size {
        return Ok((position.realized_funding_fee, position.realized_funding_fee_in_usd));
    }
    return Ok((
        position
            .realized_funding_fee
            .safe_mul(decrease_size.cast()?)?
            .safe_div(position.position_size.cast()?)?,
        position
            .realized_funding_fee_in_usd
            .safe_mul(decrease_size.cast()?)?
            .safe_div(position.position_size.cast()?)?,
    ));
}

fn cal_decrease_close_fee(
    decrease_size: u128,
    trade_token: &TradeToken,
    token_price: u128,
    close_fee_rate: u128,
    position: &UserPosition,
) -> BumpResult<(u128, u128)> {
    if position.position_size == decrease_size {
        return Ok((
            cal_utils::usd_to_token_u(position.close_fee_in_usd, trade_token.decimals, token_price)
                .unwrap(),
            position.close_fee_in_usd,
        ));
    }

    let mut close_fee_in_usd = cal_utils::mul_rate_u(decrease_size, close_fee_rate).unwrap();
    if close_fee_in_usd > position.close_fee_in_usd {
        close_fee_in_usd = position.close_fee_in_usd;
    }

    return Ok((
        cal_utils::usd_to_token_u(close_fee_in_usd, trade_token.decimals, token_price)?
            .safe_mul(decrease_size)?
            .safe_div(position.position_size)?,
        close_fee_in_usd.safe_mul(decrease_size)?.safe_div(position.position_size)?,
    ));
}

pub fn execute_add_position_margin(
    params: &UpdatePositionMarginParams,
    trade_token: &TradeToken,
    oracle_map: &mut OracleMap,
    mut pool: &mut Pool,
    position: &mut UserPosition,
) -> BumpResult<()> {
    let token_price = oracle_map.get_price_data(&trade_token.oracle)?.price;
    let user_key = position.user_key;
    validate!(
        params.update_margin_amount
            < cal_utils::usd_to_token_u(
                position.position_size.safe_sub(position.initial_margin_usd)?,
                trade_token.decimals,
                token_price
            )?,
        BumpErrorCode::AmountNotEnough
    )?;

    let mut add_or_decrease_margin_event = AddOrDecreaseMarginEvent {
        user_key,
        position: Default::default(),
        pre_position: position.clone(),
        is_add: true,
    };

    position.add_initial_margin(params.update_margin_amount)?;
    if position.cross_margin {
        position.set_initial_margin_usd(cal_utils::div_rate_u(
            position.position_size,
            position.leverage,
        )?)?;
        position.add_initial_margin_usd_from_portfolio(params.add_initial_margin_from_portfolio)?;
    } else {
        position.add_initial_margin_usd(cal_utils::token_to_usd_u(
            params.update_margin_amount,
            trade_token.decimals,
            token_price,
        )?)?;
        position.set_leverage(cal_utils::div_rate_u(
            position.position_size,
            position.initial_margin_usd,
        )?)?;
        position.set_initial_margin_usd_from_portfolio(position.initial_margin)?;
    }

    let sub_amount = params.update_margin_amount.min(position.hold_pool_amount);
    position.sub_hold_pool_amount(sub_amount)?;
    let mut pool_processor = PoolProcessor { pool: &mut pool };
    pool_processor.update_pnl_and_un_hold_pool_amount(sub_amount, 0i128, 0u128, None)?;

    add_or_decrease_margin_event.position = position.clone();
    emit!(add_or_decrease_margin_event);
    Ok(())
}

pub fn increase_position(
    user_account_loader: &AccountLoader<User>,
    pool_account_loader: &AccountLoader<Pool>,
    stable_pool_account_loader: &AccountLoader<Pool>,
    market_account_loader: &AccountLoader<Market>,
    trade_token_loader: &AccountLoader<TradeToken>,
    program_id: &Pubkey,
    order: &UserOrder,
    order_margin: u128,
    order_margin_from_balance: u128,
    execute_price: u128,
    margin_token_price: u128,
    fee: u128,
) -> BumpResult<()> {
    let trade_token =
        trade_token_loader.load().map_err(|_| BumpErrorCode::CouldNotLoadTradeTokenData)?;
    let market =
        &mut market_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadMarketData)?;
    let mut user =
        user_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadTradeTokenData)?;

    let position_key =
        pda::generate_position_key(&user.user_key, market.symbol, order.cross_margin, program_id)?;

    let position_index = user
        .get_user_position_index(&position_key)
        .or_else(|_| user.add_user_position(&position_key))?;
    let position = &mut user.user_positions[position_index];

    let is_long = order.order_side.eq(&OrderSide::LONG);
    if position.leverage != order.leverage {
        return Err(BumpErrorCode::LeverageIsNotAllowed.into());
    }
    if position.position_size == 0u128 && position.status.eq(&PositionStatus::INIT) {
        position.set_index_mint(market.index_mint)?;
        position.set_symbol(order.symbol)?;
        position.set_margin_mint(order.margin_mint)?;
        position.set_leverage(order.leverage)?;
        position.set_is_long(order.order_side.eq(&OrderSide::LONG))?;
        position.set_cross_margin(order.cross_margin)?;
        position.set_status(PositionStatus::USING)?;
    }

    let base_token_pool =
        &mut pool_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadPoolData)?;
    let stable_pool = &mut stable_pool_account_loader
        .load_mut()
        .map_err(|_| BumpErrorCode::CouldNotLoadPoolData)?;
    let pool = if position.is_long { base_token_pool } else { stable_pool };

    let increase_margin = cal_utils::sub_u128(order_margin, fee)?;
    let increase_margin_from_balance = if order_margin_from_balance > fee {
        cal_utils::sub_u128(order_margin_from_balance, fee)?
    } else {
        0u128
    };
    let decimal = trade_token.decimals;
    let increase_size = cal_utils::token_to_usd_u(
        cal_utils::mul_u128(increase_margin, order.leverage)?,
        decimal,
        margin_token_price,
    )?;
    let increase_hold =
        cal_utils::mul_rate_u(increase_margin, cal_utils::sub_u128(order.leverage, 1u128)?)?;

    if position.position_size == 0u128 {
        //new position
        position.set_margin_mint(order.margin_mint)?;
        position.set_entry_price(execute_price)?;
        position.set_initial_margin(increase_margin)?;
        position.set_initial_margin_usd(cal_utils::token_to_usd_u(
            increase_margin,
            decimal,
            margin_token_price,
        )?)?;
        position.set_initial_margin_usd_from_portfolio(cal_utils::token_to_usd_u(
            increase_margin_from_balance,
            decimal,
            margin_token_price,
        )?)?;
        position.set_close_fee_in_usd(cal_utils::mul_rate_u(
            increase_size,
            market.market_trade_config.close_fee_rate,
        )?)?;
        position.set_position_size(increase_size)?;
        position.set_leverage(order.leverage)?;
        position.set_realized_pnl(
            -cal_utils::token_to_usd_u(fee, decimal, margin_token_price)?.cast::<i128>()?,
        )?;
        position.set_open_borrowing_fee_per_token(
            pool.borrowing_fee.cumulative_borrowing_fee_per_token,
        )?;
        position.set_open_funding_fee_amount_per_size(if is_long {
            market.funding_fee.long_funding_fee_amount_per_size
        } else {
            market.funding_fee.short_funding_fee_amount_per_size
        })?;
        position.set_last_update(cal_utils::current_time())?;
        position.add_hold_pool_amount(increase_hold)?;
        emit!(AddOrDeleteUserPositionEvent { position: position.clone(), is_add: true });
    } else {
        let pre_position = position.clone();
        //increase position
        update_borrowing_fee(position, pool, margin_token_price, &trade_token)?;
        update_funding_fee(position, market, pool, margin_token_price, &trade_token)?;
        position.set_entry_price(cal_utils::compute_avg_entry_price(
            position.position_size,
            position.entry_price,
            increase_size,
            margin_token_price,
            market.market_trade_config.tick_size,
            position.is_long,
        )?)?;
        position.add_initial_margin(increase_margin)?;
        position.add_initial_margin_usd(cal_utils::token_to_usd_u(
            increase_margin,
            trade_token.decimals,
            margin_token_price,
        )?)?;
        position.add_initial_margin_usd_from_portfolio(cal_utils::token_to_usd_u(
            increase_margin_from_balance,
            trade_token.decimals,
            margin_token_price,
        )?)?;
        position.add_position_size(increase_size)?;
        position.add_realized_pnl(
            -cal_utils::token_to_usd_u(fee, decimal, margin_token_price)?.cast::<i128>()?,
        )?;
        position.set_last_update(cal_utils::current_time())?;
        position.add_hold_pool_amount(increase_hold)?;
        emit!(UpdateUserPositionEvent { pre_position, position: position.clone() });
    }

    // update market io
    let mut market_processor = MarketProcessor { market };
    market_processor.update_oi(
        true,
        UpdateOIParams {
            margin_token: order.margin_mint,
            size: increase_size,
            is_long,
            entry_price: execute_price,
        },
    )?;

    //lock pool amount
    pool.hold_pool(increase_hold)?;

    Ok(())
}

pub fn update_leverage<'info>(
    params: UpdatePositionLeverageParams,
    position_key: &Pubkey,
    user_account: &AccountLoader<'info, User>,
    authority: &Signer<'info>,
    pool: &AccountLoader<'info, Pool>,
    state: &Account<'info, State>,
    market: &AccountLoader<'info, Market>,
    user_token_account: &Account<'info, TokenAccount>,
    pool_vault: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
) -> BumpResult<()> {
    let mut user = user_account.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadUserData)?;
    let position = user.get_user_position_mut_ref(position_key)?;
    let trade_token = trade_token_map.get_trade_token(&position.margin_mint)?;
    let pool = &mut pool.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadPoolData)?;

    if position.position_size != 0u128 {
        if position.leverage > params.leverage {
            let add_margin_amount;
            let mut add_initial_margin_from_portfolio = 0u128;
            if position.cross_margin {
                let user = &mut user_account
                    .load_mut()
                    .map_err(|_| BumpErrorCode::CouldNotLoadUserData)?;
                let available_amount =
                    user.get_user_token_ref(&trade_token.mint)?.get_token_available_amount()?;
                position.set_leverage(params.leverage)?;
                let new_initial_margin_in_usd =
                    cal_utils::div_rate_u(position.position_size, position.leverage)?;
                let add_margin_in_usd = if new_initial_margin_in_usd > position.initial_margin_usd {
                    new_initial_margin_in_usd.safe_sub(position.initial_margin_usd)?
                } else {
                    0u128
                };
                let cross_available_value =
                    user.get_available_value(oracle_map, trade_token_map)?;
                validate!(
                    add_margin_in_usd.cast::<i128>()? > cross_available_value,
                    BumpErrorCode::AmountNotEnough.into()
                )?;

                let user = &mut user_account
                    .load_mut()
                    .map_err(|_e| BumpErrorCode::CouldNotLoadUserData)?;
                add_margin_amount = cal_utils::usd_to_token_u(
                    add_margin_in_usd,
                    trade_token.decimals,
                    position.entry_price,
                )?;
                add_initial_margin_from_portfolio = cal_utils::token_to_usd_u(
                    add_margin_amount.min(available_amount),
                    trade_token.decimals,
                    position.entry_price,
                )?;
                user.use_token(
                    &trade_token.mint,
                    add_margin_amount,
                    user_token_account.to_account_info().key,
                    false,
                )?;
            } else {
                add_margin_amount = params.add_margin_amount;
            }

            execute_add_position_margin(
                &UpdatePositionMarginParams {
                    position_key: *position_key,
                    is_add: true,
                    update_margin_amount: add_margin_amount,
                    add_initial_margin_from_portfolio,
                    ..UpdatePositionMarginParams::default()
                },
                &trade_token,
                oracle_map,
                pool,
                position,
            )?;
            if !params.is_cross_margin {
                token::receive(
                    token_program,
                    user_token_account,
                    pool_vault,
                    authority,
                    params.add_margin_amount,
                )
                .map_err(|_e| BumpErrorCode::TransferFailed)?;
            }
        } else {
            position.set_leverage(params.leverage)?;
            let reduce_margin = position
                .initial_margin_usd
                .safe_sub(cal_utils::div_rate_u(position.position_size, position.leverage)?)?;
            let reduce_margin_amount = execute_reduce_position_margin(
                &UpdatePositionMarginParams {
                    position_key: *position_key,
                    is_add: false,
                    update_margin_amount: reduce_margin,
                    add_initial_margin_from_portfolio: 0,
                    ..UpdatePositionMarginParams::default()
                },
                false,
                &trade_token,
                oracle_map,
                pool,
                &*market.load().map_err(|_| BumpErrorCode::CouldNotLoadMarketData)?,
                state,
                position,
            )?;
            if position.cross_margin {
                let user = &mut user_account
                    .load_mut()
                    .map_err(|_| BumpErrorCode::CouldNotLoadUserData)?;
                user.un_use_token(&position.margin_mint, reduce_margin_amount)?;
            } else {
                token::send_from_program_vault(
                    token_program,
                    pool_vault,
                    user_token_account,
                    bump_signer,
                    state.bump_signer_nonce,
                    reduce_margin_amount,
                )
                .map_err(|_e| BumpErrorCode::TransferFailed)?
            }
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
    pub is_cross_margin: bool,
    pub decimals: u16,
}

#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct DecreasePositionParams {
    pub order_id: u128,
    pub is_liquidation: bool,
    pub is_cross_margin: bool,
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
