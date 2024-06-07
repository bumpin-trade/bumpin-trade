use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::instructions::admin::initialize_pool::InitializePool;
use crate::safe_increment;
use crate::math_error;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::traits::Size;
use crate::state::state::State;

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        seeds = [b"market", state.number_of_markets.to_le_bytes().as_ref()],
        space = Market::SIZE,
        bump,
        payer = admin
    )]
    pub market: AccountLoader<'info, Market>,

    pub pool: AccountLoader<'info, Pool>,

    pub stable_pool: AccountLoader<'info, Pool>,

    pub index_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Account<'info, State>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,

}

pub fn handle_initialize_market(ctx: Context<InitializeMarket>, symbol: [u8; 32]) -> anchor_lang::Result<()> {
    let mut market = ctx.accounts.market.load_init()?;
    let pool = ctx.accounts.pool.load()?;
    let stable_pool = ctx.accounts.stable_pool.load()?;
    let state = &mut ctx.accounts.state;
    *market = Market {
        symbol,
        market_index: state.number_of_markets,
        pool_key: pool.pool_key,
        pool_mint: pool.pool_mint,
        index_mint: *ctx.accounts.index_mint.to_account_info().key,
        stable_pool_key: stable_pool.pool_key,
        stable_pool_mint: stable_pool.pool_mint,
        ..Market::default()
    };
    safe_increment!(state.number_of_markets,1);
    Ok(())
}