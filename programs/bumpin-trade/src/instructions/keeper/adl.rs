use std::cell::RefMut;
use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor;
use crate::processor::position_processor::DecreasePositionParams;
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user_map::UserMap;
use crate::state::vault_map::VaultMap;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    pool_index: u16, stable_pool_index: u16, _market_index: u16, _trade_token_index: u16
)]
pub struct ADL<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"market", _market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.key.eq(& market.load() ?.pool_key),
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.key.eq(& market.load() ?.stable_pool_key),
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), stable_pool_index.to_le_bytes().as_ref()],
        bump,
        token::mint = stable_pool.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), _trade_token_index.to_le_bytes().as_ref()],
        bump,
        token::mint = trade_token.load() ?.mint_key,
        token::authority = state.bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_adl<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ADL<'info>>,
    pool_index: u16,
    stable_pool_index: u16,
    _market_index: u16,
    _trade_token_index: u16,
    params: [ADLParams; 10],
) -> Result<()> {
    let same_pool = pool_index == stable_pool_index;
    let mut pool = ctx.accounts.pool.load_mut()?;
    let mut stable_pool: Option<RefMut<Pool>> =
        if same_pool { None } else { Some(ctx.accounts.stable_pool.load_mut()?) };

    // let pool_account_loader = &ctx.accounts.pool;
    // let stable_pool_account_loader = &ctx.accounts.stable_pool;
    let market_account_loader = &ctx.accounts.market;
    let state_account = &ctx.accounts.state;
    let pool_vault_account = &ctx.accounts.pool_vault;
    let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
    let trade_token_loader = &ctx.accounts.trade_token;
    let trade_token_vault_account = &ctx.accounts.trade_token_vault;
    let bump_signer_account_info = &ctx.accounts.bump_signer;
    let token_program = &ctx.accounts.token_program;

    let remaining_accounts = ctx.remaining_accounts;

    let AccountMaps { mut oracle_map, trade_token_map, .. } = load_maps(remaining_accounts)?;
    let user_map = UserMap::load(remaining_accounts, ctx.program_id)?;
    let vault_vec = VaultMap::load_vec(remaining_accounts)?;

    for param in params {
        let user_account_loader = user_map.get_account_loader(&param.user_key)?;
        let mut user_account = user_account_loader.load_mut()?;

        let user_token_account = vault_vec
            .iter()
            .find(|user_token_account| user_token_account.owner.eq(&user_account.authority))
            .ok_or(BumpErrorCode::CouldNotLoadUserData)?;

        let position = user_account.get_user_position_ref(&param.position_key)?;
        let is_portfolio_margin = position.is_portfolio_margin;
        let margin_token = position.margin_mint_key;
        let decrease_size = position.position_size;
        let index_mint_key = position.index_mint_key;
        let position_key = position.position_key;
        let is_long = position.is_long;
        let user_token = user_account.get_user_token_ref(&margin_token)?;

        validate!(
            user_token.user_token_account_key.eq(user_token_account.to_account_info().key),
            BumpErrorCode::InvalidTokenAccount
        )?;
        let index_trade_token = trade_token_map.get_trade_token_by_mint_ref(&index_mint_key)?;
        position_processor::decrease_position(
            DecreasePositionParams {
                order_id: 0,
                is_liquidation: false,
                is_portfolio_margin,
                margin_token,
                decrease_size,
                execute_price: oracle_map
                    .get_price_data(&index_trade_token.oracle_key)
                    .map_err(|_e| BumpErrorCode::OracleNotFound)?
                    .price,
            },
            &mut user_account,
            market_account_loader.load_mut()?.deref_mut(),
            pool.deref_mut(),
            &mut stable_pool,
            state_account,
            Some(user_token_account),
            if is_long { pool_vault_account } else { stable_pool_vault_account },
            trade_token_loader.load_mut()?.deref_mut(),
            trade_token_vault_account,
            bump_signer_account_info,
            token_program,
            &mut oracle_map,
            &position_key,
        )?;
    }
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, Eq, PartialEq)]
pub struct ADLParams {
    position_key: Pubkey,
    user_key: Pubkey,
}