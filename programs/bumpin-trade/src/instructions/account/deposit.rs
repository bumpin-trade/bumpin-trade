use crate::errors::BumpErrorCode::CouldNotFindUserToken;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;

use crate::instructions::constraints::*;
use crate::math::safe_math::SafeMath;
use crate::processor::user_processor::UserProcessor;
use crate::state::infrastructure::user_token::{UserToken, UserTokenStatus};
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::utils::token;

#[derive(Accounts)]
#[instruction(token_index: u16)]
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
        seeds = [b"trade_token", token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault", token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle_deposit(ctx: Context<Deposit>, token_index: u16, amount: u128) -> Result<()> {
    msg!("Token index: {}", token_index);
    let user = &mut ctx.accounts.user.load_mut()?;
    let trade_token = ctx.accounts.trade_token.load_mut()?;
    // msg!("User Token Account: {:?}", &ctx.accounts.user_token_account);
    token::receive(
        &ctx.accounts.token_program,
        &ctx.accounts.user_token_account,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.authority,
        amount,
    )?;
    ctx.accounts.trade_token_vault.reload()?;

    let user_token_option = user.get_user_token_mut(&ctx.accounts.trade_token_vault.mint)?;
    let user_token = match user_token_option {
        None => {
            let index = user.next_usable_user_token_index()?;
            //init user_token
            let new_token = &mut UserToken {
                user_token_status: UserTokenStatus::USING,
                token_mint: trade_token.mint,
                amount: 0,
                used_amount: 0,
                liability: 0,
            };
            user.add_user_token(new_token, index)?;
            user.get_user_token_mut(&trade_token.mint)?.ok_or(CouldNotFindUserToken)?
        },
        Some(exist_user_token) => exist_user_token,
    };

    user_token.add_token_amount(amount)?;
    trade_token.add_token(amount)?;

    let repay_amount = user_token.repay_liability()?;
    trade_token.sub_liability(repay_amount)?;
    if amount > repay_amount {
        let left_amount = amount.safe_sub(repay_amount)?;

        let mut user_processor = UserProcessor { user };
        user_processor.update_cross_position_balance(
            &ctx.accounts.user_token_account.mint,
            left_amount,
            true,
        )?;
        drop(user_processor);
    }
    Ok(())
}
