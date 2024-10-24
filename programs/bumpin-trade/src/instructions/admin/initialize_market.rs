use std::ops::DerefMut;

use anchor_lang::prelude::*;

use crate::math_error;
use crate::safe_increment;
use crate::state::market::{Market, MarketConfig};
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::traits::Size;

#[derive(Accounts)]
#[instruction(
    params: InitializeMarketParams,
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
        seeds = [b"pool".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    /// CHECK: checked by admin
    pub index_mint_oracle: AccountInfo<'info>,

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

#[derive(Accounts)]
#[instruction(
    params: ModifyMarketParams,
)]
pub struct UpdateMarket<'info> {
    #[account(
        mut,
        seeds = [b"market", params.market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Account<'info, State>,

    #[account(
        mut,
        constraint = state.admin.eq(& admin.key())
    )]
    pub admin: Signer<'info>,
}

pub fn handle_initialize_market(
    ctx: Context<InitializeMarket>,
    params: InitializeMarketParams,
) -> Result<()> {
    let mut market = ctx.accounts.market.load_init()?;
    let mut pool = ctx.accounts.pool.load_mut()?;
    let stable_pool = if params.pool_index == params.stable_pool_index {
        None
    } else {
        Some(ctx.accounts.stable_pool.load_mut()?)
    };
    let state = &mut ctx.accounts.state;

    let config = MarketConfig {
        tick_size: params.tick_size,
        open_fee_rate: params.open_fee_rate,
        close_fee_rate: params.close_fee_rate,
        maximum_long_open_interest_cap: params.maximum_long_open_interest_cap,
        maximum_short_open_interest_cap: params.maximum_short_open_interest_cap,
        long_short_ratio_limit: params.long_short_ratio_limit,
        long_short_oi_bottom_limit: params.long_short_oi_bottom_limit,
        maximum_leverage: params.maximum_leverage,
        minimum_leverage: params.minimum_leverage,
        max_pool_liquidity_share_rate: params.max_pool_liquidity_share_rate,
        padding: [0; 4],
    };

    market.index = state.market_sequence;
    market.symbol = params.symbol;
    market.pool_key = pool.key;
    market.pool_mint_key = pool.mint_key;
    market.index_mint_oracle = ctx.accounts.index_mint_oracle.key();
    match stable_pool {
        None => {
            market.stable_pool_mint_key = pool.mint_key;
            market.stable_pool_key = pool.key;
            safe_increment!(pool.deref_mut().market_number, 1);
        },
        Some(mut short_pool) => {
            market.stable_pool_mint_key = short_pool.mint_key;
            market.stable_pool_key = short_pool.key;
            safe_increment!(short_pool.deref_mut().market_number, 1);
        },
    };
    market.config = config;
    market.share_short = params.share_short;
    market.stable_loss = 0i128;
    market.stable_unsettle_loss = 0u128;
    safe_increment!(state.market_sequence, 1);
    safe_increment!(pool.deref_mut().market_number, 1);
    Ok(())
}

pub fn handle_modify_market(ctx: Context<UpdateMarket>, params: ModifyMarketParams) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;
    if let Some(symbol) = params.symbol {
        market.symbol = symbol;
    }
    if let Some(tick_size) = params.tick_size {
        market.config.tick_size = tick_size;
    }
    if let Some(open_fee_rate) = params.open_fee_rate {
        market.config.open_fee_rate = open_fee_rate;
    }
    if let Some(close_fee_rate) = params.close_fee_rate {
        market.config.close_fee_rate = close_fee_rate;
    }
    if let Some(maximum_long_open_interest_cap) = params.maximum_long_open_interest_cap {
        market.config.maximum_long_open_interest_cap = maximum_long_open_interest_cap;
    }
    if let Some(maximum_short_open_interest_cap) = params.maximum_short_open_interest_cap {
        market.config.maximum_short_open_interest_cap = maximum_short_open_interest_cap;
    }
    if let Some(long_short_ratio_limit) = params.long_short_ratio_limit {
        market.config.long_short_ratio_limit = long_short_ratio_limit;
    }
    if let Some(long_short_oi_bottom_limit) = params.long_short_oi_bottom_limit {
        market.config.long_short_oi_bottom_limit = long_short_oi_bottom_limit;
    }
    if let Some(maximum_leverage) = params.maximum_leverage {
        market.config.maximum_leverage = maximum_leverage;
    }
    if let Some(minimum_leverage) = params.minimum_leverage {
        market.config.minimum_leverage = minimum_leverage;
    }
    if let Some(max_pool_liquidity_share_rate) = params.max_pool_liquidity_share_rate {
        market.config.max_pool_liquidity_share_rate = max_pool_liquidity_share_rate;
    }
    if let Some(share_short) = params.share_short {
        market.share_short = share_short;
    }
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct InitializeMarketParams {
    pub symbol: [u8; 32],
    pub tick_size: u128,
    pub open_fee_rate: u128,
    pub close_fee_rate: u128,
    pub maximum_long_open_interest_cap: u128,
    pub maximum_short_open_interest_cap: u128,
    pub long_short_ratio_limit: u128,
    pub long_short_oi_bottom_limit: u128,
    pub maximum_leverage: u32,
    pub minimum_leverage: u32,
    pub pool_index: u16,
    pub stable_pool_index: u16,
    pub max_pool_liquidity_share_rate: u32,
    pub share_short: bool,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct ModifyMarketParams {
    pub market_index: u16,
    pub symbol: Option<[u8; 32]>,
    pub tick_size: Option<u128>,
    pub open_fee_rate: Option<u128>,
    pub close_fee_rate: Option<u128>,
    pub maximum_long_open_interest_cap: Option<u128>,
    pub maximum_short_open_interest_cap: Option<u128>,
    pub long_short_ratio_limit: Option<u128>,
    pub long_short_oi_bottom_limit: Option<u128>,
    pub maximum_leverage: Option<u32>,
    pub minimum_leverage: Option<u32>,
    pub max_pool_liquidity_share_rate: Option<u32>,
    pub share_short: Option<bool>,
}
