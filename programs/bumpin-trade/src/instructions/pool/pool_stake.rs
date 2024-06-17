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
use crate::processor::{fee_processor, user_processor};
use crate::state::infrastructure::user_stake::{UserStake, UserStakeStatus};
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::{utils, validate};

#[derive(Accounts)]
#[instruction(pool_index: u16, trade_token_index: u16)]
pub struct PoolStake<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Account<'info, State>,
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
        mut,
        constraint = pool_vault.mint.key().eq(& user_token_account.mint.key()) && trade_token_vault.mint.eq(& user_token_account.mint.key()),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct StakeParams {
    request_token_amount: u128,
    min_stake_amount: u128,
    is_native_token: bool,
    portfolio: bool,
}

pub fn handle_pool_stake<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, PoolStake>,
    stake_params: StakeParams,
) -> Result<()> {
    let mut pool = &mut ctx.accounts.pool.load_mut()?;
    let mut user = &mut ctx.accounts.user.load_mut()?;
    validate!(
        pool.pool_config.mini_stake_amount > stake_params.request_token_amount,
        BumpErrorCode::StakeToSmall
    )?;

    let user_stake_option = user.get_user_stake_mut(&pool.pool_key)?;
    //make sure user_stake exist
    match user_stake_option {
        None => {
            //add default user_stake to user
            let res = &mut UserStake {
                user_stake_status: UserStakeStatus::USING,
                pool_key: pool.pool_key,
                amount: 0,
                user_rewards: Default::default(),
            };

            let next_index = user.next_usable_stake_index()?;
            user.add_user_stake(res, next_index)?;
            res
        },
        Some(user_stake) => user_stake,
    };
    let trade_token = ctx.accounts.trade_token.load()?;

    let remaining_accounts_iter: &mut Peekable<Iter<'info, AccountInfo<'info>>> =
        &mut ctx.remaining_accounts.iter().peekable();
    let mut account_maps = load_maps(remaining_accounts_iter)?;

    update_account_fee_reward(&ctx.accounts.user, &ctx.accounts.pool)?;

    let stake_fee = fee_processor::collect_stake_fee(&mut pool, stake_params.request_token_amount)?;
    let base_mint_amount = stake_params.request_token_amount.safe_sub(stake_fee)?;
    if stake_params.portfolio {
        let mut pool_processor = PoolProcessor { pool };
        pool_processor.portfolio_to_stake(
            &ctx.accounts.user,
            &ctx.accounts.pool,
            base_mint_amount,
            &trade_token,
            &mut account_maps,
        )?;
        drop(pool_processor);
        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.trade_token_vault,
            &ctx.accounts.pool_vault,
            &ctx.accounts.authority,
            stake_params.request_token_amount,
        )?;
        ctx.accounts.trade_token_vault.reload()?;
    } else {
        let mut pool_processor = PoolProcessor { pool };
        pool_processor.stake(
            &ctx.accounts.user,
            &ctx.accounts.pool,
            base_mint_amount,
            &trade_token,
            &mut account_maps,
        )?;
        drop(pool_processor);
        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            &ctx.accounts.pool_vault,
            &ctx.accounts.authority,
            stake_params.request_token_amount,
        )?;
        ctx.accounts.pool_vault.reload()?;
    }

    Ok(())
}
