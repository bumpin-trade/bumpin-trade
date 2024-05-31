use std::cell::Ref;
use std::ops::{Deref, DerefMut};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use crate::{get_then_update_id, validate};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;

use crate::instructions::constraints::*;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::position_processor::{DecreasePositionParams, IncreasePositionParams, PositionProcessor};
use crate::processor::user_processor::UserProcessor;
use crate::state::infrastructure::user_order::{OrderSide, OrderStatus, OrderType, PositionSide, StopType, UserOrder};
use crate::state::infrastructure::user_position::{PositionStatus, UserPosition};
use crate::state::market::Market;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::utils::token;

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(
        mut,
        constraint = can_sign_for_user(& user_account, & authority) ?
    )]
    pub user_account: AccountLoader<'info, User>,

    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = pool.load() ?.pool_mint.eq(& margin_token.mint.key())
    )]
    pub margin_token: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pool.load() ?.pool_mint == market.load() ?.pool_mint_key
    )]
    pub pool: AccountLoader<'info, Pool>,

    pub market: AccountLoader<'info, Market>,

    pub state: Account<'info, State>,
    #[account(
        mut,
        constraint = & pool_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub pool_vault: Account<'info, TokenAccount>,

    pub trade_token: AccountLoader<'info, TradeToken>,

    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct PlaceOrderParams {
    pub symbol: [u8; 32],
    pub is_cross_margin: bool,
    pub is_native_token: bool,
    pub order_side: OrderSide,
    pub position_side: PositionSide,
    pub order_type: OrderType,
    pub stop_type: StopType,
    pub size: u128,
    pub order_margin: u128,
    pub leverage: u128,
    pub trigger_price: u128,
    pub acceptable_price: u128,
    pub place_time: u128,
}


pub fn handle_place_order(ctx: Context<PlaceOrder>, order: PlaceOrderParams) -> anchor_lang::Result<()> {
    if order.position_side == PositionSide::INCREASE && !order.is_cross_margin {
        //isolate order, transfer order_margin into pool
        token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            &ctx.accounts.pool_vault,
            &ctx.accounts.authority,
            order.order_margin,
        )?;
    }

    let market = ctx.accounts.market.load()?;
    let user = ctx.accounts.user_account.load_mut()?;
    let pool = ctx.accounts.pool.load()?;
    let token = &ctx.accounts.margin_token;
    validate!(validate_place_order(order, &token.mint, &market, &pool, &ctx.accounts.state)?, BumpErrorCode::InvalidParam.into());

    if user.has_other_short_order(order.symbol, token.mint, order.is_cross_margin)? {
        return Err(BumpErrorCode::OnlyOneShortOrderAllowed.into());
    }

    let order_id = get_then_update_id!(user, next_order_id);
    let mut user_order = UserOrder {
        authority: user.authority,
        order_id,
        symbol: order.symbol,
        order_side: order.order_side,
        position_side: order.position_side,
        order_type: order.order_type,
        stop_type: order.stop_type,
        cross_margin: order.is_cross_margin,
        margin_token: token.mint,
        order_margin: order.order_margin,
        leverage: order.leverage,
        order_size: order.size,
        trigger_price: order.trigger_price,
        acceptable_price: order.acceptable_price,
        time: cal_utils::current_time(),
        status: OrderStatus::USING,
    };
    if order.position_side.eq(&PositionSide::INCREASE) && order.is_cross_margin {
        //hold usd
        user.add_order_hold_in_usd(order.order_margin)?;
    }


    if order.order_type.eq(&OrderType::MARKET) {
        //execute order
        return handle_execute_order(ctx, user_order, order_id, false);
    } else {
        //store order, wait to execute
        let next_index = user.next_usable_order_index();
        user.add_order(&mut user_order, next_index?)?;
    }
    Ok(())
}

fn validate_place_order(order: PlaceOrderParams, token: &Pubkey, market: &Ref<Market>, pool: &Ref<Pool>, state: &State) -> BumpResult<bool> {
    let mut res = true;
    match order.order_type {
        OrderType::NONE => { res = false }
        OrderType::MARKET => {}
        OrderType::LIMIT => {}
        OrderType::STOP => {}
    };

    if order.position_side.eq(&PositionSide::DECREASE) && order.size == 0u128 {
        res = false;
    }

    if order.order_side.eq(&OrderSide::NONE) {
        res = false;
    }
    if order.order_type.eq(&OrderType::LIMIT) && order.position_side.eq(&PositionSide::DECREASE) {
        res = false;
    }
    if order.order_type.eq(&OrderType::STOP) && (order.stop_type.eq(&StopType::NONE) || order.trigger_price == 0u128) {
        res = false;
    }
    if order.position_side.eq(&PositionSide::INCREASE) {
        if order.order_margin == 0u128 {
            res = false;
        }
        if order.order_side.eq(&OrderSide::LONG) && !token.eq(&market.pool_mint_key) {
            res = false;
        }

        if order.order_side.eq(&OrderSide::SHORT) && !pool.pool_mint.eq(&token) {
            res = false;
        }
        if order.is_cross_margin && order.order_margin < state.min_order_margin_usd {
            res = false;
        }
    }
    Ok(res)
}

