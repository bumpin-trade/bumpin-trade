use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;

#[derive(Accounts)]
#[instruction(pool_index: u16, stable_pool_index: u16, trade_token_index: u16)]
pub struct CollectRewards<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Box<Account<'info, State>>,

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

    #[account(
        seeds = [b"trade_token".as_ref(), trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    pub keeper_signer: Signer<'info>,
}

pub fn handle_collect_rewards<'a, 'b, 'c: 'info, 'info>(
    _ctx: Context<'a, 'b, 'c, 'info, CollectRewards<'info>>,
    _pool_index: u16,
    _stable_pool_index: u16,
    _trade_token_index: u16,
) -> Result<()> {
    Ok(())
}
