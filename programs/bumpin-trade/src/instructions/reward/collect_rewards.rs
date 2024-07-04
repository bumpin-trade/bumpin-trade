use crate::instructions::{cal_utils, swap};
use crate::math::safe_math::SafeMath;
use crate::state::pool::Pool;
use crate::state::rewards::Rewards;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::utils::token;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(_pool_index: u16, _stable_pool_index: u16,)]
pub struct CollectRewards<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Box<Account<'info, State>>,

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
        seeds = [b"pool_vault".as_ref(), _stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"pool_rewards".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub rewards: AccountLoader<'info, Rewards>,

    #[account(
        mut,
        seeds = [b"pool_rewards_vault".as_ref(), _pool_index.to_le_bytes().as_ref()],
        bump,
        token::authority = bump_signer
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = rewards.load() ?.dao_rewards_vault.eq(& dao_rewards_vault.to_account_info().key())
    )]
    pub dao_rewards_vault: Box<Account<'info, TokenAccount>>,

    pub keeper_signer: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_collect_rewards<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CollectRewards<'info>>,
    _pool_index: u16,
    _stable_pool_index: u16,
) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_mut()?;
    let total_supply = pool.total_supply;
    let fee_reward = &pool.fee_reward;
    let mut total_fee_amount = fee_reward.fee_amount;
    if !pool.stable {
        //need swap stable_fee_reward to amount
        let stable_fee_reward = &mut pool.stable_fee_reward;
        let fee_amount = stable_fee_reward.fee_amount;
        // todo swap stable to un_stable token, using jup_swap.
        let amount = swap::jup_swap()?;
        total_fee_amount = total_fee_amount.safe_add(amount)?;
        stable_fee_reward.sub_un_settle_amount(fee_amount)?
    }

    //split fee to pool_rewards & dao_rewards
    let pool_rewards_amount =
        cal_utils::mul_rate_u(total_fee_amount, ctx.accounts.state.pool_fee_reward_ratio as u128)?;
    let dao_rewards_amount = total_fee_amount.safe_sub(pool_rewards_amount)?;

    //transfer pool rewards
    token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.pool_vault,
        &ctx.accounts.pool_rewards_vault,
        &ctx.accounts.bump_signer,
        ctx.accounts.state.bump_signer_nonce,
        pool_rewards_amount,
    )?;
    // record pool rewards
    let mut rewards = ctx.accounts.rewards.load_mut()?;
    rewards.add_pool_total_rewards_amount(pool_rewards_amount)?;
    rewards.add_pool_un_claim_rewards(pool_rewards_amount)?;
    let fee_reward = &mut pool.fee_reward;
    fee_reward.add_cumulative_rewards_per_stake_token(
        pool_rewards_amount.safe_div_ceil(total_supply)?,
    )?;
    fee_reward.sub_fee_amount(fee_reward.fee_amount)?;


    //transfer dao rewards
    token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.pool_vault,
        &ctx.accounts.dao_rewards_vault,
        &ctx.accounts.bump_signer,
        ctx.accounts.state.bump_signer_nonce,
        dao_rewards_amount,
    )?;
    rewards.add_dao_total_rewards_amount(dao_rewards_amount)?;
    Ok(())
}