pub fn handle_execute_order(ctx: Context<PlaceOrder>, mut user_order: UserOrder, order_id: u128, execute_from_remote: bool) -> anchor_lang::Result<()> {
    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
    let oracle_map = &mut OracleMap::load(remaining_accounts_iter).as_ref().unwrap();
    let trade_token_map = &mut TradeTokenMap::load(remaining_accounts_iter)?;

    let mut user = ctx.accounts.user_account.load_mut()?;
    let margin_token = &ctx.accounts.margin_token;
    let mut market = ctx.accounts.market.load_mut()?;
    let trade_token = ctx.accounts.trade_token.load()?;
    let mut market_processor = MarketProcessor { market: &mut market };
    let mut pool = ctx.accounts.pool.load_mut()?;
    let mut pool_processor = PoolProcessor { pool: &mut pool };

    let order = if execute_from_remote { user.find_order_by_id(order_id)? } else { &mut user_order };

    //validate order
    validate_execute_order(order, &market, &pool)?;

    let is_long = OrderSide::LONG == order.order_side;
    let execute_price = get_execution_price(oracle_map, order, market.index_mint_key)?;

    let position_option = user.find_position_by_seed(&user.authority, market.symbol, &margin_token.mint, order.cross_margin, ctx.program_id)?;
    let mut position;
    match position_option {
        None => { position = UserPosition::default() }
        Some(pos) => { position = if pos.status.eq(&PositionStatus::INIT) { UserPosition::default() } else { *pos } }
    }

    //update funding_fee_rate and borrowing_fee_rate
    market_processor.update_market_funding_fee_rate(&ctx.accounts.state, oracle_map)?;
    pool_processor.update_pool_borrowing_fee_rate()?;

    let mut position_processor = PositionProcessor { position: &mut position };

    //do execute order and change position, cal fee....
    match order.position_side {
        PositionSide::NONE => { Err(BumpErrorCode::AmountNotEnough) }
        PositionSide::INCREASE => Ok({
            let margin_token_price;
            let decimals;

            if market.index_mint_key == margin_token.mint {
                margin_token_price = execute_price;
                decimals = trade_token.decimals;
            } else {
                margin_token_price = oracle_map.get_price_data(&margin_token.mint)?.price;
                decimals = market.index_mint_key_decimal;
            }

            let (order_margin, order_margin_from_balance) = execute_increase_order_margin(order, &margin_token.mint, decimals, user.deref_mut(), margin_token_price, oracle_map, trade_token_map, &ctx.accounts.state.into_inner());
            if position.position_size == 0u128 && position.status.eq(&PositionStatus::INIT) {
                if user.has_other_order(order.order_id)? && user.get_order_leverage(order.symbol, order.order_side, order.cross_margin, order.leverage)? == order.leverage {
                    return Err(BumpErrorCode::AmountNotEnough.into());
                }
                position.set_position_key(user.generate_position_key(&user.authority, order.symbol, &order.margin_token, order.cross_margin, ctx.program_id)?)?;
                position.set_authority(user.authority)?;
                position.set_index_mint(market.index_mint_key)?;
                position.set_symbol(order.symbol)?;
                position.set_margin_mint(order.margin_token)?;
                position.set_leverage(order.leverage)?;
                position.set_is_long(order.order_side.eq(&OrderSide::LONG))?;
                position.set_cross_margin(order.cross_margin)?;
                position.set_status(PositionStatus::USING)?;
                user.add_position(&mut position, user.next_usable_position_index()?)?;
            } else if position.leverage != order.leverage {
                return Err(BumpErrorCode::LeverageIsNotAllowed.into());
            }

            position_processor.increase_position(IncreasePositionParams {
                margin_token: order.margin_token,
                increase_margin: order_margin?,
                increase_margin_from_balance: order_margin_from_balance?,
                margin_token_price,
                index_token_price: execute_price,
                leverage: order.leverage,
                is_long,
                is_cross_margin: order.cross_margin,
                decimals,
            }, &trade_token, user.deref_mut(), &mut ctx.accounts.state, market.deref_mut(), pool.deref_mut(), &mut market_processor)?;
        }),
        PositionSide::DECREASE => Ok({
            //decrease
            if position.position_size == 0u128 || position.status.eq(&PositionStatus::INIT) {
                return Err(BumpErrorCode::InvalidParam.into());
            }
            if position.is_long == is_long {
                return Err(BumpErrorCode::InvalidParam.into());
            }

            if position.position_size < order.order_size {
                order.order_size = position.position_size;
            }
            position_processor.decrease_position(DecreasePositionParams {
                order_id,
                is_liquidation: false,
                is_cross_margin: false,
                margin_token: order.margin_token,
                decrease_size: order.order_size,
                execute_price,
            }, trade_token.deref(), user.deref_mut(), &mut ctx.accounts.state, market.deref_mut(), pool.deref_mut(), oracle_map, ctx)?
        })
    }?;
    //delete order
    user.delete_order(order_id)?;
    Ok(())
}


