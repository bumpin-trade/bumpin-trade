use crate::errors::BumpErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solana_program::account_info::AccountInfo;

use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::state::infrastructure::user_stake::UserStakeStatus;
use crate::state::state::State;
use crate::state::user::User;
use crate::state::vault_map::VaultMap;
use crate::utils;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    pub authority: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_claim_rewards<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, ClaimRewards<'c>>,
) -> Result<()> {
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { pool_map, .. } = load_maps(remaining_accounts, &ctx.accounts.state.admin)?;
    let token_account_vec = VaultMap::load_vec(remaining_accounts)?;

    let user = &mut ctx.accounts.user.load_mut()?;
    for user_stake in user.user_stakes.iter_mut() {
        if user_stake.user_stake_status.eq(&UserStakeStatus::INIT)
            || user_stake.user_rewards.realised_rewards_token_amount <= 0u128
        {
            continue;
        }

        let pool_account_loader = pool_map.get_account_loader(&user_stake.pool_key)?;
        update_account_fee_reward(&ctx.accounts.user, pool_account_loader)?;
        let pool = pool_account_loader.load()?;
        let user = ctx.accounts.user.load()?;
        //transfer token to user wallet
        let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;

        let pool_rewards_vault = token_account_vec
            .iter()
            .find(|token_account| {
                token_account.owner.eq(&ctx.accounts.bump_signer.owner)
                    && token_account.mint.eq(&pool.pool_mint)
                    && token_account.key().eq(&pool.pool_mint_vault)
            })
            .ok_or(BumpErrorCode::InvalidParam)?;
        let user_token_account = token_account_vec
            .iter()
            .find(|token_account| {
                token_account.owner.eq(&user.authority) && token_account.mint.eq(&pool.pool_mint)
            })
            .ok_or(BumpErrorCode::InvalidParam)?;

        utils::token::send_from_program_vault(
            &ctx.accounts.token_program,
            pool_rewards_vault,
            user_token_account,
            &ctx.accounts.bump_signer,
            bump_signer_nonce,
            user_stake.user_rewards.realised_rewards_token_amount,
        )?;

        user_stake.user_rewards.realised_rewards_token_amount = 0;
        user_stake.user_rewards.open_rewards_per_stake_token =
            pool.fee_reward.cumulative_rewards_per_stake_token;
    }
    Ok(())
}
