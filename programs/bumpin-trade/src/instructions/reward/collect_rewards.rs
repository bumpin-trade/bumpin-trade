use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::pubkey::Pubkey;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;

#[derive(Accounts)]
#[instruction(pool_index: u16, stable_pool_index: u16)]
pub struct CollectRewards<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    pub keeper_signer: Signer<'info>,
}

pub fn handle_liquidate_cross_position<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, CollectRewards>, pool_index: u16, stable_pool_index: u16) -> Result<()> {
    Ok(())
}