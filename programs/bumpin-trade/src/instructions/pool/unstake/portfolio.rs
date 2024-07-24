use std::ops::DerefMut;

use crate::can_sign_for_user;
use crate::errors::BumpErrorCode;
use crate::instructions::unstake::UnStakeParams;
use crate::is_normal;

use crate::math::safe_math::SafeMath;
use crate::processor::optional_accounts::load_maps;
use crate::processor::{fee_processor, pool_processor, user_processor};
use crate::state::bump_events::StakeOrUnStakeEvent;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::{User, UserTokenUpdateReason};
use crate::{utils, validate};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[instruction(params: UnStakeParams,)]
pub struct PortfolioUnStake<'info> {
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

    #[account(
        mut,
        seeds = [b"pool".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token".as_ref(), params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"pool_rewards_vault".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

#[track_caller]
pub fn handle_portfolio_un_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PortfolioUnStake>,
    param: UnStakeParams,
) -> anchor_lang::Result<()> {
    let pool = &mut ctx.accounts.pool.load_mut()?;
    let user = &mut ctx.accounts.user.load_mut()?;

    let user_stake = &user.get_user_stake_ref(&pool.key)?.clone();
    validate!(user_stake.staked_share >= param.share, BumpErrorCode::UnStakeTooSmall)?;

    let remaining_accounts = ctx.remaining_accounts;
    let mut account_maps = load_maps(remaining_accounts)?;

    validate!(pool.total_supply != 0, BumpErrorCode::UnStakeTooSmall)?;

    let un_stake_token_amount = pool_processor::un_stake(
        pool,
        user,
        param.share,
        &account_maps.trade_token_map,
        &mut account_maps.oracle_map,
        &account_maps.market_map,
    )?;

    let un_stake_token_amount_fee =
        fee_processor::collect_un_stake_fee(pool, un_stake_token_amount)?;

    user_processor::update_account_fee_reward(pool.deref_mut(), user.deref_mut())?;

    let rewards_amount = user_stake.user_rewards.realised_rewards_token_amount;
    let transfer_amount =
        un_stake_token_amount.safe_add(rewards_amount)?.safe_sub(un_stake_token_amount_fee)?;

    let mut trade_token = ctx.accounts.trade_token.load_mut()?;

    utils::token::receive(
        &ctx.accounts.token_program,
        &ctx.accounts.pool_vault,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.authority,
        transfer_amount,
    )?;

    user.add_user_token_amount(
        &trade_token.mint_key,
        rewards_amount.safe_add(transfer_amount)?,
        &UserTokenUpdateReason::TransferFromStake,
    )?;
    trade_token.add_amount(transfer_amount)?;

    let repay_liability =
        user.repay_liability(&trade_token.mint_key, UserTokenUpdateReason::TransferFromStake)?;
    if repay_liability > 0 {
        trade_token.sub_liability(repay_liability)?;
    }

    user_processor::update_cross_position_balance(
        user,
        &pool.mint_key,
        transfer_amount.safe_sub(repay_liability)?,
        true,
    )?;

    pool.sub_amount_and_supply(un_stake_token_amount, param.share)?;
    pool.update_pool_borrowing_fee_rate()?;

    let user_stake = user.get_user_stake_mut_ref(&pool.key)?;

    let user = &mut ctx.accounts.user.load_mut()?;
    if user_stake.staked_share <= 0u128 {
        user.delete_user_stake(&user_stake.pool_key)?
    }

    emit!(StakeOrUnStakeEvent {
        user_key: ctx.accounts.user.load()?.key,
        token_mint: ctx.accounts.pool.load()?.mint_key,
        change_supply_amount: un_stake_token_amount,
        user_stake: user_stake.clone(),
    });

    Ok(())
}
