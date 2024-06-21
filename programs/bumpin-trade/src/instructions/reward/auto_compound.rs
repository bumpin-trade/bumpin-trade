use crate::errors::BumpErrorCode;
use anchor_lang::prelude::*;

use crate::instructions::StakeParams;
use crate::processor::optional_accounts::load_maps;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::stake_processor;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::validate;

#[derive(Accounts)]
#[instruction(_stable_trade_token_index: u16,)]
pub struct AutoCompoundRewards<'info> {
    #[account(
        mut,
        seeds = [b"user", authority.key().as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Box<Account<'info, State>>,

    #[account(
        mut,
        seeds = [b"trade_token".as_ref(), _stable_trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_trade_token: AccountLoader<'info, TradeToken>,
}

pub fn handle_auto_compound<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, AutoCompoundRewards<'c>>,
    _stable_trade_token_index: u16,
) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;
    validate!(user.authority.eq(&ctx.accounts.authority.owner), BumpErrorCode::UserNotFound)?;
    let stable_trade_token = ctx.accounts.stable_trade_token.load()?;

    let remaining_accounts = ctx.remaining_accounts;
    for user_stake in user.user_stakes.iter_mut() {
        let account_maps = load_maps(remaining_accounts, &ctx.accounts.state.admin)?;
        let pool_key_map = &account_maps.pool_map;
        let pool_account_loader = pool_key_map.get_account_loader(&user_stake.pool_key)?;

        let pool = &mut pool_account_loader.load_mut()?;
        let trade_token_loader =
            account_maps.trade_token_map.get_account_loader(&pool.pool_mint)?;

        let trade_token = trade_token_loader.load()?;

        let account_maps = &mut load_maps(remaining_accounts, &ctx.accounts.state.admin)?;
        let stake_amount = stake_processor::stake(
            &pool_account_loader,
            &ctx.accounts.user,
            &trade_token_loader,
            account_maps,
            &StakeParams {
                request_token_amount: user_stake.user_rewards.realised_rewards_token_amount,
                portfolio: false,
            },
        )?;
        user_stake.user_rewards.realised_rewards_token_amount = 0;
        user_stake.user_rewards.open_rewards_per_stake_token =
            pool.fee_reward.cumulative_rewards_per_stake_token;

        let account_maps = &mut load_maps(remaining_accounts, &ctx.accounts.state.admin)?;
        let mut pool_processor = PoolProcessor { pool };
        pool_processor.stake(
            &ctx.accounts.user,
            pool_account_loader,
            stake_amount,
            &trade_token,
            &stable_trade_token,
            account_maps,
        )?;
    }
    Ok(())
}
