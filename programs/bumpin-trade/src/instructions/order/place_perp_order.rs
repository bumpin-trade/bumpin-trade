use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::instructions::constraints::*;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor::DecreasePositionParams;
use crate::processor::user_processor::UserProcessor;
use crate::processor::{fee_processor, position_processor};
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
use crate::state::user::{User, UserTokenUpdateReason};
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
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,

    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market", order.market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        constraint = market.load() ?.pool_mint.eq(& margin_token.key()) || market.load() ?.stable_pool_mint.eq(& margin_token.key())
    )]
    pub margin_token: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [b"pool", order.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.pool_key.eq(& market.load() ?.pool_key)
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), order.pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.pool_mint,
        token::authority = bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool", order.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.pool_key.eq(& market.load() ?.stable_pool_key)
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), order.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = stable_pool.load() ?.pool_mint,
        token::authority = bump_signer
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token", order.trade_token_index.to_le_bytes().as_ref()],
        bump,
        constraint = trade_token.load() ?.mint.eq(& user_token_account.mint),
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), order.trade_token_index.to_le_bytes().as_ref()],
        bump,
        token::mint = trade_token.load() ?.mint,
        token::authority = bump_signer,
        constraint = trade_token_vault.key() == trade_token.load() ?.trade_token_vault
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token", order.index_trade_token_index.to_le_bytes().as_ref()],
        bump,
        constraint = index_trade_token.load() ?.mint.eq(& market.load() ?.index_mint),
    )]
    pub index_trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        constraint = pool_vault.mint.eq(& user_token_account.mint) || stable_pool_vault.mint.eq(& user_token_account.mint),
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
    pub pool_index: u16,
    pub stable_pool_index: u16,
    pub market_index: u16,
    pub trade_token_index: u16,
    pub index_trade_token_index: u16,
}

