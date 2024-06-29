use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::can_sign_for_user;
use crate::errors::BumpErrorCode;
use crate::instructions::Either;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::processor::optional_accounts::load_maps;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::user_processor::UserProcessor;
use crate::state::bump_events::StakeOrUnStakeEvent;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::{User, UserTokenUpdateReason};
use crate::{utils, validate};

#[derive(Accounts)]
#[instruction(un_stake_params: UnStakeParams,)]
pub struct PortfolioUnStake<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,
    #[account(
        mut,
        seeds = [b"user", authority.key.as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), un_stake_params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), un_stake_params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_mint_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), un_stake_params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token".as_ref(), un_stake_params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"pool_rewards_vault".as_ref(), un_stake_params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(un_stake_params: UnStakeParams,)]
pub struct WalletUnStake<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,
    #[account(
        mut,
        seeds = [b"user", authority.key.as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), un_stake_params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), un_stake_params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_mint_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token".as_ref(), un_stake_params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"pool_rewards_vault".as_ref(), un_stake_params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = pool_mint_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UnStakeParams {
    pub share: u128,
    pub pool_index: u16,
    pub trade_token_index: u16,
}

pub fn handle_portfolio_un_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PortfolioUnStake>,
    param: UnStakeParams,
) -> Result<()> {
    handle_pool_un_stake0(Either::Left(ctx), param)
}

pub fn handle_wallet_un_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, WalletUnStake>,
    param: UnStakeParams,
) -> Result<()> {
    handle_pool_un_stake0(Either::Right(ctx), param)
}

//TODO: Refactor this function to use the same logic as the stake function
fn handle_pool_un_stake0<'a, 'b, 'c: 'info, 'info>(
    ctx: Either<
        Context<'a, 'b, 'c, 'info, PortfolioUnStake>,
        Context<'a, 'b, 'c, 'info, WalletUnStake>,
    >,
    un_stake_params: UnStakeParams,
) -> Result<()> {
    match ctx {
        Either::Left(ctx) => {
            let pool = &mut ctx.accounts.pool.load_mut()?;
            let user = &mut ctx.accounts.user.load_mut()?;

            let user_stake = user.get_user_stake_ref(&pool.key)?;
            validate!(
                user_stake.staked_share >= un_stake_params.share,
                BumpErrorCode::UnStakeTooSmall
            )?;

            let remaining_accounts = ctx.remaining_accounts;
            let mut account_maps = load_maps(remaining_accounts, &ctx.accounts.state.admin)?;

            validate!(pool.total_supply != 0, BumpErrorCode::UnStakeTooSmall)?;

            let pool_processor = PoolProcessor { pool };

            let un_stake_token_amount = pool_processor.un_stake(
                &ctx.accounts.user,
                un_stake_params.share,
                &account_maps.trade_token_map,
                &mut account_maps.oracle_map,
                &account_maps.market_map,
            )?;
            drop(pool_processor);

            let un_stake_token_amount_fee =
                fee_processor::collect_un_stake_fee(pool, un_stake_token_amount)?;

            update_account_fee_reward(&ctx.accounts.user, &ctx.accounts.pool)?;

            let rewards_amount = user_stake.user_rewards.realised_rewards_token_amount;
            let transfer_amount = un_stake_token_amount
                .safe_add(rewards_amount)?
                .safe_sub(un_stake_token_amount_fee)?;

            let mut trade_token = ctx.accounts.trade_token.load_mut()?;
            utils::token::receive(
                &ctx.accounts.token_program,
                &ctx.accounts.pool_mint_vault,
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

            let repay_liability = user
                .repay_liability(&trade_token.mint_key, UserTokenUpdateReason::TransferFromStake)?;
            if repay_liability > 0 {
                trade_token.sub_liability(repay_liability)?;
            }

            let mut user_processor = UserProcessor { user };
            user_processor.update_cross_position_balance(
                &pool.mint_key,
                transfer_amount.safe_sub(repay_liability)?,
                true,
            )?;

            pool.sub_amount_and_supply(un_stake_token_amount, un_stake_params.share)?;

            let user_stake = user.get_user_stake_mut_ref(&pool.key)?;

            let user = &mut ctx.accounts.user.load_mut()?;
            if user_stake.staked_share <= 0 {
                user.delete_user_stake(&user_stake.pool_key)?
            }

            emit!(StakeOrUnStakeEvent {
                user_key: ctx.accounts.user.load()?.user_key,
                token_mint: ctx.accounts.pool.load()?.mint_key,
                change_supply_amount: un_stake_token_amount,
                user_stake: user_stake.clone(),
            });
        },
        Either::Right(ctx) => {
            let pool = &mut ctx.accounts.pool.load_mut()?;
            let user = &mut ctx.accounts.user.load_mut()?;

            let user_stake = user.get_user_stake_ref(&pool.key)?;
            validate!(
                user_stake.staked_share >= un_stake_params.share,
                BumpErrorCode::UnStakeTooSmall
            )?;

            let remaining_accounts = ctx.remaining_accounts;
            let mut account_maps = load_maps(remaining_accounts, &ctx.accounts.state.admin)?;

            validate!(pool.total_supply != 0, BumpErrorCode::UnStakeTooSmall)?;

            let mut pool_processor = PoolProcessor { pool };

            let un_stake_token_amount = pool_processor.un_stake(
                &ctx.accounts.user,
                un_stake_params.share,
                &account_maps.trade_token_map,
                &mut account_maps.oracle_map,
                &account_maps.market_map,
            )?;
            drop(pool_processor);

            let un_stake_token_amount_fee =
                fee_processor::collect_un_stake_fee(pool, un_stake_token_amount)?;

            update_account_fee_reward(&ctx.accounts.user, &ctx.accounts.pool)?;

            let rewards_amount = user_stake.user_rewards.realised_rewards_token_amount;
            let transfer_amount = un_stake_token_amount
                .safe_add(rewards_amount)?
                .safe_sub(un_stake_token_amount_fee)?;

            let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;

            utils::token::send_from_program_vault(
                &ctx.accounts.token_program,
                &ctx.accounts.pool_mint_vault,
                &ctx.accounts.user_token_account,
                &ctx.accounts.authority,
                bump_signer_nonce,
                transfer_amount,
            )?;

            pool.sub_amount_and_supply(un_stake_token_amount, un_stake_params.share)?;

            let user_stake = user.get_user_stake_mut_ref(&pool.key)?;

            let user = &mut ctx.accounts.user.load_mut()?;
            if user_stake.staked_share <= 0 {
                user.delete_user_stake(&user_stake.pool_key)?
            }

            emit!(StakeOrUnStakeEvent {
                user_key: ctx.accounts.user.load()?.user_key,
                token_mint: ctx.accounts.pool.load()?.mint_key,
                change_supply_amount: un_stake_token_amount,
                user_stake: user_stake.clone(),
            });
        },
    }

    Ok(())
}
