use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::rent::Rent;
use crate::{safe_increment};
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use anchor_lang::error;
use crate::math_error;

#[derive(Accounts)]
pub struct InitializeTradeToken<'info> {
    #[account(
        init,
        seeds = [b"trade_token", state.number_of_trade_tokens.to_le_bytes().as_ref()],
        space = std::mem::size_of::< TradeToken > () + 8,
        bump,
        payer = admin
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,
    pub trade_token_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [b"trade_token_vault".as_ref(), state.number_of_trade_tokens.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = trade_token_mint,
        token::authority = bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,
    pub oracle: AccountInfo<'info>,
    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    pub bump_signer: AccountInfo<'info>,
    #[account(
        mut,
        has_one = bump_signer
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn initialize_trade_token(ctx: Context<InitializeTradeToken>, discount: u128, liquidation_factor: u128) -> anchor_lang::Result<()> {
    let state = &mut ctx.accounts.state;
    let mut trade_token = ctx.accounts.trade_token.load_init()?;
    *trade_token = TradeToken {
        mint: ctx.accounts.trade_token_mint.key(),
        oracle: *ctx.accounts.oracle.to_account_info().key,
        token_index: state.number_of_trade_tokens,
        discount,
        liquidation_factor,
        decimals: ctx.accounts.trade_token_mint.decimals,
        trade_token_vault: *ctx.accounts.trade_token_vault.to_account_info().key,
    };
    safe_increment!(state.number_of_trade_tokens,1);
    Ok(())
}