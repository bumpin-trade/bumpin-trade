use std::ops::DerefMut;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::math_error;
use crate::safe_increment;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::traits::Size;

#[derive(Accounts)]
#[instruction(
    _pool_index: u16, _stable_pool_index: u16
)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        seeds = [b"market", state.market_sequence.to_le_bytes().as_ref()],
        space = Market::SIZE,
        bump,
        payer = admin
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), _stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    pub index_mint: Account<'info, Mint>,

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Account<'info, State>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize_market(
    ctx: Context<InitializeMarket>,
    symbol: [u8; 32],
    _pool_index: u16,
    _stable_pool_index: u16,
) -> Result<()> {
    let mut market = ctx.accounts.market.load_init()?;
    let mut pool = ctx.accounts.pool.load_mut()?;
    let mut stable_pool =  ctx.accounts.stable_pool.load_mut()?;
    let state = &mut ctx.accounts.state;
    market.index = state.market_sequence;
    market.symbol = symbol;
    market.pool_key = pool.key;
    market.pool_mint_key = pool.mint_key;
    market.index_mint_key = ctx.accounts.index_mint.key();
    market.stable_pool_mint_key = stable_pool.mint_key;
    market.stable_pool_key = stable_pool.key;
    safe_increment!(state.market_sequence, 1);
    safe_increment!(pool.deref_mut().market_number, 1);
    safe_increment!(stable_pool.deref_mut().market_number, 1);
    Ok(())
}
