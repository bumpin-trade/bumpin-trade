use anchor_lang::prelude::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::processor::user_processor::UserProcessor;
use crate::state::infrastructure::user_order::OrderStatus;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    _pool_index: u16,
)]
pub struct CancelOrderCtx<'info> {
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
        seeds = [b"pool", _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.pool_mint,
        token::authority = bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = pool_vault.mint.eq(& user_token_account.mint),
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

pub fn handle_cancel_order(
    ctx: Context<CancelOrderCtx>,
    order_id: u128,
    _pool_index: u16,
) -> Result<()> {
    let user = ctx.accounts.user.load().unwrap();
    let order = user.find_ref_order_by_id(order_id)?;
    if order.status.eq(&OrderStatus::INIT) {
        return Err(BumpErrorCode::InvalidParam.into());
    }

    //validate pool is correct
    validate!(
        order.margin_mint.eq(&ctx.accounts.pool.load()?.pool_mint),
        BumpErrorCode::InvalidParam
    )?;

    let user = &mut ctx.accounts.user.load_mut().unwrap();
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
