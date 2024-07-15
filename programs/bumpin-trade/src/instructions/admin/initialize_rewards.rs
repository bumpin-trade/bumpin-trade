use crate::state::pool::Pool;
use crate::state::rewards::Rewards;
use crate::state::state::State;
use crate::traits::Size;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(_pool_index: u16)]
pub struct InitializePoolRewards<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin,
    )]
    pub state: Account<'info, State>,

    #[account(
        seeds = [b"pool".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        constraint = pool_mint.key().eq(& pool.load() ?.mint_key)
    )]
    pub pool_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"rewards".as_ref(), _pool_index.to_le_bytes().as_ref()],
        space = Rewards::SIZE,
        bump,
        payer = admin,
    )]
    pub rewards: AccountLoader<'info, Rewards>,

    #[account(
        init,
        seeds = [b"pool_rewards_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = pool_mint,
        token::authority = bump_signer
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        token::mint = pool_mint,
    )]
    pub dao_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_initialize_rewards(ctx: Context<InitializePoolRewards>) -> Result<()> {
    let mut rewards = ctx.accounts.rewards.load_init()?;
    let pool = ctx.accounts.pool.load()?;
    rewards.pool_index = pool.index;
    rewards.pool_rewards_vault = ctx.accounts.pool_rewards_vault.to_account_info().key();
    rewards.dao_rewards_vault = ctx.accounts.dao_rewards_vault.to_account_info().key();
    Ok(())
}
