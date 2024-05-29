use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::{utils, validate};
use crate::errors::BumpErrorCode::UnStakeNotEnough;
use crate::math::safe_math::SafeMath;
use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::processor::pool_processor::PoolProcessor;
use crate::state::market_map::MarketMap;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::can_sign_for_user;

#[derive(Accounts)]
#[instruction(pool_index: u16, trade_token_index: u16)]
pub struct PoolUnStake<'info> {
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
        seeds = [b"pool_mint_vault".as_ref(), pool_index.to_le_bytes.as_ref()],
        bump,
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), trade_token_index.to_le_bytes.as_ref()],
        bump,
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = & pool_vault.mint.eq(& user_token_account.mint) && & trade_token_vault.mint.eq(& user_token_account.mint),
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

pub fn handle_pool_un_stake(mut ctx: Context<PoolUnStake>, pool_index: u16, un_stake_params: UnStakeParams) -> Result<()> {
    let mut pool = &mut ctx.accounts.pool.load_mut()?;
    let mut user = &mut ctx.accounts.user.load_mut()?;
    let mut state = &mut ctx.accounts.state.load_mut()?;

    let user_stake = user.get_user_stake_mut(pool_index)?;
    validate!(user_stake.amount>=un_stake_params.un_stake_token_amount, UnStakeNotEnough);

    let remaining_accounts = &mut ctx.remaining_accounts.iter().peekable();
    let mut oracle_map = OracleMap::load(remaining_accounts)?;

    let mut pool_processor = PoolProcessor { pool };
    let pool_value = pool_processor.get_pool_usd_value(&mut oracle_map)?;
    validate!(pool.total_supply==0||pool_value==0, UnStakeNotEnough);

    let market_vec = MarketMap::load(remaining_accounts)?;
    let un_stake_token_amount = pool_processor.un_stake(un_stake_params.un_stake_token_amount, &mut oracle_map, pool_value)?;
    validate!(un_stake_token_amount>pool.pool_config.mini_un_stake_amount, UnStakeNotEnough);


    let max_un_stake_amount = pool.get_current_max_un_stake()?;
    validate!(un_stake_token_amount<max_un_stake_amount, UnStakeNotEnough);

    let un_stake_token_amount_fee = pool_processor.collect_un_stake_fee(&mut state, un_stake_token_amount)?;

    update_account_fee_reward(user_stake, &pool);
    let user_stake = user.get_user_stake_mut(pool_index)?;

    let transfer_amount = un_stake_token_amount.safe_sub(un_stake_token_amount_fee)?;
    if un_stake_params.portfolio {
        let mut user_token = user.get_user_token_mut(&pool.pool_mint)?;

        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.trade_token_vault,
            &ctx.accounts.authority,
            transfer_amount,
        )?;

        ctx.accounts.pool_vault.reload()?;
        ctx.accounts.trade_token_vault.reload()?;
        user_token.add_token_amount(transfer_amount);
    } else {
        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.user_token_account,
            &ctx.accounts.authority,
            transfer_amount,
        )?;
        ctx.accounts.pool_vault.reload()?;
    }
    user_stake.sub_user_stake(un_stake_params.un_stake_token_amount);
    Ok(())
}