fn execute_increase_order_margin(order: &mut UserOrder,
                                 margin_token: &Pubkey,
                                 decimals: u8,
                                 mut user: &mut User,
                                 margin_token_price: u128,
                                 oracle_map: &mut OracleMap,
                                 trade_token_map: &mut TradeTokenMap,
                                 state: &State) -> (BumpResult<u128>, BumpResult<u128>) {
    let order_margin;
    let order_margin_from_balance;

    let mut user_processor = UserProcessor { user: &mut user };

    if order.cross_margin {
        let available_value = user_processor.get_available_value(oracle_map, trade_token_map).unwrap();
        if available_value < 0i128 {
            let fix_order_margin_in_usd = order.order_size.cast::<i128>().unwrap().safe_add(available_value).unwrap().cast::<i128>().unwrap();
            validate!(fix_order_margin_in_usd > 0i128, BumpErrorCode::BalanceNotEnough.into());
            user.sub_order_hold_in_usd(order.order_size).unwrap();
            order.order_size = fix_order_margin_in_usd.cast::<u128>().unwrap();
        } else {
            user.sub_order_hold_in_usd(order.order_size).unwrap();
        }
        order_margin = cal_utils::usd_to_token_u(order.order_size, decimals, margin_token_price).unwrap();
        order_margin_from_balance = user.use_token(margin_token, order_margin, false).unwrap();
    } else {
        let order_margin_in_usd = cal_utils::token_to_usd_u(order.order_margin, decimals, margin_token_price).unwrap();
        validate!(order_margin_in_usd >= state.min_order_margin_usd, BumpErrorCode::AmountNotEnough.into());
        order_margin = order.order_margin;
        order_margin_from_balance = order.order_margin;
    }


    return (Ok(order_margin), Ok(order_margin_from_balance));
}


fn get_execution_price(oracle_map: &mut OracleMap, order: &mut UserOrder, index_token: Pubkey) -> BumpResult<u128> {
    let index_price = oracle_map.get_price_data(&index_token)?.price;

    if order.order_type.eq(&OrderType::MARKET) {
        if order.order_side.eq(&OrderSide::LONG) && index_price >= order.acceptable_price {
            return Err(BumpErrorCode::PriceIsNotAllowed.into());
        }
        if order.order_side.eq(&OrderSide::SHORT) && index_price <= order.acceptable_price {
            return Err(BumpErrorCode::PriceIsNotAllowed.into());
        }
        return Ok(index_price);
    }

    let long = OrderSide::LONG == order.order_side;
    if order.order_type.eq(&OrderType::LIMIT) || (order.order_type.eq(&OrderType::STOP) && order.stop_type.eq(&StopType::TakeProfit)) {
        if (long && order.trigger_price >= index_price) || (!long && order.trigger_price <= index_price) {
            return Ok(index_price);
        }
        return Err(BumpErrorCode::PriceIsNotAllowed.into());
    }

    if order.order_type.eq(&OrderType::STOP) && order.stop_type.eq(&StopType::StopLoss) {
        if (long && order.trigger_price <= index_price) || (!long && order.trigger_price >= index_price) {
            return Ok(index_price);
        }
    }

    Err(BumpErrorCode::PriceIsNotAllowed.into())
}

fn validate_execute_order(order: &mut UserOrder, market: &Market, pool: &Pool) -> BumpResult<()> {

    // token verify
    if order.margin_token != market.pool_mint_key {
        return Err(BumpErrorCode::TokenNotMatch.into());
    }

    if pool.pool_mint != order.margin_token {
        return Err(BumpErrorCode::TokenNotMatch.into());
    }

    if order.leverage > market.market_trade_config.max_leverage {
        return Err(BumpErrorCode::LeverageIsNotAllowed.into());
    }
    Ok(())
}
