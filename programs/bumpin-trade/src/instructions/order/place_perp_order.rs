use std::iter::Peekable;
use std::slice::Iter;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::instructions::constraints::*;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::position_processor::{
    DecreasePositionParams, IncreasePositionParams, PositionProcessor,
};
use crate::processor::user_processor::UserProcessor;
use crate::state::infrastructure::user_order::{
    OrderSide, OrderStatus, OrderType, PositionSide, StopType, UserOrder,
};
use crate::state::infrastructure::user_position::PositionStatus;
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::utils::{pda, token};
use crate::{get_then_update_id, validate};

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
        constraint = pool.load() ?.pool_mint == market.load() ?.pool_mint
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        constraint = stable_pool.load() ?.pool_mint == market.load() ?.stable_pool_mint
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    pub market: AccountLoader<'info, Market>,

    pub state: Account<'info, State>,
    #[account(
        mut,
        constraint = pool_vault.mint.eq(& user_token_account.mint) || stable_pool_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pool_vault.mint == pool.load() ?.pool_mint
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = stable_pool_vault.mint == stable_pool.load() ?.pool_mint
    )]
    pub stable_pool_vault: Account<'info, TokenAccount>,

    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        constraint = trade_token_vault.mint == trade_token.load() ?.trade_token_vault
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,

    /// CHECK: ?
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

pub fn handle_place_order<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PlaceOrder>,
    order: PlaceOrderParams,
) -> Result<()> {
    let market = ctx.accounts.market.load()?;
    let mut user = ctx.accounts.user_account.load_mut()?;
    let pool = ctx.accounts.pool.load()?;
    let token = &ctx.accounts.margin_token;
    let remaining_accounts_iter: &mut Peekable<Iter<'info, AccountInfo<'info>>> =
        &mut ctx.remaining_accounts.iter().peekable();
    let AccountMaps { trade_token_map, mut oracle_map, .. } = load_maps(remaining_accounts_iter)?;
    let token_price = oracle_map.get_price_data(&token.mint).unwrap().price;
    validate!(
        validate_place_order(
            &order,
            &token.mint,
            &market,
            &pool,
            &ctx.accounts.state,
            token_price
        )?,
        BumpErrorCode::InvalidParam.into()
    )?;

    if order.position_side == PositionSide::INCREASE && !order.is_cross_margin {
        //isolate order, transfer order_margin into pool
        token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            if order.order_side.eq(&OrderSide::LONG) {
                &ctx.accounts.pool_vault
            } else {
                &ctx.accounts.stable_pool_vault
            },
            &ctx.accounts.authority,
            order.order_margin,
        )?;
    }

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
        let user_account_loader = &ctx.accounts.user_account;
        let margin_token_account = &ctx.accounts.margin_token;
        let pool_account_loader = &ctx.accounts.pool;
        let stable_pool_account_loader = &ctx.accounts.stable_pool;
        let market_account_loader = &ctx.accounts.market;
        let state_account = &ctx.accounts.state;
        let user_token_account = &ctx.accounts.user_token_account;
        let pool_vault_account = &ctx.accounts.pool_vault;
        let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
        let trade_token_loader = &ctx.accounts.trade_token;
        let trade_token_vault_account = &ctx.accounts.trade_token_vault;
        let bump_signer_account_info = &ctx.accounts.bump_signer;
        let token_program = &ctx.accounts.token_program;

        return handle_execute_order(
            user_account_loader,
            margin_token_account,
            pool_account_loader,
            stable_pool_account_loader,
            market_account_loader,
            state_account,
            user_token_account,
            pool_vault_account,
            stable_pool_vault_account,
            trade_token_loader,
            trade_token_vault_account,
            bump_signer_account_info,
            token_program,
            ctx.program_id,
            &trade_token_map,
            &mut oracle_map,
            &mut user_order,
            order_id,
            false,
        );
    } else {
        //store order, wait to execute
        let next_index = user.next_usable_order_index()?;
        user.add_order(user_order, next_index)?;
    }
    Ok(())
}

