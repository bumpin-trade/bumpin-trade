use std::ops::{Deref, DerefMut};

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::instructions::constraints::*;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor::DecreasePositionParams;
use crate::processor::{fee_processor, position_processor};
use crate::state::infrastructure::user_order::{
    OrderSide, OrderStatus, OrderType, PositionSide, StopType, UserOrder,
};
use crate::state::infrastructure::user_position::PositionStatus;
use crate::state::market::Market;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::pool_map::PoolMap;
use crate::state::state::State;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::state::vault_map::VaultMap;
use crate::state::UserTokenUpdateReason;
use crate::utils::{pda, token};
use crate::{get_then_update_id, position, validate};

#[derive(Accounts)]
#[instruction(
    order: PlaceOrderParams
)]
pub struct PlaceOrder<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ? && is_normal(& user) ?,
    )]
    pub user: AccountLoader<'info, User>,

    pub authority: Signer<'info>,

    #[account(
        mut,
        token::authority = authority
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct PlaceOrderParams {
    pub symbol: [u8; 32],
    pub size: u128,
    pub order_margin: u128,
    pub leverage: u32,
    pub trigger_price: u128,
    pub acceptable_price: u128,
    pub place_time: i64,
    pub pool_index: u16,
    pub stable_pool_index: u16,
    pub market_index: u16,
    pub trade_token_index: u16,
    pub stable_trade_token_index: u16,
    pub is_portfolio_margin: bool,
    pub is_native_token: bool,
    pub order_side: OrderSide,
    pub position_side: PositionSide,
    pub order_type: OrderType,
    pub stop_type: StopType,
    pub order_id: u64, // only for execute order from keeper
}

pub fn handle_place_order<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PlaceOrder<'c>>,
    order: PlaceOrderParams,
) -> Result<()> {
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { trade_token_map, mut oracle_map, market_map, pool_map, vault_map } =
        load_maps(remaining_accounts)?;
    let market = market_map.get_mut_ref(&order.symbol)?;
    let user = &mut ctx.accounts.user.load_mut()?;
    let pool = pool_map.get_mut_ref(&market.pool_key)?;
    let stable_pool = pool_map.get_mut_ref(&market.stable_pool_key)?;
    let pool_vault = vault_map.get_account(&pool.mint_vault_key)?;
    let stable_pool_vault = vault_map.get_account(&stable_pool.mint_vault_key)?;
    let margin_token = if order.order_side.eq(&OrderSide::LONG) {
        &market.pool_mint_key
    } else {
        &market.stable_pool_mint_key
    };
    validate!(ctx.accounts.user_token_account.mint.eq(margin_token), BumpErrorCode::InvalidParam)?;

    let token_price = oracle_map
        .get_price_data(&market.index_mint_oracle)
        .map_err(|_e| BumpErrorCode::OracleNotFound)?
        .price;
    validate!(
        validate_place_order(
            &order,
            margin_token,
            &market,
            if order.order_side.eq(&OrderSide::LONG) { pool.deref() } else { stable_pool.deref() },
            &ctx.accounts.state,
            token_price
        )?,
        BumpErrorCode::InvalidParam
    )?;

    if order.position_side == PositionSide::INCREASE && !order.is_portfolio_margin {
        //isolate order, transfer order_margin into pool
        token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            if order.order_side.eq(&OrderSide::LONG) { pool_vault } else { stable_pool_vault },
            &ctx.accounts.authority,
            order.order_margin,
        )?;
    }
    if order.position_side.eq(&PositionSide::INCREASE) && order.is_portfolio_margin {
        //hold usd
        user.add_order_hold_in_usd(order.order_margin)?;
    }

    if user.has_other_short_order(order.symbol, margin_token.key(), order.is_portfolio_margin)? {
        return Err(BumpErrorCode::OnlyOneShortOrderAllowed.into());
    }

    let order_id = get_then_update_id!(user, next_order_id);
    let user_order = UserOrder {
        authority: user.authority,
        order_id,
        symbol: order.symbol,
        order_side: order.order_side,
        position_side: order.position_side,
        order_type: order.order_type,
        stop_type: order.stop_type,
        is_portfolio_margin: order.is_portfolio_margin,
        margin_mint_key: margin_token.key(),
        order_margin: order.order_margin,
        leverage: order.leverage,
        order_size: order.size,
        trigger_price: order.trigger_price,
        acceptable_price: order.acceptable_price,
        created_at: cal_utils::current_time(),
        status: OrderStatus::USING,
        ..Default::default()
    };
    drop(market);
    drop(pool);
    drop(stable_pool);
    if order.order_type.eq(&OrderType::MARKET) {
        let state_account = &ctx.accounts.state;
        let bump_signer_account_info = &ctx.accounts.bump_signer;
        let token_program = &ctx.accounts.token_program;
        return handle_execute_order(
            user,
            &market_map,
            &pool_map,
            state_account,
            &ctx.accounts.user_token_account,
            &vault_map,
            bump_signer_account_info,
            token_program,
            ctx.program_id,
            &trade_token_map,
            &mut oracle_map,
            &user_order,
        );
    } else {
        //store order, wait to execute
        let next_index = user.next_usable_order_index()?;
        user.add_order(&user_order, next_index)?;
    }
    Ok(())
}

