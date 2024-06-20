use std::iter::Peekable;
use std::slice::Iter;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::can_sign_for_user;
use crate::errors::BumpErrorCode;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::processor::optional_accounts::load_maps;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::user_processor::UserProcessor;
use crate::state::infrastructure::user_token::{UserToken, UserTokenStatus};
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::{utils, validate};

#[derive(Accounts)]
#[instruction(pool_index: u16, trade_token_index: u16)]
pub struct PoolUnStake<'info> {
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
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token".as_ref(), trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"pool_rewards_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_rewards_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = pool_vault.mint.eq(& user_token_account.mint) && trade_token_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UnStakeParams {
    un_stake_token_amount: u128,
    portfolio: bool,
}

pub fn handle_pool_un_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PoolUnStake>,
    un_stake_params: UnStakeParams,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool.load_mut()?;
    let user = &mut ctx.accounts.user.load_mut()?;

    let user_stake = user.get_user_stake_ref(&pool.pool_key)?.ok_or(BumpErrorCode::StakePaused)?;
    validate!(
        user_stake.amount >= un_stake_params.un_stake_token_amount,
        BumpErrorCode::UnStakeNotEnough
    )?;

    let remaining_accounts: &mut Peekable<Iter<'info, AccountInfo<'info>>> =
        &mut ctx.remaining_accounts.iter().peekable();
    let mut account_maps = load_maps(remaining_accounts, &ctx.accounts.state.admin)?;

    validate!(pool.total_supply == 0, BumpErrorCode::UnStakeNotEnough)?;

    let mut pool_processor = PoolProcessor { pool };

    let un_stake_token_amount = pool_processor.un_stake(
        &ctx.accounts.pool,
        &ctx.accounts.user,
        un_stake_params.un_stake_token_amount,
        &mut account_maps.oracle_map,
        &account_maps.market_map,
    )?;

    let un_stake_token_amount_fee = pool_processor.collect_un_stake_fee(un_stake_token_amount)?;

    update_account_fee_reward(&ctx.accounts.user, &ctx.accounts.pool)?;

    let rewards_amount = user_stake.user_rewards.realised_rewards_token_amount;
    let transfer_amount = un_stake_token_amount.safe_sub(un_stake_token_amount_fee)?;

    if un_stake_params.portfolio {
        let trade_token = ctx.accounts.trade_token.load_mut()?;
        let user_token_option = user.get_user_token_mut(&ctx.accounts.trade_token_vault.mint)?;
        let user_token = match user_token_option {
            None => {
                let index = user.next_usable_user_token_index()?;
                //init user_token
                let new_token = &mut UserToken {
                    user_token_status: UserTokenStatus::USING,
                    token_mint: trade_token.mint,
                    user_token_account_key: *ctx.accounts.user_token_account.to_account_info().key,
                    amount: 0,
                    used_amount: 0,
                    liability: 0,
                };
                user.add_user_token(new_token, index)?;
                user.get_user_token_mut(&ctx.accounts.trade_token_vault.mint)?
                    .ok_or(BumpErrorCode::CouldNotFindUserToken)?
            },
            Some(exist_user_token) => exist_user_token,
        };

        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.trade_token_vault,
            &ctx.accounts.authority,
            transfer_amount,
        )?;

        if rewards_amount > 0 {
            utils::token::receive(
                &ctx.accounts.token_program,
                &ctx.accounts.pool_rewards_vault,
                &ctx.accounts.trade_token_vault,
                &ctx.accounts.authority,
                rewards_amount,
            )?;
        }

        user_token.add_token_amount(rewards_amount.safe_add(transfer_amount)?)?;
        trade_token.add_token(rewards_amount.safe_add(transfer_amount)?)?;

        let repay_liability = user_token.repay_liability()?;
        if repay_liability > 0 {
            trade_token.sub_liability(repay_liability)?;
        }

        let mut user_processor = UserProcessor { user };
        user_processor.update_cross_position_balance(
            &pool.pool_mint,
            rewards_amount.safe_add(transfer_amount)?.safe_sub(repay_liability)?,
            true,
        )?;
    } else {
        let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;

        utils::token::send_from_program_vault(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.user_token_account,
            &ctx.accounts.authority,
            bump_signer_nonce,
            transfer_amount,
        )?;

        if rewards_amount > 0 {
            utils::token::send_from_program_vault(
                &ctx.accounts.token_program,
                &ctx.accounts.pool_rewards_vault,
                &ctx.accounts.user_token_account,
                &ctx.accounts.authority,
                bump_signer_nonce,
                rewards_amount,
            )?;
        }
    }

    user.sub_user_stake(&pool.pool_key, un_stake_params.un_stake_token_amount)?;

    let user_stake = user.get_user_stake_mut(&pool.pool_key)?.ok_or(BumpErrorCode::StakePaused)?;

    let user = &mut ctx.accounts.user.load_mut()?;
    if user_stake.amount <= 0 {
        user.delete_user_stake(&user_stake.pool_key)?
    }

    Ok(())
}
