use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;

use crate::instructions::constraints::*;
use crate::math::safe_math::SafeMath;
use crate::processor::user_processor::UserProcessor;
use crate::state::user::User;
use crate::utils::token;

#[derive(Accounts)]
#[instruction(token_index: u16,)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"user", authority.key.as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = & trade_token_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"trade_token_vault", token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle_deposit(ctx: Context<Deposit>, amount: u128) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;

    token::receive(
        &ctx.accounts.token_program,
        &ctx.accounts.user_token_account,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.authority,
        amount,
    )?;
    ctx.accounts.trade_token_vault.reload()?;

    let user_token = user.get_user_token_mut(&ctx.accounts.trade_token_vault.mint.key())?;
    user_token.add_token_amount(amount)?;

    let repay_amount = user_token.repay_liability(amount)?;
    if amount > repay_amount {
        let left_amount = amount
            .safe_sub(repay_amount)?;

        let mut user_processor = UserProcessor { user };
        user_processor.update_cross_position_balance(&ctx.accounts.user_token_account.mint,
                                                     left_amount, true)?;
    }
    Ok(())
}