pub fn handle_execute_order<'info>(
    user: &mut User,
    market_map: &MarketMap,
    pool_map: &PoolMap,
    state_account: &Account<'info, State>,
    user_token_account: &Account<'info, TokenAccount>,
    vault_map: &VaultMap<'info>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    program_id: &Pubkey,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    user_order: &UserOrder,
) -> Result<()> {
    let user_key = user.key;
    let mut market = market_map.get_mut_ref(&user_order.symbol)?;
    let mut base_token_pool = pool_map.get_mut_ref(&market.pool_key)?;
    let mut stable_pool = pool_map.get_mut_ref(&market.stable_pool_key)?;
    let mut trade_token = trade_token_map.get_trade_token_ref_mut(&market.pool_mint_key)?;
    let pool_vault = vault_map.get_account(&base_token_pool.mint_vault_key)?;
    let stable_pool_vault = vault_map.get_account(&stable_pool.mint_vault_key)?;
    let mut stable_trade_token =
        trade_token_map.get_trade_token_ref_mut(&market.stable_pool_mint_key)?;
    let token_vault = if user_order.order_side.eq(&OrderSide::LONG) {
        vault_map.get_account(&trade_token.vault_key)?
    } else {
        vault_map.get_account(&stable_trade_token.vault_key)?
    };

    //validate trade_token_vault
    validate!(
        if user_order.order_side.eq(&OrderSide::LONG) {
            trade_token.vault_key.eq(token_vault.to_account_info().key)
        } else {
            stable_trade_token.vault_key.eq(token_vault.to_account_info().key)
        },
        BumpErrorCode::InvalidParam
    )?;

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
        .get_price_data(if user_order.order_side.eq(&OrderSide::LONG) {
            &trade_token.oracle_key
        } else {
            &stable_trade_token.oracle_key
        })
        .map_err(|_e| BumpErrorCode::OracleNotFound)?
        .price;
    //update funding_fee_rate and borrowing_fee_rate
    market.deref_mut().update_market_funding_fee_rate(
        state_account,
        margin_token_price,
        if user_order.order_side.eq(&OrderSide::LONG) {
            trade_token.decimals
        } else {
            stable_trade_token.decimals
        },
    )?;

    if user_order.order_side.eq(&OrderSide::LONG) {
        base_token_pool.deref_mut().update_pool_borrowing_fee_rate()?;
    } else {
        stable_pool.deref_mut().update_pool_borrowing_fee_rate()?;
    }
    let position_key = pda::generate_position_key(
        &user_key,
        market.symbol,
        user_order.is_portfolio_margin,
        program_id,
    )?;

    // //do execute order and change position
    match user_order.position_side {
        PositionSide::NONE => Err(BumpErrorCode::PositionSideNotSupport),
        PositionSide::INCREASE => {
            {
                let margin_token = if user_order.order_side.eq(&OrderSide::LONG) {
                    &market.pool_mint_key
                } else {
                    &market.stable_pool_mint_key
                };
                //calculate real order_margin with validation
                let (order_margin, order_margin_from_balance) = execute_increase_order_margin(
                    user_token_account.to_account_info().key,
                    &user_order,
                    margin_token,
                    if user_order.order_side.eq(&OrderSide::LONG) {
                        trade_token.decimals
                    } else {
                        stable_trade_token.decimals
                    },
                    user,
                    margin_token_price,
                    oracle_map,
                    trade_token_map,
                    market_map,
                    state_account,
                )?;

                //collect open fee
                let fee = if user_order.order_side.eq(&OrderSide::LONG) {
                    fee_processor::collect_long_open_position_fee(
                        &market,
                        base_token_pool.deref_mut(),
                        order_margin.safe_mul_rate(user_order.leverage.cast()?)?,
                        user_order.is_portfolio_margin,
                    )?
                } else {
                    fee_processor::collect_short_open_position_fee(
                        &market,
                        base_token_pool.deref_mut(),
                        stable_pool.deref_mut(),
                        state_account,
                        order_margin.safe_mul_rate(user_order.leverage.cast()?)?,
                        user_order.is_portfolio_margin,
                    )?
                };

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
                //increase position
                position_processor::increase_position(
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
                )?;
                Ok(())
            }
        },

        PositionSide::DECREASE => {
            {
                let position_side = { position!(&user.positions, &position_key)?.is_long };
                //decrease
                let position = user.get_user_position_ref(&position_key)?;
                if position.position_size == 0u128 || position.status.eq(&PositionStatus::INIT) {
                    return Err(BumpErrorCode::InvalidParam.into());
                }
                if position.is_long == is_long {
                    return Err(BumpErrorCode::InvalidParam.into());
                }

                position_processor::decrease_position(
                    DecreasePositionParams {
                        order_id: user_order.order_id,
                        is_liquidation: false,
                        is_portfolio_margin: false,
                        margin_token: user_order.margin_mint_key,
                        decrease_size: user_order.order_size,
                        execute_price,
                    },
                    user,
                    market.deref_mut(),
                    base_token_pool.deref_mut(),
                    stable_pool.deref_mut(),
                    state_account,
                    Some(user_token_account),
                    if position_side { pool_vault } else { stable_pool_vault },
                    if user_order.order_side.eq(&OrderSide::LONG) {
                        trade_token.deref_mut()
                    } else {
                        stable_trade_token.deref_mut()
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

fn validate_place_order(
    order: &PlaceOrderParams,
    token: &Pubkey,
    market: &Market,
    pool: &Pool,
    state: &State,
    token_price: u128,
) -> BumpResult<bool> {
    match order.order_type {
        OrderType::NONE => Ok(false),
        _ => {
            if order.position_side.eq(&PositionSide::DECREASE) && order.size == 0u128 {
                Ok(false)
            } else if order.order_side.eq(&OrderSide::NONE) {
                Ok(false)
            } else if order.order_type.eq(&OrderType::LIMIT)
                && order.position_side.eq(&PositionSide::DECREASE)
            {
                Ok(false)
            } else if order.order_type.eq(&OrderType::STOP)
                && (order.stop_type.eq(&StopType::NONE) || order.trigger_price == 0u128)
            {
                Ok(false)
            } else if order.position_side.eq(&PositionSide::INCREASE) {
                if order.order_margin == 0u128 {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::LONG) && !token.eq(&market.pool_mint_key)
                {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::LONG)
                    && !market.pool_mint_key.eq(&pool.mint_key)
                {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::SHORT) && !pool.mint_key.eq(token) {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::SHORT)
                    && !market.stable_pool_mint_key.eq(&pool.mint_key)
                {
                    Ok(false)
                } else if order.is_portfolio_margin
                    && order.order_margin < state.minimum_order_margin_usd
                {
                    Ok(false)
                } else if !order.is_portfolio_margin
                    && order.order_margin.safe_mul(token_price)? < state.minimum_order_margin_usd
                {
                    Ok(false)
                } else {
                    Ok(true)
                }
            } else {
                Ok(true)
            }
        },
    }
}

fn execute_increase_order_margin(
    user_token_account_key: &Pubkey,
    order: &UserOrder,
    margin_token: &Pubkey,
    decimals: u16,
    user: &mut User,
    margin_token_price: u128,
    oracle_map: &mut OracleMap,
    trade_token_map: &TradeTokenMap,
    market_map: &MarketMap,
    state: &Account<State>,
) -> BumpResult<(u128, u128)> {
    let order_margin;
    let order_margin_from_balance;
    if order.is_portfolio_margin {
        let available_value =
            user.get_available_value(trade_token_map, oracle_map, market_map, state)?;
        let order_margin_temp;
        if available_value < 0i128 {
            let fix_order_margin_in_usd =
                order.order_margin.cast::<i128>()?.safe_add(available_value)?;
            validate!(fix_order_margin_in_usd > 0i128, BumpErrorCode::BalanceNotEnough.into())?;
            user.sub_order_hold_in_usd(order.order_margin)?;
            order_margin_temp = fix_order_margin_in_usd.abs().cast::<u128>()?;
        } else {
            order_margin_temp = order.order_margin;
            user.sub_order_hold_in_usd(order.order_margin)?;
        }
        order_margin = cal_utils::usd_to_token_u(order_margin_temp, decimals, margin_token_price)?;
        order_margin_from_balance =
            user.use_token(margin_token, order_margin, user_token_account_key, false)?;
    } else {
        let order_margin_in_usd =
            cal_utils::token_to_usd_u(order.order_margin, decimals, margin_token_price)?;
        validate!(
            order_margin_in_usd >= state.minimum_order_margin_usd,
            BumpErrorCode::AmountNotEnough
        )?;
        order_margin = order.order_margin;
        order_margin_from_balance = order.order_margin;
    }

    Ok((order_margin, order_margin_from_balance))
}

fn get_execution_price(index_price: u128, order: &UserOrder) -> BumpResult<u128> {
    if order.order_type.eq(&OrderType::MARKET) {
        if order.order_side.eq(&OrderSide::LONG) && index_price >= order.acceptable_price {
            return Err(BumpErrorCode::PriceIsNotAllowed);
        }
        if order.order_side.eq(&OrderSide::SHORT) && index_price <= order.acceptable_price {
            return Err(BumpErrorCode::PriceIsNotAllowed);
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

fn validate_execute_order(order: &UserOrder, market: &Market) -> BumpResult<()> {
    // token verify
    if order.position_side.eq(&PositionSide::INCREASE) {
        if order.order_side.eq(&OrderSide::LONG) && order.margin_mint_key != market.pool_mint_key {
            return Err(BumpErrorCode::TokenNotMatch.into());
        }

        if order.order_side.eq(&OrderSide::SHORT)
            && order.margin_mint_key != market.stable_pool_mint_key
        {
            return Err(BumpErrorCode::TokenNotMatch);
        }
    }

    if order.leverage > market.config.maximum_leverage
        || order.leverage < market.config.minimum_leverage
    {
        return Err(BumpErrorCode::LeverageIsNotAllowed);
    }
    Ok(())
}
