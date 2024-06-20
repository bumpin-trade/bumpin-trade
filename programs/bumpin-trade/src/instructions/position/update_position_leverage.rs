use crate::errors::BumpErrorCode;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor::PositionProcessor;
use crate::processor::user_processor::UserProcessor;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::utils::pda;
use crate::{can_sign_for_user, validate};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Accounts)]
pub struct UpdatePositionLeverage<'info> {
    #[account(
        mut,
        constraint = can_sign_for_user(& user_account, & authority) ?
    )]
    pub user_account: AccountLoader<'info, User>,
    pub authority: Signer<'info>,
    pub trade_token: AccountLoader<'info, TradeToken>,
    pub pool: AccountLoader<'info, Pool>,
    pub state: Box<Account<'info, State>>,
    pub market: AccountLoader<'info, Market>,
    #[account(
        mut,
        constraint = & pool_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    pub pool_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UpdatePositionLeverageParams {
    pub symbol: [u8; 32],
    pub is_long: bool,
    pub is_cross_margin: bool,
    pub leverage: u128,
    pub add_margin_amount: u128,
}

pub fn handle_update_position_leverage<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, UpdatePositionLeverage>,
    params: UpdatePositionLeverageParams,
) -> anchor_lang::Result<()> {
    let user_mut = &mut ctx.accounts.user_account.load_mut()?;
    let trade_token = ctx.accounts.trade_token.load_mut()?;

    let remaining_accounts: &mut Peekable<Iter<'info, AccountInfo<'info>>> =
        &mut ctx.remaining_accounts.iter().peekable();
    let AccountMaps { trade_token_map, mut oracle_map, .. } = load_maps(remaining_accounts)?;

    let market = ctx.accounts.market.load_mut()?;
    validate!(
        params.leverage <= market.market_trade_config.max_leverage,
        BumpErrorCode::AmountNotEnough.into()
    )?;

    let user_processor = UserProcessor { user: user_mut };
    let position_key = pda::generate_position_key(
        &user_processor.user.authority,
        params.symbol,
        params.is_cross_margin,
        &ctx.program_id,
    )?;
    let position_mut = user_processor.user.find_position_mut_by_key(&position_key)?;
    let mut position_processor = PositionProcessor { position: position_mut };
    validate!(
        position_processor.position.leverage != params.leverage,
        BumpErrorCode::AmountNotEnough.into()
    )?;

    let token_price = oracle_map.get_price_data(&trade_token.mint)?.price;

    position_processor.update_leverage(
        token_price,
        params,
        position_key,
        &ctx.accounts.user_account,
        &ctx.accounts.authority,
        &ctx.accounts.trade_token,
        &ctx.accounts.pool,
        &ctx.accounts.state,
        &ctx.accounts.market,
        &ctx.accounts.user_token_account,
        &ctx.accounts.pool_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
        &trade_token_map,
        &mut oracle_map,
    )?;
    Ok(())
}
