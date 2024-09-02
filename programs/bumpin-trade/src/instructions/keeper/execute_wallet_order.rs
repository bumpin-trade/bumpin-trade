use crate::instructions::constraints::*;
use crate::instructions::ExecuteOrderParams;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor;
use crate::state::state::State;
use crate::state::User;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(
    params: ExecuteOrderParams
)]
pub struct ExecuteWalletOrder<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"user", params.user_authority_key.as_ref()],
        bump,
        constraint = is_normal(& user)?,
    )]
    pub user: AccountLoader<'info, User>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        constraint = state.keeper_key.eq(& keeper_key.key())
    )]
    pub keeper_key: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_execute_wallet_order<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ExecuteWalletOrder<'c>>,
    params: ExecuteOrderParams,
) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;
    let order = user.orders[user.get_user_order_index(params.order_id)?];
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { trade_token_map, mut oracle_map, market_map, pool_map, vault_map, .. } =
        load_maps(remaining_accounts)?;
    let state_account = &ctx.accounts.state;
    let user_token_account = &ctx.accounts.user_token_account;
    let bump_signer_account_info = &ctx.accounts.bump_signer;
    let token_program = &ctx.accounts.token_program;

    position_processor::handle_execute_order(
        user,
        &market_map,
        &pool_map,
        state_account,
        Some(user_token_account),
        &vault_map,
        bump_signer_account_info,
        token_program,
        ctx.program_id,
        &trade_token_map,
        &mut oracle_map,
        &order,
    )?;
    Ok(())
}
