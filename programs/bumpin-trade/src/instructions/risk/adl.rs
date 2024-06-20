use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode;
use crate::errors::BumpErrorCode::CouldNotFindUserToken;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor::{DecreasePositionParams, PositionProcessor};
use crate::state::market::Market;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user_map::UserMap;
use crate::state::vault_map::VaultMap;
use crate::validate;

#[derive(Accounts)]
pub struct ADL<'info> {
    #[account(
        mut,
        constraint = pool.load() ?.pool_mint.eq(& margin_token.mint.key())
    )]
    pub margin_token: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = pool.load() ?.pool_mint == market.load() ?.pool_mint
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        constraint = stable_pool.load() ?.pool_mint == market.load() ?.stable_pool_mint
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    pub market: AccountLoader<'info, Market>,

    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        constraint = pool_vault.mint == pool.load() ?.pool_mint
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = stable_pool_vault.mint == stable_pool.load() ?.pool_mint
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        constraint = trade_token_vault.mint == trade_token.load() ?.trade_token_vault
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
    params: [ADLParams; 10],
) -> Result<()> {
    let pool_account_loader = &ctx.accounts.pool;
    let stable_pool_account_loader = &ctx.accounts.stable_pool;
    let market_account_loader = &ctx.accounts.market;
    let state_account = &ctx.accounts.state;
    let pool_vault_account = &ctx.accounts.pool_vault;
    let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
    let trade_token_loader = &ctx.accounts.trade_token;
    let trade_token_vault_account = &ctx.accounts.trade_token_vault;
    let bump_signer_account_info = &ctx.accounts.bump_signer;
    let token_program = &ctx.accounts.token_program;

    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();

    let AccountMaps { mut oracle_map, .. } =
        load_maps(remaining_accounts_iter, &state_account.admin)?;
    let user_map = UserMap::load(remaining_accounts_iter, ctx.program_id)?;
    let vault_vec = VaultMap::load_vec(remaining_accounts_iter)?;

    for param in params {
        let user_account_loader = user_map.get_account_loader(&param.user_key)?;
        let user_account = &mut user_map.get_mut_ref(&param.user_key)?;

        let user_token_account = vault_vec
            .iter()
            .find(|user_token_account| user_token_account.owner.eq(&user_account.authority))
            .ok_or(BumpErrorCode::CouldNotLoadUserData)?;

        let position = user_account.find_position_mut_by_key(&param.position_key)?;

        let user_account = user_map.get_ref(&param.user_key)?;
        let user_token =
            user_account.get_user_token_ref(&position.margin_mint)?.ok_or(CouldNotFindUserToken)?;

        validate!(
            user_token.user_token_account_key.eq(user_token_account.to_account_info().key),
            BumpErrorCode::InvalidTokenAccount
        )?;

        let mut position_processor = PositionProcessor { position };
        position_processor.decrease_position(
            DecreasePositionParams {
                order_id: 0,
                is_liquidation: false,
                is_cross_margin: position_processor.position.cross_margin,
                margin_token: position_processor.position.margin_mint,
                decrease_size: position_processor.position.position_size,
                execute_price: oracle_map
                    .get_price_data(&position_processor.position.index_mint)
                    .unwrap()
                    .price,
            },
            &user_account_loader,
            pool_account_loader,
            stable_pool_account_loader,
            market_account_loader,
            state_account,
            Some(user_token_account),
            if position_processor.position.is_long {
                pool_vault_account
            } else {
                stable_pool_vault_account
            },
            trade_token_loader,
            trade_token_vault_account,
            bump_signer_account_info,
            token_program,
            ctx.program_id,
            &mut oracle_map,
        )?;
    }
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct ADLParams {
    position_key: Pubkey,
    user_key: Pubkey,
}
