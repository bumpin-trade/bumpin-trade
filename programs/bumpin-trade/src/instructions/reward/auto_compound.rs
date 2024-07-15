use crate::errors::BumpErrorCode;
use crate::instructions::constraints::*;
use crate::processor::optional_accounts::load_maps;
use crate::processor::{pool_processor, stake_processor};
use crate::state::bump_events::StakeOrUnStakeEvent;
use crate::state::infrastructure::user_stake::UserStakeStatus;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::utils::token;
use crate::validate;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(_pool_index: u16,)]
pub struct AutoCompoundRewards<'info> {
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
        seeds = [b"pool".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool_rewards_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
        token::authority = bump_signer
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_auto_compound<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, AutoCompoundRewards<'c>>,
    _pool_index: u16,
) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let mut pool = ctx.accounts.pool.load_mut()?;
    let user_stake = *user.get_user_stake_ref(&pool.key)?;
    validate!(
        user_stake.user_stake_status.eq(&UserStakeStatus::USING)
            && user_stake.user_rewards.realised_rewards_token_amount > 0u128,
        BumpErrorCode::CouldNotFindUserStake
    )?;

    let remaining_accounts = ctx.remaining_accounts;
    let account_maps = &mut load_maps(remaining_accounts)?;
    let stake_amount = stake_processor::stake(
        &mut pool,
        &mut user,
        &account_maps.trade_token_map,
        &mut account_maps.oracle_map,
        user_stake.user_rewards.realised_rewards_token_amount,
    )?;

    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;
    let token_amount = user_stake.user_rewards.realised_rewards_token_amount;
    user_stake.user_rewards.realised_rewards_token_amount = 0;
    user_stake.user_rewards.open_rewards_per_stake_token =
        pool.fee_reward.cumulative_rewards_per_stake_token;

    let account_maps = &mut load_maps(remaining_accounts)?;
    let supply_amount = pool_processor::stake(
        &mut pool,
        stake_amount,
        &account_maps.trade_token_map,
        &mut account_maps.oracle_map,
        &account_maps.market_map,
    )?;
    user_stake.add_staked_share(supply_amount)?;
    //transfer from pool_reward_vault to pool_vault
    token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.pool_rewards_vault,
        &ctx.accounts.pool_vault,
        &ctx.accounts.bump_signer,
        ctx.accounts.state.bump_signer_nonce,
        token_amount,
    )
    .map_err(|_e| BumpErrorCode::TransferFailed)?;
    pool.add_amount_and_supply(token_amount, supply_amount)?;
    emit!(StakeOrUnStakeEvent {
        user_key: ctx.accounts.user.load()?.key,
        token_mint: pool.mint_key,
        change_supply_amount: supply_amount,
        user_stake: *user_stake,
    });
    Ok(())
}
