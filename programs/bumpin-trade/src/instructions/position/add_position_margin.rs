use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::pubkey::Pubkey;

use crate::constraints::*;
use crate::errors::{BumpErrorCode};
use crate::processor::position_processor::PositionProcessor;
use crate::state::market::Market;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::utils::token;
use crate::validate;
use solana_program::msg;

#[derive(Accounts)]
pub struct AddPositionMargin<'info> {
    #[account(
        mut,
        constraint = can_sign_for_user(& user_account, & authority) ?
    )]
    pub user_account: AccountLoader<'info, User>,
    pub authority: Signer<'info>,
    pub trade_token: AccountLoader<'info, TradeToken>,
    pub pool: AccountLoader<'info, Pool>,
    pub state: AccountLoader<'info, State>,
    pub market: AccountLoader<'info, Market>,
    #[account(
        mut,
        constraint = & pool_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub pool_vault: Account<'info, TokenAccount>,
    pub bump_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UpdatePositionMarginParams {
    pub position_key: Pubkey,
    pub is_add: bool,
    pub update_margin_amount: u128,
}

pub fn handle_add_position_margin(ctx: Context<AddPositionMargin>, params: UpdatePositionMarginParams) -> anchor_lang::Result<()> {
    validate!(params.update_margin_amount > 0u128, BumpErrorCode::AmountNotEnough.into());
    let mut user = ctx.accounts.user_account.load_mut()?;
    let trade_token = ctx.accounts.trade_token.load_mut()?;
    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
    let mut oracle_map = OracleMap::load(remaining_accounts_iter)?;
    let mut position = user.find_position_mut_by_key(&params.position_key)?;
    let mut position_processor = PositionProcessor { position: &mut position };
    let mut pool = ctx.accounts.pool.load_mut()?;
    let state = ctx.accounts.state.load_mut()?;
    let market = ctx.accounts.market.load_mut()?;
    validate!(position.cross_margin, BumpErrorCode::AmountNotEnough.into());
    if params.is_add {
        token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            &ctx.accounts.pool_vault,
            &ctx.accounts.authority,
            params.update_margin_amount,
        )?;
    }
    validate!(trade_token.mint.eq(&position.margin_mint), BumpErrorCode::AmountNotEnough.into());

    if params.is_add {
        position_processor.execute_add_position_margin(&params, &trade_token, &mut oracle_map, &mut pool)?;
    } else {
        let reduce_margin_amount = position_processor.execute_reduce_position_margin(&params, true, &trade_token, &mut oracle_map, &mut pool, &market, &state)?;
        token::send_from_program_vault(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.user_token_account,
            &ctx.accounts.bump_signer,
            ctx.accounts.state.load()?.bump_signer_nonce,
            reduce_margin_amount,
        )?;
    }
    user.update_all_orders_leverage(position.leverage, position.symbol, &position.margin_mint, position.is_long, position.cross_margin)?;
    Ok(())
}