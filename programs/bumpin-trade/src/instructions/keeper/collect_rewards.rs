use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode;
use crate::instructions::{calculator, div_to_precision_u, token_to_usd_u};
use crate::math::constants::{PER_TOKEN_PRECISION, RATE_PRECISION};
use crate::math::safe_math::SafeMath;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::state::pool::Pool;
use crate::state::rewards::Rewards;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::utils::token;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    _pool_index: u16, _stable_pool_index: u16, _trade_token_index: u16, _stable_trade_token_index: u16
)]
pub struct CollectRewards<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_key,
        has_one = bump_signer,
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
        seeds = [b"trade_token", _trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"trade_token", _stable_trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        mut,
        seeds = [b"rewards".as_ref(), _pool_index.to_le_bytes().as_ref()],
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

pub fn handle_collect_rewards<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CollectRewards<'info>>,
) -> Result<()> {
    let mut pool = ctx.accounts.pool.load_mut()?;
    let total_supply = pool.total_supply;
    let fee_reward = &pool.fee_reward;
    let remaining_accounts = ctx.remaining_accounts;
    let AccountMaps { mut oracle_map, trade_token_map, market_map, .. } =
        load_maps(remaining_accounts)?;
    let trade_token = ctx.accounts.trade_token.load()?;

    validate!(total_supply > 0u128, BumpErrorCode::PoolMintSupplyIsZero)?;

    let total_fee_amount = fee_reward.fee_amount;
    let total_fee_value = token_to_usd_u(
        total_fee_amount,
        trade_token.decimals,
        oracle_map.get_price_data(&trade_token.oracle_key)?.price,
    )?;

    let pool_value = pool.get_pool_usd_value(&trade_token_map, &mut oracle_map, &market_map)?;
    let apr = div_to_precision_u(total_fee_value, pool_value, RATE_PRECISION)?;
    pool.set_apr(apr)?;
    //split fee to pool_rewards & dao_rewards
    let pool_rewards_amount =
        calculator::mul_rate_u(total_fee_amount, ctx.accounts.state.pool_fee_reward_ratio as u128)?;
    let dao_rewards_amount = total_fee_amount.safe_sub(pool_rewards_amount)?;

    //transfer pool rewards
    token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.pool_vault,
        &ctx.accounts.pool_rewards_vault,
        &ctx.accounts.bump_signer,
        ctx.accounts.state.bump_signer_nonce,
        pool_rewards_amount,
    )
    .map_err(|_e| BumpErrorCode::TransferFailed)?;
    // record pool rewards
    let mut rewards = ctx.accounts.rewards.load_mut()?;
    rewards.add_pool_total_rewards_amount(pool_rewards_amount)?;
    rewards.add_pool_un_claim_rewards(pool_rewards_amount)?;
    let fee_reward = &mut pool.fee_reward;
    let delta = pool_rewards_amount.safe_mul(PER_TOKEN_PRECISION)?.safe_div_ceil(total_supply)?;
    fee_reward.add_cumulative_rewards_per_stake_token(delta)?;
    fee_reward.push_last_rewards_per_stake_token_deltas(delta)?;
    fee_reward.sub_fee_amount(fee_reward.fee_amount)?;

    //transfer dao rewards
    token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.pool_vault,
        &ctx.accounts.dao_rewards_vault,
        &ctx.accounts.bump_signer,
        ctx.accounts.state.bump_signer_nonce,
        dao_rewards_amount,
    )
    .map_err(|_e| BumpErrorCode::TransferFailed)?;
    rewards.add_dao_total_rewards_amount(dao_rewards_amount)?;
    Ok(())
}
