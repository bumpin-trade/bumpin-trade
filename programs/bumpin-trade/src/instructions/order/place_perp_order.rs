use std::ops::{Deref, DerefMut};

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::calculator;
use crate::instructions::constraints::*;
use crate::math::safe_math::SafeMath;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor;
use crate::processor::position_processor::use_base_token;
use crate::state::infrastructure::user_order::{
    OrderSide, OrderStatus, OrderType, PositionSide, StopType, UserOrder,
};
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::utils::token;
use crate::{get_then_update_id, validate};

#[derive(Accounts)]
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

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct PlaceOrderParams {
    pub symbol: [u8; 32],
    pub size: u128,
    pub order_margin: u128,
    pub leverage: u32,
    pub trigger_price: u128,
    pub acceptable_price: u128,
    pub place_time: i64,
    pub is_portfolio_margin: bool,
    pub is_native_token: bool,
    pub order_side: OrderSide,
    pub position_side: PositionSide,
    pub order_type: OrderType,
    pub stop_type: StopType,
}

pub fn handle_place_order<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PlaceOrder<'c>>,
    order: PlaceOrderParams,
) -> Result<()> {
    msg!("============handle_place_order, order:{:?}", order);
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { trade_token_map, mut oracle_map, market_map, pool_map, vault_map } =
        load_maps(remaining_accounts)?;
    let market = market_map.get_mut_ref(&order.symbol)?;
    let mut user = ctx.accounts.user.load_mut()?;
    let pool = match use_base_token(&order.position_side, &order.order_side)? {
        true => pool_map.get_mut_ref(&market.pool_key)?,
        false => pool_map.get_mut_ref(&market.stable_pool_key)?,
    };
    let margin_token = match use_base_token(&order.position_side, &order.order_side)? {
        true => &market.pool_mint_key,
        false => &market.stable_pool_mint_key,
    };
    validate!(ctx.accounts.user_token_account.mint.eq(margin_token), BumpErrorCode::InvalidTokenAccount)?;

    let token_price = oracle_map
        .get_price_data(&market.index_mint_oracle)
        .map_err(|_e| BumpErrorCode::OracleNotFound)?
        .price;
    validate!(
        validate_place_order(
            &order,
            margin_token,
            &market,
            pool.deref(),
            &ctx.accounts.state,
            token_price
        )?,
        BumpErrorCode::InvalidParam
    )?;
    msg!("==========handle_place_order, validate_place_order");

    if order.position_side.eq(&PositionSide::INCREASE) && !order.is_portfolio_margin {
        //isolate order, transfer order_margin into pool
        token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            vault_map.get_account(&pool.pool_vault_key)?,
            &ctx.accounts.authority,
            order.order_margin,
        )?;
    }
    if order.position_side.eq(&PositionSide::INCREASE) && order.is_portfolio_margin {
        //hold usd
        user.deref_mut().add_order_hold_in_usd(order.order_margin)?;
    }

    if !user.make_order_is_allowed(
        order.symbol,
        order.is_portfolio_margin,
        use_base_token(&order.position_side, &order.order_side)?,
        ctx.program_id,
    )? {
        return Err(BumpErrorCode::OnlyOneTypeOrderAllowed.into());
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
        created_at: calculator::current_time(),
        status: OrderStatus::USING,
        ..Default::default()
    };
    if order.order_type.eq(&OrderType::MARKET) {
        //execute order immediately
        drop(market);
        drop(pool);
        let state_account = &ctx.accounts.state;
        let bump_signer_account_info = &ctx.accounts.bump_signer;
        let token_program = &ctx.accounts.token_program;
        position_processor::handle_execute_order(
            user.deref_mut(),
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
        )?;
    } else {
        //store order, wait to execute
        let next_index = user.next_usable_order_index()?;
        user.add_order(&user_order, next_index)?;
    }
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
