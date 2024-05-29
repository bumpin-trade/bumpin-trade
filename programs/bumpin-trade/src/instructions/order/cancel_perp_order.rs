use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;
use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::load_mut;
use crate::state::user::User;
use crate::state::infrastructure::user_order::{OrderStatus, PositionSide};
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::utils::token;
use anchor_lang::prelude::ErrorCode;

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

    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    
    pub state: Account<'info, State>,
}


pub fn handle_cancel_order(ctx: Context<CancelOrderCtx>, order_id: u128, reason_code: u128) -> anchor_lang::Result<()> {
    let mut user = load_mut!(ctx.accounts.user_account)?;
    let order = user.find_order_by_id(order_id)?;
    if order.status.eq(&OrderStatus::INIT) {
        return Err(BumpErrorCode::InvalidParam.into());
    }
    user.delete_order(order_id)?;
    if order.position_side.eq(&PositionSide::INCREASE) && order.cross_margin {
        user.sub_order_hold_in_usd(order.order_size)
    } else if order.position_side.eq(&PositionSide::INCREASE) && !order.cross_margin {
        token::send_from_program_vault(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.user_token_account,
            &ctx.accounts.bump_signer,
            ctx.accounts.state.load()?.bump_signer_nonce,
            order.order_margin,
        )?;
    }
    Ok(())
}