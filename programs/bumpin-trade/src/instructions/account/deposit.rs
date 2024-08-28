use crate::can_sign_for_user;
use crate::is_normal;
use crate::math::safe_math::SafeMath;
use crate::processor::user_processor;
use crate::state::bump_events::DepositEvent;
use crate::state::trade_token::TradeToken;
use crate::state::user::{User, UserTokenUpdateReason};
use crate::utils::token;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(_token_index: u16)]
pub struct Deposit<'info> {
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

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_deposit(ctx: Context<Deposit>, _token_index: u16, amount: u128) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let trade_token = &mut ctx.accounts.trade_token.load_mut()?;
    let token_mint = trade_token.mint_key;

    token::receive(
        &ctx.accounts.token_program,
        &ctx.accounts.user_token_account,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.authority,
        amount,
    )?;

    trade_token.add_total_amount(amount)?;
    //check user token exist, if not create new user token
    user.force_get_user_token_mut_ref(&token_mint)?;

    user.add_user_token_amount(&token_mint, amount, &UserTokenUpdateReason::DEPOSIT)?;

    let repay_amount =
        user.repay_liability(&trade_token.mint_key, UserTokenUpdateReason::DEPOSIT)?;
    trade_token.sub_total_liability(repay_amount)?;
    if amount > repay_amount {
        let left_amount = amount.safe_sub(repay_amount)?;
        user_processor::update_cross_position_balance(
            &mut user,
            &trade_token.mint_key,
            left_amount,
            true,
        )?
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
