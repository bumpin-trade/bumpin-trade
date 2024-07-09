use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::user_processor;
use crate::state::bump_events::WithdrawEvent;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::{utils, validate};

#[derive(Accounts)]
#[instruction(token_index: u16,)]
pub struct Withdraw<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        mut,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = & trade_token_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token_vault".as_ref(), token_index.to_le_bytes().as_ref()],
        bump
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token", token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: forced drift_signer
    pub bump_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handle_withdraw<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Withdraw>,
    amount: u128,
) -> Result<()> {
    validate!(amount > 0, BumpErrorCode::AmountZero)?;

    let mut user = ctx.accounts.user.load_mut()?;
    let mut trade_token = ctx.accounts.trade_token.load_mut()?;
    let token_mint = &ctx.accounts.user_token_account.mint;
    let oracle = &trade_token.oracle_key;

    let user_token = user.get_user_token_ref(token_mint)?;
    validate!(user_token.amount > amount, BumpErrorCode::AmountNotEnough)?;

    let remaining_accounts = ctx.remaining_accounts;

    let AccountMaps { trade_token_map, mut oracle_map, .. } = load_maps(remaining_accounts)?;

    user_processor::withdraw(
        &mut user,
        amount,
        oracle,
        &trade_token,
        token_mint,
        &mut oracle_map,
        &trade_token_map,
    )?;
    trade_token.sub_amount(amount)?;

    let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;
    utils::token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.user_token_account,
        &ctx.accounts.bump_signer,
        bump_signer_nonce,
        amount,
    )?;

    emit!(WithdrawEvent {
        user_key: ctx.accounts.user.load()?.key,
        token_mint: *token_mint,
        amount,
    });

    Ok(())
}
