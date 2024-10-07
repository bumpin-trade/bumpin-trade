use crate::can_sign_for_user;
use crate::errors::BumpErrorCode;
use crate::is_normal;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::user_processor;
use crate::state::bump_events::WithdrawEvent;
use crate::state::state::State;
use crate::state::user::User;
use crate::{utils, validate};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

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
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault", token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: forced bump_signer
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[track_caller]
pub fn handle_withdraw<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, Withdraw>,
    token_index: u16,
    amount: u128,
) -> Result<()> {
    validate!(amount > 0, BumpErrorCode::AmountZero)?;

    let mut user = ctx.accounts.user.load_mut()?;
    let token_mint = &ctx.accounts.user_token_account.mint;
    let user_token = user.get_user_token_ref(token_mint)?;
    validate!(user_token.amount >= amount, BumpErrorCode::AmountNotEnough)?;

    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { trade_token_map, mut oracle_map, .. } = load_maps(remaining_accounts)?;

    let trade_token = trade_token_map.get_trade_token_by_mint_ref(token_mint)?;
    validate!(
        trade_token.index == token_index && trade_token.mint_key.eq(token_mint),
        BumpErrorCode::InvalidParam
    )?;
    drop(trade_token);

    user_processor::withdraw(&mut user, amount, token_mint, &mut oracle_map, &trade_token_map)?;

    let mut trade_token = trade_token_map.get_trade_token_by_mint_ref_mut(token_mint)?;
    trade_token.sub_total_amount(amount)?;

    let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;
    utils::token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.user_token_account,
        &ctx.accounts.bump_signer,
        bump_signer_nonce,
        amount,
    )?;

    emit!(WithdrawEvent { user_key: user.key, token_mint: *token_mint, amount });

    Ok(())
}