pub fn handle_execute_order<'info>(
    user_account_loader: &AccountLoader<'info, User>,
    margin_token_account: &Account<'info, TokenAccount>,
    pool_account_loader: &AccountLoader<'info, Pool>,
    stable_pool_account_loader: &AccountLoader<'info, Pool>,
    market_account_loader: &AccountLoader<'info, Market>,
    state_account: &Account<'info, State>,
    user_token_account: &Account<'info, TokenAccount>,
    pool_vault_account: &Account<'info, TokenAccount>,
    stable_pool_vault_account: &Account<'info, TokenAccount>,
    trade_token_loader: &AccountLoader<'info, TradeToken>,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    program_id: &Pubkey,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    user_order: &mut UserOrder,
    order_id: u128,
    execute_from_remote: bool,
) -> Result<()> {
    let user = user_account_loader.load().unwrap();
    let margin_token = margin_token_account;
    let market = &mut market_account_loader.load_mut().unwrap();
    let trade_token = trade_token_loader.load().unwrap();

    let order = if execute_from_remote { user.find_ref_order_by_id(order_id)? } else { user_order };
    let next_use_index = user.next_usable_position_index()?;
    let user_authority = user.authority;

    let stake_token_pool = &mut pool_account_loader.load_mut().unwrap();
    let stable_pool = &mut stable_pool_account_loader.load_mut().unwrap();
    let pool = if order.order_side.eq(&OrderSide::LONG) { stake_token_pool } else { stable_pool };

    //validate order
    validate_execute_order(&order, &market)?;
    let is_long = OrderSide::LONG == order.order_side;
    let execute_price =
        get_execution_price(oracle_map.get_price_data(&market.index_mint).unwrap().price, &order)?;

    let user = &mut user_account_loader.load_mut().unwrap();
    let position = user.find_position_by_seed(
        &user_authority,
        market.symbol,
        order.cross_margin,
        program_id,
    )?;

    //update funding_fee_rate and borrowing_fee_rate
    let mut market_processor = MarketProcessor { market };
    market_processor.update_market_funding_fee_rate(state_account, oracle_map)?;
    let mut pool_processor = PoolProcessor { pool };
    pool_processor.update_pool_borrowing_fee_rate()?;

    let user = &mut user_account_loader.load_mut().unwrap();
    let market = market_account_loader.load().unwrap();
    //do execute order and change position, cal fee....
    match order.position_side {
        PositionSide::NONE => Err(BumpErrorCode::PositionSideNotSupport),
        PositionSide::INCREASE => Ok({
            let margin_token_price;
            if market.index_mint == margin_token.mint {
                margin_token_price = execute_price;
            } else {
                margin_token_price = oracle_map.get_price_data(&margin_token.mint)?.price;
            }

            let (order_margin, order_margin_from_balance) = execute_increase_order_margin(
                order,
                &margin_token.mint,
                trade_token.decimals,
                user,
                margin_token_price,
                oracle_map,
                trade_token_map,
                state_account,
            )?;

            if position.position_size == 0u128 && position.status.eq(&PositionStatus::INIT) {
                if user.has_other_order(order.order_id)?
                    && user.get_order_leverage(
                        order.symbol,
                        order.order_side,
                        order.cross_margin,
                        order.leverage,
                    )? == order.leverage
                {
                    return Err(BumpErrorCode::AmountNotEnough.into());
                }

                position.set_position_key(pda::generate_position_key(
                    &user.authority,
                    order.symbol,
                    order.cross_margin,
                    program_id,
                )?)?;
                position.set_authority(user.authority)?;
                position.set_index_mint(market.index_mint)?;
                position.set_symbol(order.symbol)?;
                position.set_margin_mint(order.margin_token)?;
                position.set_leverage(order.leverage)?;
                position.set_is_long(order.order_side.eq(&OrderSide::LONG))?;
                position.set_cross_margin(order.cross_margin)?;
                position.set_status(PositionStatus::USING)?;
                user.add_position(position, next_use_index)?;
            } else if position.leverage != order.leverage {
                return Err(BumpErrorCode::LeverageIsNotAllowed.into());
            }

            let mut position_processor = PositionProcessor { position };
            position_processor.increase_position(
                IncreasePositionParams {
                    margin_token: order.margin_token,
                    increase_margin: order_margin,
                    increase_margin_from_balance: order_margin_from_balance,
                    margin_token_price,
                    index_token_price: execute_price,
                    leverage: order.leverage,
                    is_long,
                    is_cross_margin: order.cross_margin,
                    decimals: trade_token.decimals,
                },
                user_account_loader,
                pool_account_loader,
                stable_pool_account_loader,
                market_account_loader,
                state_account,
                trade_token_loader,
            )?;
        }),

        PositionSide::DECREASE => Ok({
            //decrease
            if position.position_size == 0u128 || position.status.eq(&PositionStatus::INIT) {
                return Err(BumpErrorCode::InvalidParam.into());
            }
            if position.is_long == is_long {
                return Err(BumpErrorCode::InvalidParam.into());
            }

            let mut position_processor = PositionProcessor { position };
            position_processor.decrease_position(
                DecreasePositionParams {
                    order_id,
                    is_liquidation: false,
                    is_cross_margin: false,
                    margin_token: order.margin_token,
                    decrease_size: if position_processor.position.position_size < order.order_size {
                        position_processor.position.position_size
                    } else {
                        order.order_size
                    },
                    execute_price,
                },
                user_account_loader,
                pool_account_loader,
                stable_pool_account_loader,
                market_account_loader,
                state_account,
                Some(user_token_account),
                if position_processor.position.is_long {
                    pool_vault_account
                } else {
                    stable_pool_vault_account
                },
                trade_token_loader,
                trade_token_vault_account,
                bump_signer,
                token_program,
                program_id,
                oracle_map,
            )?
        }),
    }?;
    //delete order
    user.delete_order(order_id)?;
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
    let mut res = true;
    match order.order_type {
        OrderType::NONE => res = false,
        OrderType::MARKET => {},
        OrderType::LIMIT => {},
        OrderType::STOP => {},
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
    if order.order_type.eq(&OrderType::STOP)
        && (order.stop_type.eq(&StopType::NONE) || order.trigger_price == 0u128)
    {
        res = false;
    }
    if order.position_side.eq(&PositionSide::INCREASE) {
        if order.order_margin == 0u128 {
            res = false;
        }
        if order.order_side.eq(&OrderSide::LONG) && !token.eq(&market.pool_mint) {
            res = false;
        }

        if order.order_side.eq(&OrderSide::LONG) && !market.pool_mint.eq(&pool.pool_mint) {
            res = false;
        }

        if order.order_side.eq(&OrderSide::SHORT) && !pool.pool_mint.eq(&token) {
            res = false;
        }

        if order.order_side.eq(&OrderSide::SHORT) && !market.stable_pool_mint.eq(&pool.pool_mint) {
            res = false;
        }

        if order.is_cross_margin && order.order_margin < state.min_order_margin_usd {
            res = false;
        }
        if !order.is_cross_margin
            && order.order_margin.safe_mul(token_price)? < state.min_order_margin_usd
        {
            res = false;
        }
    }
    Ok(res)
}

fn execute_increase_order_margin(
    order: &UserOrder,
    margin_token: &Pubkey,
    decimals: u8,
    user: &mut User,
    margin_token_price: u128,
    oracle_map: &mut OracleMap,
    trade_token_map: &TradeTokenMap,
    state: &Account<State>,
) -> BumpResult<(u128, u128)> {
    let order_margin;
    let order_margin_from_balance;

    let mut user_processor = UserProcessor { user };

    if order.cross_margin {
        let available_value = user_processor.get_available_value(oracle_map, trade_token_map)?;
        let order_margin_temp;
        if available_value < 0i128 {
            let fix_order_margin_in_usd =
                order.order_size.cast::<i128>()?.safe_add(available_value)?.cast::<i128>()?;
            validate!(fix_order_margin_in_usd > 0i128, BumpErrorCode::BalanceNotEnough.into())?;
            user.sub_order_hold_in_usd(order.order_size).unwrap();
            order_margin_temp = fix_order_margin_in_usd.abs().cast::<u128>()?;
        } else {
            order_margin_temp = order.order_size;
            user.sub_order_hold_in_usd(order.order_size)?;
        }
        order_margin = cal_utils::usd_to_token_u(order_margin_temp, decimals, margin_token_price)?;
        order_margin_from_balance = user.use_token(margin_token, order_margin, false)?;
    } else {
        let order_margin_in_usd =
            cal_utils::token_to_usd_u(order.order_margin, decimals, margin_token_price)?;
        validate!(
            order_margin_in_usd >= state.min_order_margin_usd,
            BumpErrorCode::AmountNotEnough.into()
        )?;
        order_margin = order.order_margin;
        order_margin_from_balance = order.order_margin;
    }

    Ok((order_margin, order_margin_from_balance))
}

fn get_execution_price(index_price: u128, order: &UserOrder) -> BumpResult<u128> {
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
    if order.order_type.eq(&OrderType::LIMIT)
        || (order.order_type.eq(&OrderType::STOP) && order.stop_type.eq(&StopType::TakeProfit))
    {
        if (long && order.trigger_price >= index_price)
            || (!long && order.trigger_price <= index_price)
        {
            return Ok(index_price);
        }
        return Err(BumpErrorCode::PriceIsNotAllowed.into());
    }

    if order.order_type.eq(&OrderType::STOP) && order.stop_type.eq(&StopType::StopLoss) {
        if (long && order.trigger_price <= index_price)
            || (!long && order.trigger_price >= index_price)
        {
            return Ok(index_price);
        }
    }

    Err(BumpErrorCode::PriceIsNotAllowed.into())
}

fn validate_execute_order(order: &UserOrder, market: &Market) -> BumpResult<()> {
    // token verify
    if order.position_side.eq(&PositionSide::INCREASE) {
        if order.order_side.eq(&OrderSide::LONG) && order.margin_token != market.pool_mint {
            return Err(BumpErrorCode::TokenNotMatch.into());
        }

        if order.order_side.eq(&OrderSide::SHORT) && order.margin_token != market.stable_pool_mint {
            return Err(BumpErrorCode::TokenNotMatch.into());
        }
    }

    if order.leverage > market.market_trade_config.max_leverage {
        return Err(BumpErrorCode::LeverageIsNotAllowed.into());
    }
    Ok(())
}
