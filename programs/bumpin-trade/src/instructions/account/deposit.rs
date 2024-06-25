use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode::CouldNotFindUserToken;
use crate::instructions::constraints::*;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::{DepositEvent, UserTokenBalanceUpdateEvent};
use crate::state::infrastructure::user_token::UserToken;
use crate::state::trade_token::TradeToken;
use crate::state::user::{User, UserCrossPosition, UserTokenUpdateReason};
use crate::utils::token;

#[derive(Accounts)]
#[instruction(_token_index: u16)]
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
        seeds = [b"trade_token", _token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault", _token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handle_deposit(ctx: Context<Deposit>, _token_index: u16, amount: u128) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let user_key = user.user_key;
    let trade_token = &mut ctx.accounts.trade_token.load_mut()?;
    let token_mint = trade_token.mint;

    token::receive(
        &ctx.accounts.token_program,
        &ctx.accounts.user_token_account,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.authority,
        amount,
    )?;
    ctx.accounts.trade_token_vault.reload()?;

    let user_token = match User::token_get_mut(&mut user.user_tokens, &trade_token.mint) {
        None => {
            User::token_insert(
                &mut user.user_tokens,
                UserToken::new_using(
                    ctx.accounts.trade_token_vault.mint,
                    *ctx.accounts.user_token_account.to_account_info().key,
                ),
            )?;
            User::token_get_mut(&mut user.user_tokens, &token_mint).ok_or(CouldNotFindUserToken)?
        },
        Some(exist_user_token) => exist_user_token,
    };
    trade_token.add_amount(amount)?;
    let pre_user_token = user_token.add_amount(amount)?;

    emit!(UserTokenBalanceUpdateEvent {
        user_key,
        token_mint,
        pre_user_token,
        user_token: user_token.clone(),
        update_origin: UserTokenUpdateReason::DEPOSIT,
    });

    let repay_amount = user.repay_liability(&trade_token.mint, UserTokenUpdateReason::DEPOSIT)?;
    trade_token.sub_liability(repay_amount)?;
    if amount > repay_amount {
        let left_amount = amount.safe_sub(repay_amount)?;
        UserCrossPosition::update_balance(
            &mut user.user_positions,
            &trade_token.mint,
            left_amount,
            true,
        )?;
    }
    emit!(DepositEvent {
        user_key: ctx.accounts.user.to_account_info().key(),
        token_mint: ctx.accounts.trade_token_vault.mint,
        amount,
        deposit_origin: DepositOrigin::MANUAL,
    });
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum DepositOrigin {
    MANUAL,
    ORDER,
    STAKE,
}
