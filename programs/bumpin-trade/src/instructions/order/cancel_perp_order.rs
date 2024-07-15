use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::state::infrastructure::user_order::OrderStatus;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::validate;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

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
        seeds = [b"user", authority.key().as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ? && is_normal(& user) ?,
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
        token::mint = pool.load() ?.mint_key,
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

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_cancel_order(
    ctx: Context<CancelOrderCtx>,
    order_id: u64,
    _pool_index: u16,
) -> Result<()> {
    let user = ctx.accounts.user.load()?;
    let order = user.get_user_order_ref(order_id)?;
    if order.status.eq(&OrderStatus::INIT) {
        return Err(BumpErrorCode::InvalidParam.into());
    }

    //validate pool is correct
    validate!(
        order.margin_mint_key.eq(&ctx.accounts.pool.load()?.mint_key),
        BumpErrorCode::InvalidParam
    )?;

    let mut user =
        ctx.accounts.user.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadUserData)?;
    user.cancel_order(
        order,
        &ctx.accounts.token_program,
        &ctx.accounts.pool_vault,
        &ctx.accounts.user_token_account,
        &ctx.accounts.bump_signer,
        &ctx.accounts.state,
    )?;
    Ok(())
}
