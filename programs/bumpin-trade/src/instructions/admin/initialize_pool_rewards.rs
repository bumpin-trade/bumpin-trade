use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::rent::Rent;
use crate::state::pool_rewards::PoolRewards;
use crate::state::state::State;
use crate::traits::Size;

#[derive(Accounts)]
#[instruction(pool_index: u16)]
pub struct InitializePoolRewards<'info> {
    pub pool_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"pool_rewards".as_ref(), pool_index.to_le_bytes().as_ref()],
        space = PoolRewards::SIZE,
        bump,
        payer = admin
    )]
    pub pool_rewards: AccountLoader<'info, PoolRewards>,

    #[account(
        init,
        seeds = [b"pool_rewards_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        payer = admin
        token::mint = pool_mint,
        token::authority = bump_signer
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = bump_signer
    )]
    pub state: Box<Account<'info, State>>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub bump_signer: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_initialize_dao_rewards(ctx: Context<InitializePoolRewards>, pool_index: u16) -> anchor_lang::Result<()> {
    let mut dao_rewards = ctx.accounts.pool_rewards.load_init()?;
    dao_rewards.pool_index = pool_index;
    dao_rewards.poo_rewards_vault = ctx.accounts.pool_rewards_vault.mint.key();
    Ok(())
}