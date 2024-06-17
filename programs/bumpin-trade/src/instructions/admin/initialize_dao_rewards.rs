use crate::instructions::Deposit;
use crate::state::dao_rewards::DaoRewards;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::traits::Size;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::rent::Rent;

#[derive(Accounts)]
#[instruction(pool_index: u16)]
pub struct InitializeDaoRewards<'info> {
    pub dao_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"dao_rewards".as_ref(), pool_index.to_le_bytes().as_ref()],
        space = DaoRewards::SIZE,
        bump,
        payer = admin
    )]
    pub dao_rewards: AccountLoader<'info, DaoRewards>,

    #[account(
        init,
        seeds = [b"dao_rewards_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = dao_mint,
        token::authority = bump_signer
    )]
    pub dao_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = bump_signer
    )]
    pub state: Box<Account<'info, State>>,

    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_initialize_dao_rewards(
    ctx: Context<InitializeDaoRewards>,
    pool_index: u16,
) -> Result<()> {
    let mut dao_rewards = ctx.accounts.dao_rewards.load_init()?;
    dao_rewards.pool_index = pool_index;
    dao_rewards.dao_rewards_vault = ctx.accounts.dao_rewards_vault.mint.key();
    Ok(())
}
