use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;

use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::processor::user_processor::UserProcessor;
use crate::state::infrastructure::user_order::OrderStatus;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;

#[derive(Accounts)]
pub struct CancelOrderCtx<'info> {
    #[account(
        mut,
        constraint = can_sign_for_user(& user_account, & authority) ?
    )]
    pub user_account: AccountLoader<'info, User>,

    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = & pool_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub pool_vault: Account<'info, TokenAccount>,

    pub trade_token: AccountLoader<'info, TradeToken>,

    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,

    pub state: Account<'info, State>,
}

pub fn handle_cancel_order(ctx: Context<CancelOrderCtx>, order_id: u128) -> Result<()> {
    let user = ctx.accounts.user_account.load().unwrap();
    let order = user.find_ref_order_by_id(order_id)?;
    if order.status.eq(&OrderStatus::INIT) {
        return Err(BumpErrorCode::InvalidParam.into());
    }

    let user = &mut ctx.accounts.user_account.load_mut().unwrap();
    let mut user_processor = UserProcessor { user };
    user_processor.cancel_order(
        order,
        &ctx.accounts.token_program,
        &ctx.accounts.pool_vault,
        &ctx.accounts.user_token_account,
        &ctx.accounts.bump_signer,
        &ctx.accounts.state,
    )?;
    Ok(())
}
