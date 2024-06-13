use anchor_lang::Accounts;
use anchor_lang::prelude::{Account, Program, Signer, System, Sysvar};
use anchor_spl::token::Token;
use solana_program::account_info::AccountInfo;
use solana_program::rent::Rent;
use crate::state::state::State;

#[derive(Accounts)]
pub struct InitializeDaoRewards<'info> {
    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    pub bump_signer: AccountInfo<'info>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Box<Account<'info, State>>,
    
    #[account(mut)]
    pub admin: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}