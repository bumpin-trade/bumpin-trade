use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::processor::position_processor;
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::utils::token;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    params: UpdatePositionMarginParams
)]
pub struct AddPositionMargin<'info> {
    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        bump,
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
        seeds = [b"trade_token", params.trade_token_index.to_le_bytes().as_ref()],
        bump,
        constraint = trade_token.load() ?.mint_key.eq(& user_token_account.mint),
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"pool", params.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.mint_key.eq(& user_token_account.mint),
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        seeds = [b"market", params.market_index.to_le_bytes().as_ref()],
        bump,
        constraint = (market.load() ?.pool_mint_key.eq(& user_token_account.mint) || market.load() ?.stable_pool_mint_key.eq(& user_token_account.mint)) && market.load() ?.pool_key.eq(& pool.load() ?.key) || market.load() ?.stable_pool_key.eq(& pool.load() ?.key),
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
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Eq, PartialEq)]
pub struct UpdatePositionMarginParams {
    pub position_key: Pubkey,
    pub is_add: bool,
    pub update_margin_amount: u128,
    pub add_initial_margin_from_portfolio: u128,
    pub market_index: u16,
    pub pool_index: u16,
    pub trade_token_index: u16,
}

pub fn handle_add_position_margin<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, AddPositionMargin>,
    params: UpdatePositionMarginParams,
) -> Result<()> {
    validate!(params.update_margin_amount > 0u128, BumpErrorCode::AmountNotEnough.into())?;
    let mut user = ctx.accounts.user.load_mut()?;
    let trade_token = ctx.accounts.trade_token.load_mut()?;
    let remaining_accounts = ctx.remaining_accounts;
    let mut oracle_map = OracleMap::load(remaining_accounts)?;
    let position = user.get_user_position_mut_ref(&params.position_key)?;
    let mut pool = ctx.accounts.pool.load_mut()?;
    let market = ctx.accounts.market.load_mut()?;
    validate!(position.cross_margin, BumpErrorCode::OnlyIsolatePositionAllowed.into())?;
    if params.is_add {
        token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            &ctx.accounts.pool_mint_vault,
            &ctx.accounts.authority,
            params.update_margin_amount,
        )?;
    }
    validate!(
        trade_token.mint_key.eq(&position.margin_mint_key),
        BumpErrorCode::AmountNotEnough.into()
    )?;

    if params.is_add {
        position_processor::execute_add_position_margin(
            &params,
            &trade_token,
            &mut oracle_map,
            &mut pool,
            position,
        )?;
    } else {
        let reduce_margin_amount = position_processor::execute_reduce_position_margin(
            &params,
            true,
            &trade_token,
            &mut oracle_map,
            &mut pool,
            &market,
            &ctx.accounts.state,
            position,
        )?;
        token::send_from_program_vault(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_mint_vault,
            &ctx.accounts.user_token_account,
            &ctx.accounts.bump_signer,
            ctx.accounts.state.bump_signer_nonce,
            reduce_margin_amount,
        )?;
    }
    let mut user = ctx.accounts.user.load_mut()?;
    user.update_all_orders_leverage(
        position.leverage,
        position.symbol,
        &position.margin_mint_key,
        position.is_long,
        position.cross_margin,
    )?;
    Ok(())
}
