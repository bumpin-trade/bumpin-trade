use std::ops::{Deref, DerefMut};

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::instructions::place::validate_place_order;
use crate::instructions::{calculator, PlaceOrderParams};
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor;
use crate::processor::position_processor::use_base_token;
use crate::state::infrastructure::user_order::{OrderStatus, OrderType, PositionSide, UserOrder};
use crate::state::state::State;
use crate::state::user::User;
use crate::utils::token;
use crate::{get_then_update_id, validate};

#[derive(Accounts)]
pub struct WalletPlaceOrder<'info> {
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

pub fn handle_place_wallet_order<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, WalletPlaceOrder<'c>>,
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
    validate!(
        ctx.accounts.user_token_account.mint.eq(margin_token),
        BumpErrorCode::InvalidTokenAccount
    )?;

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
        user_token_account: ctx.accounts.user_token_account.key(),
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
            Some(&ctx.accounts.user_token_account),
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
