use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::position_processor::{DecreasePositionParams, PositionProcessor};
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    _market_index: u16, _pool_index: u16, _stable_pool_index: u16, _trade_token_index: u16, _index_trade_token_index: u16, _user_authority_key: Pubkey
)]
pub struct LiquidateIsolatePosition<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        seeds = [b"user", _user_authority_key.as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        constraint = user_token_account.owner.eq(& user.load() ?.authority)
        && (pool_mint_vault.mint.eq(& user_token_account.mint) || stable_pool_mint_vault.mint.eq(& user_token_account.mint)),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"market", _market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool", _pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.pool_mint.eq(& market.load() ?.pool_mint)
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool", _stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.pool_mint.eq(& market.load() ?.stable_pool_mint)
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_mint_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), _stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool_mint_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub index_trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"trade_token_vault".as_ref(), _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,

    pub keeper_signer: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_liquidate_isolate_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, LiquidateIsolatePosition>,
    position_key: Pubkey,
    _market_index: u16,
    _pool_index: u16,
    _stable_pool_index: u16,
    _index_trade_token_index: u16,
    _user_authority_key: Pubkey,
) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;
    let user_position = user.find_position_mut_by_key(&position_key)?;

    validate!(!user_position.cross_margin, BumpErrorCode::OnlyLiquidateIsolatePosition)?;
    let market = &mut ctx.accounts.market.load_mut()?;

    let remaining_accounts = ctx.remaining_accounts;
    let mut oracle_map = OracleMap::load(remaining_accounts)?;

    let mut market_processor = MarketProcessor { market };
    let trade_token = ctx.accounts.trade_token.load()?;
    market_processor.update_market_funding_fee_rate(
        &ctx.accounts.state,
        oracle_map.get_price_data(&trade_token.oracle)?.price,
    )?;

    let pool = &mut ctx.accounts.pool.load_mut().unwrap();
    let stable_pool = &mut ctx.accounts.stable_pool.load_mut().unwrap();
    let mut pool_processor = if user_position.is_long {
        PoolProcessor { pool }
    } else {
        PoolProcessor { pool: stable_pool }
    };
    pool_processor.update_pool_borrowing_fee_rate()?;

    let mut position_processor = PositionProcessor { position: user_position };
    let margin_token_price = oracle_map.get_price_data(&trade_token.oracle)?.price;
    let liquidation_price = position_processor.get_liquidation_price(
        &market,
        &pool,
        &ctx.accounts.state,
        margin_token_price,
        trade_token.decimals,
    )?;
    let index_trade_token = ctx.accounts.index_trade_token.load()?;

    let index_price = oracle_map.get_price_data(&index_trade_token.oracle)?;
    if (position_processor.position.is_long && index_price.price > liquidation_price)
        || (!position_processor.position.is_long && index_price.price < liquidation_price)
    {
        position_processor.decrease_position(
            DecreasePositionParams {
                order_id: 0,
                is_liquidation: true,
                is_cross_margin: false,
                margin_token: position_processor.position.margin_mint,
                decrease_size: position_processor.position.position_size,
                execute_price: liquidation_price,
            },
            &ctx.accounts.user,
            &ctx.accounts.pool,
            &ctx.accounts.stable_pool,
            &ctx.accounts.market,
            &ctx.accounts.state,
            Some(&ctx.accounts.user_token_account),
            if position_processor.position.is_long {
                &ctx.accounts.pool_mint_vault
            } else {
                &ctx.accounts.stable_pool_mint_vault
            },
            &ctx.accounts.trade_token,
            &ctx.accounts.trade_token_vault,
            &ctx.accounts.bump_signer,
            &ctx.accounts.token_program,
            &ctx.program_id,
            &mut oracle_map,
        )?;
    }
    Ok(())
}
