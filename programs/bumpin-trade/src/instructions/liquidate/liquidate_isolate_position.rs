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
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::msg;
use solana_program::pubkey::Pubkey;

#[derive(Accounts)]
#[instruction(market_index: u16, trade_token_index: u16, user: Pubkey)]
pub struct LiquidateIsolatePosition<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        constraint = pool_vault.mint.eq(& user_token_account.mint) || stable_pool_vault.mint.eq(& user_token_account.mint),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"market_index", market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    pub pool: AccountLoader<'info, Pool>,

    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        constraint = pool_vault.mint == pool.load() ?.pool_mint
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = stable_pool_vault.mint == stable_pool.load() ?.pool_mint
    )]
    pub stable_pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"trade_token", trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,
    #[account(
        constraint = trade_token_vault.mint == trade_token.load() ?.trade_token_vault
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,

    pub keeper_signer: Signer<'info>,

    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_liquidate_isolate_position(
    ctx: Context<LiquidateIsolatePosition>,
    position_key: Pubkey,
) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;
    let user_position = user.find_position_mut_by_key(&position_key)?;

    validate!(!user_position.cross_margin, BumpErrorCode::OnlyLiquidateIsolatePosition)?;
    let market = &mut ctx.accounts.market.load_mut()?;

    let remaining_accounts = &mut ctx.remaining_accounts.iter().peekable();
    let mut oracle_map = OracleMap::load(remaining_accounts)?;

    let mut market_processor = MarketProcessor { market };
    market_processor.update_market_funding_fee_rate(&ctx.accounts.state, &mut oracle_map)?;

    let pool = &mut ctx.accounts.pool.load_mut().unwrap();
    let stable_pool = &mut ctx.accounts.stable_pool.load_mut().unwrap();
    let mut pool_processor = if user_position.is_long {
        PoolProcessor { pool }
    } else {
        PoolProcessor { pool: stable_pool }
    };
    pool_processor.update_pool_borrowing_fee_rate()?;

    let mut position_processor = PositionProcessor { position: user_position };
    let liquidation_price = position_processor.get_liquidation_price(
        &market,
        &pool,
        &ctx.accounts.state,
        &mut oracle_map,
    )?;

    let index_price = oracle_map.get_price_data(&user_position.index_mint)?;
    if (user_position.is_long && index_price.price > liquidation_price)
        || (!user_position.is_long && index_price.price < liquidation_price)
    {
        position_processor.decrease_position(DecreasePositionParams {
            order_id: 0,
            is_liquidation: true,
            is_cross_margin: false,
            margin_token: position_processor.position.margin_mint,
            decrease_size: position_processor.position.position_size,
            execute_price: liquidation_price,
        }, &ctx.accounts.user, &ctx.accounts.pool,
                                             &ctx.accounts.stable_pool,
                                             &ctx.accounts.market,
                                             &ctx.accounts.state,
                                             Some(&ctx.accounts.user_token_account), if position_processor.position.is_long { &ctx.accounts.pool_vault } else { &ctx.accounts.stable_pool_vault },
                                             &ctx.accounts.trade_token,
                                             &ctx.accounts.trade_token_vault,
                                             &ctx.accounts.bump_signer,
                                             &ctx.accounts.token_program,
                                             &ctx.program_id,
                                             &mut oracle_map)?;
    }
    Ok(())
}
