use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::math_error;
use crate::safe_increment;
use crate::state::pool::{Pool, PoolConfig};
use crate::state::state::State;
use crate::traits::Size;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        seeds = [b"pool", state.pool_sequence.to_le_bytes().as_ref()],
        space = Pool::SIZE,
        bump,
        payer = admin
    )]
    pub pool: AccountLoader<'info, Pool>,

    pub pool_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [b"pool_vault".as_ref(), state.pool_sequence.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = pool_mint,
        token::authority = bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Account<'info, State>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct InitializePoolParams {
    pub name: [u8; 32],
    pub stable_mint_key: [u8; 32],
    pub pool_config: PoolConfig,
    pub stable: bool,
}

pub fn handle_initialize_pool(
    ctx: Context<InitializePool>,
    params: InitializePoolParams,
) -> Result<()> {
    let key = ctx.accounts.pool.key();
    let pool = &mut ctx.accounts.pool.load_init()?;
    let state = &mut ctx.accounts.state;
    pool.key = key;
    pool.mint_key = ctx.accounts.pool_vault.mint;
    pool.pool_vault_key = ctx.accounts.pool_vault.key();
    pool.name = params.name;
    pool.index = state.pool_sequence;
    pool.stable = params.stable;
    pool.stable_mint_key = Pubkey::new_from_array(params.stable_mint_key);
    pool.config = params.pool_config;
    safe_increment!(state.pool_sequence, 1);
    Ok(())
}
