use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::{utils, validate};
use crate::errors::{BumpErrorCode};
use crate::math::safe_math::SafeMath;
use crate::processor::fee_processor;
use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::user_processor::UserProcessor;
use crate::state::market_map::MarketMap;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::can_sign_for_user;

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

pub fn handle_pool_stake(mut ctx: Context<PoolStake>, pool_index: usize, trade_token_index: u16, stake_params: StakeParams) -> anchor_lang::Result<()> {
    let mut pool = &mut ctx.accounts.pool.load_mut()?;
    validate!(pool.pool_config.mini_stake_amount>stake_params.request_token_amount, BumpErrorCode::StakeToSmall);

    let mut user = &mut ctx.accounts.user.load_mut()?;
    let trade_token = ctx.accounts.trade_token.load()?;
    let remaining_accounts = &mut ctx.remaining_accounts.iter().peekable();
    let mut oracle_map = OracleMap::load(remaining_accounts)?;
    let market_map = MarketMap::load(remaining_accounts)?;

    let user_stake = user.get_user_stake_mut(pool_index)?;
    update_account_fee_reward(user_stake, &pool)?;

    if stake_params.portfolio {
        let mut user_processor = UserProcessor { user };

        let mut user_token = user.get_user_token_mut(&pool.pool_mint)?;
        validate!(user_token.amount>stake_params.request_token_amount, BumpErrorCode::AmountNotEnough);

        let mut trade_token_map = TradeTokenMap::load(remaining_accounts)?;
        validate!(  user_processor.get_available_value(&mut oracle_map, &mut trade_token_map)?>0, BumpErrorCode::AmountNotEnough);

        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.trade_token_vault,
            &ctx.accounts.pool_vault,
            &ctx.accounts.authority,
            stake_params.request_token_amount,
        )?;
        ctx.accounts.trade_token_vault.reload()?;
        user_processor.sub_user_token_amount(user_token, stake_params.request_token_amount)?;
    } else {
        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.user_token_account,
            &ctx.accounts.pool_vault,
            &ctx.accounts.authority,
            stake_params.request_token_amount,
        )?;
    }
    ctx.accounts.pool_vault.reload()?;

    let stake_fee = fee_processor::collect_stake_fee(&mut pool,
                                                     &mut ctx.accounts.state,
                                                     stake_params.request_token_amount)?;
    let base_mint_amount = stake_params.request_token_amount.safe_sub(stake_fee)?;

    let mut pool_processor = PoolProcessor { pool };
    let stake_amount = pool_processor.stake(base_mint_amount, &mut oracle_map, &market_map, &trade_token)?;
    validate!(stake_amount < stake_params.min_stake_amount, BumpErrorCode::StakeToSmall);

    user_stake.add_user_stake(stake_amount)?;
    Ok(())
}