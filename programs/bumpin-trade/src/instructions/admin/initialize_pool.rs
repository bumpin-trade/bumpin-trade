use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::math_error;
use crate::safe_increment;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::traits::Size;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        seeds = [b"pool", state.number_of_pools.to_le_bytes().as_ref()],
        space = Pool::SIZE,
        bump,
        payer = admin
    )]
    pub pool: AccountLoader<'info, Pool>,

    pub pool_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"pool_mint_vault".as_ref(), state.number_of_pools.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = pool_mint,
        token::authority = bump_signer
    )]
    pub pool_mint_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        seeds = [b"pool_rewards_vault".as_ref(), state.number_of_pools.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = pool_mint,
        token::authority = bump_signer
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        seeds = [b"pool_fee_vault".as_ref(), state.number_of_pools.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = pool_mint,
        token::authority = bump_signer
    )]
    pub pool_fee_vault: Box<Account<'info, TokenAccount>>,

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

pub fn handle_initialize_pool(ctx: Context<InitializePool>, name: [u8; 32]) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_init()?;
    let state = &mut ctx.accounts.state;
    pool.pool_key = ctx.accounts.pool.key();
    pool.pool_mint = ctx.accounts.pool_mint.key();
    pool.pool_mint_vault = *ctx.accounts.pool_mint_vault.to_account_info().key;
    pool.pool_rewards_vault = *ctx.accounts.pool_rewards_vault.to_account_info().key;
    pool.pool_fee_vault = *ctx.accounts.pool_fee_vault.to_account_info().key;
    pool.pool_name = name;

    // *pool = Pool {
    //     pool_key: ctx.accounts.pool.key(),
    //     pool_mint: ctx.accounts.pool_mint.key(),
    //     pool_mint_vault: *ctx.accounts.pool_mint_vault.to_account_info().key,
    //     pool_rewards_vault: *ctx.accounts.pool_rewards_vault.to_account_info().key,
    //     pool_fee_vault: *ctx.accounts.pool_fee_vault.to_account_info().key,
    //     pool_name: name,
    //     ..Pool::default()
    // };
    safe_increment!(state.number_of_pools, 1);
    Ok(())
}