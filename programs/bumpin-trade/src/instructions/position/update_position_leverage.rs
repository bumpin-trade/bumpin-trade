use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::utils::pda;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    params: UpdatePositionLeverageParams,
)]
pub struct UpdatePositionLeverage<'info> {
    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ? && is_normal(& user) ?,
    )]
    pub user: AccountLoader<'info, User>,

    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = pool_mint_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        seeds = [b"pool", params.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.mint_key.eq(& user_token_account.mint),
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        seeds = [b"pool", params.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.mint_key.eq(& user_token_account.mint),
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        seeds = [b"market", params.market_index.to_le_bytes().as_ref()],
        bump,
        constraint = (market.load() ?.pool_mint_key.eq(& user_token_account.mint) || market.load() ?.pool_key.eq(& user_token_account.mint)) && market.load() ?.pool_key.eq(& pool.load() ?.key) || market.load() ?.stable_pool_key.eq(& pool.load() ?.key),
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        seeds = [b"pool_vault".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = bump_signer
    )]
    pub pool_mint_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: forced drift_signer
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UpdatePositionLeverageParams {
    pub symbol: [u8; 32],
    pub is_long: bool,
    pub is_portfolio_margin: bool,
    pub leverage: u32,
    pub add_margin_amount: u128,
    pub market_index: u16,
    pub pool_index: u16,
}

pub fn handle_update_position_leverage<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, UpdatePositionLeverage>,
    params: UpdatePositionLeverageParams,
) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;
    let pool = &mut ctx.accounts.pool.load_mut()?;
    let stable_pool = &mut ctx.accounts.stable_pool.load_mut()?;
    let market = &mut ctx.accounts.market.load_mut()?;

    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { trade_token_map, mut oracle_map, market_map, .. } =
        load_maps(remaining_accounts)?;

    validate!(
        params.leverage <= market.config.maximum_leverage,
        BumpErrorCode::LeverageIsNotAllowed
    )?;

    let position_key = pda::generate_position_key(
        &user.key,
        params.symbol,
        params.is_portfolio_margin,
        ctx.program_id,
    )?;
    {
        let position = user.get_user_position_ref(&position_key)?;
        validate!(position.leverage != params.leverage, BumpErrorCode::LeverageIsNotAllowed)?;
    }

    position_processor::update_leverage(
        params,
        &position_key,
        user,
        &ctx.accounts.authority,
        pool,
        stable_pool,
        &ctx.accounts.state,
        market,
        &ctx.accounts.user_token_account,
        &ctx.accounts.pool_mint_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
        &trade_token_map,
        &mut oracle_map,
        &market_map,
    )?;
    Ok(())
}