pub fn handle_place_order<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PlaceOrder>,
    order: PlaceOrderParams,
) -> Result<()> {
    let market = ctx.accounts.market.load()?;
    let mut user = ctx.accounts.user.load_mut()?;
    let pool = ctx.accounts.pool.load()?;
    let stable_pool = ctx.accounts.stable_pool.load()?;
    let token = &ctx.accounts.margin_token;
    let trade_token = &ctx.accounts.trade_token.load()?;
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { trade_token_map, mut oracle_map, .. } =
        load_maps(remaining_accounts, &ctx.accounts.state.admin)?;
    let token_price = oracle_map.get_price_data(&trade_token.oracle)?.price;
    validate!(
        validate_place_order(
            &order,
            &token.key(),
            &market,
            if order.order_side.eq(&OrderSide::LONG) { &pool } else { &stable_pool },
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
    if order.position_side.eq(&PositionSide::INCREASE) && order.is_cross_margin {
        //hold usd
        user.add_order_hold_in_usd(order.order_margin)?;
    }

    if user.has_other_short_order(order.symbol, token.key(), order.is_cross_margin)? {
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
        cross_margin: order.is_cross_margin,
        margin_mint: token.key(),
        order_margin: order.order_margin,
        leverage: order.leverage,
        order_size: order.size,
        trigger_price: order.trigger_price,
        acceptable_price: order.acceptable_price,
        time: cal_utils::current_time(),
        status: OrderStatus::USING,
        padding: [0u8; 10],
    };

    if order.order_type.eq(&OrderType::MARKET) {
        drop(user);
        //execute order
        let user_account_loader = &ctx.accounts.user;
        let margin_token_account = &ctx.accounts.margin_token;
        let pool_account_loader = &ctx.accounts.pool;
        let stable_pool_account_loader = &ctx.accounts.stable_pool;
        let market_account_loader = &ctx.accounts.market;
        let state_account = &ctx.accounts.state;
        let user_token_account = &ctx.accounts.user_token_account;
        let pool_vault_account = &ctx.accounts.pool_vault;
        let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
        let trade_token_loader = &ctx.accounts.trade_token;
        let index_trade_token_loader = &ctx.accounts.index_trade_token;
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
            index_trade_token_loader,
            trade_token_vault_account,
            bump_signer_account_info,
            token_program,
            ctx.program_id,
            &trade_token_map,
            &mut oracle_map,
            &user_order,
            order_id,
            false,
        );
    } else {
        //store order, wait to execute
        let next_index = user.next_usable_order_index()?;
        user.add_order(&user_order, next_index)?;
    }

    Ok(())
}

pub fn handle_execute_order<'info>(
    user_account_loader: &AccountLoader<'info, User>,
    margin_token_account: &Account<'info, Mint>,
    pool_account_loader: &AccountLoader<'info, Pool>,
    stable_pool_account_loader: &AccountLoader<'info, Pool>,
    market_account_loader: &AccountLoader<'info, Market>,
    state_account: &Account<'info, State>,
    user_token_account: &Account<'info, TokenAccount>,
    pool_vault_account: &Account<'info, TokenAccount>,
    stable_pool_vault_account: &Account<'info, TokenAccount>,
    trade_token_loader: &AccountLoader<'info, TradeToken>,
    index_trade_token_loader: &AccountLoader<'info, TradeToken>,
    trade_token_vault_account: &Account<'info, TokenAccount>,
    bump_signer: &AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    program_id: &Pubkey,
    trade_token_map: &TradeTokenMap,
    oracle_map: &mut OracleMap,
    user_order: &UserOrder,
    order_id: u128,
    execute_from_remote: bool,
) -> Result<()> {
    let user = user_account_loader.load_mut()?;

    let margin_token = margin_token_account;
    let mut market = market_account_loader.load_mut()?;
    let trade_token = trade_token_loader.load()?;
    let index_trade_token = index_trade_token_loader.load()?;
    let mut stake_token_pool =
        pool_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadPoolData)?;
    let stable_pool =
        stable_pool_account_loader.load_mut().map_err(|_| BumpErrorCode::CouldNotLoadPoolData)?;

    let order = if execute_from_remote {
        let user_order_index = user.get_user_order_index(order_id)?;
        &user.user_orders[user_order_index]
    } else {
        user_order
    }
    .clone();

    let user_key = user.user_key;

    //validate order
    validate_execute_order(&order, &market)?;
    let is_long = OrderSide::LONG == order.order_side;
    let execute_price = get_execution_price(
        oracle_map.get_price_data(&index_trade_token.oracle).unwrap().price,
        &order,
    )?;

    //update funding_fee_rate and borrowing_fee_rate
    let mut market_processor = MarketProcessor { market: &mut market };
    market_processor.update_market_funding_fee_rate(
        state_account,
        oracle_map.get_price_data(&trade_token.oracle)?.price,
    )?;

    let mut base_token_pool = pool_account_loader.load_mut()?;
    let mut stable_pool = stable_pool_account_loader.load_mut()?;

    if order.order_side.eq(&OrderSide::LONG) {
        base_token_pool.update_pool_borrowing_fee_rate()?;
    } else {
        stable_pool.update_pool_borrowing_fee_rate()?;
    }
    let position_key =
        pda::generate_position_key(&user_key, market.symbol, order.cross_margin, program_id)?;

    // drop(user);
    //do execute order and change position
    match order.position_side {
        PositionSide::NONE => Err(BumpErrorCode::PositionSideNotSupport),
        PositionSide::INCREASE => Ok({
            let mut user = user_account_loader.load_mut()?;
            let margin_token_price;
            if market.index_mint.eq(&margin_token.key()) {
                margin_token_price = execute_price;
            } else {
                margin_token_price = oracle_map.get_price_data(&trade_token.oracle)?.price;
            }
            //calculate real order_margin with validation
            let (order_margin, order_margin_from_balance) = execute_increase_order_margin(
                user_token_account.to_account_info().key,
                &order,
                &margin_token.key(),
                trade_token.decimals,
                &mut user,
                margin_token_price,
                oracle_map,
                trade_token_map,
                state_account,
            )?;

            //collect open fee
            let mut base_token_pool = pool_account_loader.load_mut()?;
            let mut stable_pool = stable_pool_account_loader.load_mut()?;
            let fee = if order.order_side.eq(&OrderSide::LONG) {
                fee_processor::collect_long_open_position_fee(
                    &market,
                    &mut base_token_pool,
                    order_margin,
                    order.cross_margin,
                )?
            } else {
                fee_processor::collect_short_open_position_fee(
                    &market,
                    &mut base_token_pool,
                    &mut stable_pool,
                    state_account,
                    order_margin,
                    order.cross_margin,
                )?
            };

            //record fee in user
            if order.cross_margin {
                let trade_token = &mut trade_token_loader
                    .load_mut()
                    .map_err(|_| BumpErrorCode::CouldNotLoadTradeTokenData)?;
                user.un_use_token(&order.margin_mint, fee)?;
                user.sub_token_with_liability(
                    &order.margin_mint,
                    trade_token,
                    fee,
                    &UserTokenUpdateReason::SettleFee,
                )?;
            }

            drop(user);
            drop(market);
            // drop(pool);
            drop(base_token_pool);
            drop(stable_pool);

            //increase position
            position_processor::increase_position(
                user_account_loader,
                pool_account_loader,
                stable_pool_account_loader,
                market_account_loader,
                trade_token_loader,
                program_id,
                &order,
                order_margin,
                order_margin_from_balance,
                execute_price,
                margin_token_price,
                fee,
            )?
        }),

        PositionSide::DECREASE => Ok({
            let position_side = { position!(&user.user_positions, &position_key)?.is_long };
            //decrease
            let mut user = user_account_loader.load_mut()?;
            let position = user.get_user_position_ref(&position_key)?.clone();
            if position.position_size == 0u128 || position.status.eq(&PositionStatus::INIT) {
                return Err(BumpErrorCode::InvalidParam.into());
            }
            if position.is_long == is_long {
                return Err(BumpErrorCode::InvalidParam.into());
            }
            // drop(market);
            // drop(pool);

            position_processor::decrease_position1(
                DecreasePositionParams {
                    order_id,
                    is_liquidation: false,
                    is_cross_margin: false,
                    margin_token: order.margin_mint,
                    decrease_size: order.order_size,
                    execute_price,
                },
                user.deref_mut(),
                market.deref_mut(),
                stake_token_pool.deref_mut(),
                stable_pool.deref_mut(),
                state_account,
                Some(user_token_account),
                if position_side { pool_vault_account } else { stable_pool_vault_account },
                trade_token,
                trade_token_vault_account,
                bump_signer,
                token_program,
                oracle_map,
                &position_key,
            )?
        }),
    }?;
    let mut user = user_account_loader.load_mut()?;
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
                } else if order.order_side.eq(&OrderSide::LONG) && !token.eq(&market.pool_mint) {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::LONG)
                    && !market.pool_mint.eq(&pool.pool_mint)
                {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::SHORT) && !pool.pool_mint.eq(&token) {
                    Ok(false)
                } else if order.order_side.eq(&OrderSide::SHORT)
                    && !market.stable_pool_mint.eq(&pool.pool_mint)
                {
                    Ok(false)
                } else if order.is_cross_margin && order.order_margin < state.min_order_margin_usd {
                    Ok(false)
                } else if !order.is_cross_margin
                    && order.order_margin.safe_mul(token_price)? < state.min_order_margin_usd
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
        order_margin_from_balance =
            user.use_token(margin_token, order_margin, user_token_account_key, false)?;
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
        if order.order_side.eq(&OrderSide::LONG) && order.margin_mint != market.pool_mint {
            return Err(BumpErrorCode::TokenNotMatch.into());
        }

        if order.order_side.eq(&OrderSide::SHORT) && order.margin_mint != market.stable_pool_mint {
            return Err(BumpErrorCode::TokenNotMatch.into());
        }
    }

    if order.leverage > market.market_trade_config.max_leverage {
        return Err(BumpErrorCode::LeverageIsNotAllowed.into());
    }
    Ok(())
}
