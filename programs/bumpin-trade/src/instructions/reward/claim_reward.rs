use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::instructions::calculator;
use crate::instructions::constraints::*;
use crate::instructions::ClaimRewardsParams;
use crate::processor::user_processor;
use crate::state::infrastructure::user_stake::UserStakeStatus;
use crate::state::pool::Pool;
use crate::state::rewards::Rewards;
use crate::state::state::State;
use crate::state::user::User;
use crate::{utils, validate};

#[derive(Accounts)]
#[instruction(params: ClaimRewardsParams,)]
pub struct ClaimRewards<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

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
        seeds = [b"rewards".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub rewards: AccountLoader<'info, Rewards>,

    #[account(
        mut,
        constraint = user_token_account.owner.eq(& user.load() ?.authority) && pool_rewards_vault.mint.eq(& user_token_account.mint),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_rewards_vault".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
        token::authority = bump_signer
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_claim_rewards<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ClaimRewards<'c>>,
    params: ClaimRewardsParams,
) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let mut pool = ctx.accounts.pool.load_mut()?;
    let mut reward = ctx.accounts.rewards.load_mut()?;
    validate!(params.pool_index == pool.index, BumpErrorCode::InvalidPoolAccount)?;
    msg!("=========handle_claim_rewards, user_key:{}", user.authority);
    msg!("=========handle_claim_rewards, pool_key:{}", pool.key);
    user_processor::update_account_fee_reward(&mut pool, &mut user)?;
    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;
    validate!(
        user_stake.user_stake_status.eq(&UserStakeStatus::USING)
            && user_stake.user_rewards.realised_rewards_token_amount > 0u128,
        BumpErrorCode::UserStakeHasNoMoreClaim
    )?;
    //transfer token to user wallet
    let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;

    let pool_rewards_vault = &ctx.accounts.pool_rewards_vault;
    let user_token_account = &ctx.accounts.user_token_account;

    msg!(
        "=========handle_claim_rewards, stake:{}",
        user_stake.user_rewards.realised_rewards_token_amount
    );
    reward.sub_pool_un_claim_rewards(user_stake.user_rewards.realised_rewards_token_amount)?;

    utils::token::send_from_program_vault(
        &ctx.accounts.token_program,
        pool_rewards_vault,
        user_token_account,
        &ctx.accounts.bump_signer,
        bump_signer_nonce,
        user_stake.user_rewards.realised_rewards_token_amount,
    )
    .map_err(|_e| BumpErrorCode::TransferFailed)?;
    user_stake.user_rewards.total_claim_rewards_amount = calculator::add_u128(
        user_stake.user_rewards.total_claim_rewards_amount,
        user_stake.user_rewards.realised_rewards_token_amount,
    )?;
    user_stake.user_rewards.realised_rewards_token_amount = 0;
    user_stake.user_rewards.open_rewards_per_stake_token =
        pool.fee_reward.cumulative_rewards_per_stake_token;
    Ok(())
}
