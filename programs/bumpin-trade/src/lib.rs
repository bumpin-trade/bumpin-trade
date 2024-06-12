pub mod macros;
pub mod errors;
pub mod state;
pub mod instructions;
pub mod ids;
pub mod math;
pub mod traits;
pub mod utils;
pub mod processor;

use std::iter::Peekable;
use std::slice::Iter;
use anchor_lang::prelude::*;
use instructions::*;
declare_id!("GhzHdLjZ1qLLPnPq6YdeqJAszuBRN8WnLnK455yBbig6");

#[program]
pub mod bumpin_trade {
    use crate::processor::optional_accounts::{AccountMaps, load_maps};
    use crate::state::infrastructure::user_order::{UserOrder};
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    /*-----pool pool------*/
    pub fn pool_stake<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, PoolStake>,  params: StakeParams) -> Result<()> {
        handle_pool_stake(ctx, params)
    }

    pub fn pool_un_stake<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, PoolUnStake>, pool_index: usize, params: UnStakeParams) -> Result<()> {
        handle_pool_un_stake(ctx, pool_index, params)
    }

    /*-----account------*/
    pub fn deposit<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, Deposit>, amount: u128) -> Result<()> { handle_deposit(ctx, amount) }

    pub fn withdraw<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, Withdraw>, amount: u128) -> Result<()> { handle_withdraw(ctx, amount) }

    /*-----order------*/
    pub fn place_order<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, PlaceOrder>, params: PlaceOrderParams,
    ) -> Result<()> {
        handle_place_order(ctx, params)
    }

    pub fn execute_order<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, PlaceOrder>, order_id: u128,
    ) -> Result<()> {
        let user_account_loader = &ctx.accounts.user_account;
        let margin_token_account = &ctx.accounts.margin_token;
        let pool_account_loader = &ctx.accounts.pool;
        let stable_pool_account_loader = &ctx.accounts.stable_pool;
        let market_account_loader = &ctx.accounts.market;
        let state_account = &ctx.accounts.state;
        let user_token_account = &ctx.accounts.user_token_account;
        let pool_vault_account = &ctx.accounts.pool_vault;
        let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
        let trade_token_loader = &ctx.accounts.trade_token;
        let trade_token_vault_account = &ctx.accounts.trade_token_vault;
        let bump_signer_account_info = &ctx.accounts.bump_signer;
        let token_program = &ctx.accounts.token_program;
        let remaining_accounts_iter: &mut Peekable<Iter<'info, AccountInfo<'info>>> = &mut ctx.remaining_accounts.iter().peekable();
        let AccountMaps {
            trade_token_map,
            mut oracle_map,
            ..
        } = load_maps(remaining_accounts_iter)?;

        handle_execute_order(user_account_loader,
                             margin_token_account,
                             pool_account_loader,
                             stable_pool_account_loader,
                             market_account_loader,
                             state_account,
                             user_token_account,
                             pool_vault_account,
                             stable_pool_vault_account,
                             trade_token_loader,
                             trade_token_vault_account,
                             bump_signer_account_info,
                             token_program,
                             ctx.program_id,
                             &trade_token_map,
                             &mut oracle_map,
                             &mut UserOrder::default(),
                             order_id,
                             false)
    }

    pub fn cancel_order(ctx: Context<CancelOrderCtx>, order_id: u128) -> Result<()> {
        handle_cancel_order(ctx, order_id)
    }

    /*-----position------*/
    pub fn add_position_margin<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, AddPositionMargin>, params: UpdatePositionMarginParams) -> Result<()> {
        handle_add_position_margin(ctx, params)
    }

    pub fn update_position_leverage<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info, UpdatePositionLeverage>, params: UpdatePositionLeverageParams) -> Result<()> {
        handle_update_position_leverage(ctx, params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
