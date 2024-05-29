use std::alloc::System;
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::transfer;
use solana_program::msg;
use solana_program::program::invoke;
use solana_program::rent::Rent;
use crate::errors::BumpErrorCode::{CantPayUserInitFee};
use crate::errors::BumpResult;
use crate::load_mut;
use crate::state::state::State;
use crate::state::user::User;
use anchor_lang::require_keys_neq;
use anchor_lang::prelude::Pubkey;
use anchor_lang::error;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
    init,
    seeds = [b"user", authority.key.as_ref()],
    space = std::mem::size_of::< User > () + 8,
    bump,
    payer = payer
    )]
    pub user: AccountLoader<'info, User>,
    pub state: Box<Account<'info, State>>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_user(ctx: Context<InitializeUser>) -> BumpResult<()> {
    let user_key = ctx.accounts.user.key();
    let user = ctx.accounts.user.load_init()?;

    **user = User {
        authority: user.authority.key(),
        ..User::default()
    };

    let state = load_mut!( ctx.accounts.state);
    let init_fee = state.get_init_user_fee()?;

    if init_fee > 0 {
        let payer_lamports = ctx.accounts.payer.to_account_info().try_lamports()?;
        if payer_lamports < init_fee {
            msg!("payer lamports {} init fee {}", payer_lamports, init_fee);
            return Err(CantPayUserInitFee.into());
        }

        invoke(
            &transfer(
                &ctx.accounts.payer.key(),
                &ctx.accounts.user.key(),
                init_fee,
            ),
            &[
                ctx.accounts.payer.to_account_info().clone(),
                ctx.accounts.user.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;
    }
    Ok(())
}