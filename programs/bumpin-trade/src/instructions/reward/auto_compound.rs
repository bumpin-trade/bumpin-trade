use crate::errors::BumpErrorCode;
use anchor_lang::prelude::*;

use crate::instructions::StakeParams;
use crate::processor::optional_accounts::load_maps;
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::stake_processor;
use crate::state::user::User;
use crate::validate;

#[derive(Accounts)]
pub struct AutoCompoundRewards<'info> {
    pub user: AccountLoader<'info, User>,

    pub authority: Signer<'info>,
}

pub fn claim_rewards<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, AutoCompoundRewards<'c>>,
) -> Result<()> {
    let user = &mut ctx.accounts.user.load_mut()?;
    validate!(user.authority.eq(&ctx.accounts.authority.owner), BumpErrorCode::UserNotFound)?;

    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
    for user_stake in user.user_stakes.iter_mut() {
        let account_maps = load_maps(remaining_accounts_iter)?;
        let pool_key_map = &account_maps.pool_map;
        let pool_account_loader = pool_key_map.get_account_loader(&user_stake.pool_key)?;

        let pool = &mut pool_account_loader.load_mut()?;
        let trade_token_loader =
            account_maps.trade_token_map.get_account_loader(&pool.pool_mint)?;
        let trade_token = trade_token_loader.load()?;

        let account_maps = &mut load_maps(remaining_accounts_iter)?;
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

        let account_maps = &mut load_maps(remaining_accounts_iter)?;
        let mut pool_processor = PoolProcessor { pool };
        pool_processor.stake(
            &ctx.accounts.user,
            pool_account_loader,
            stake_amount,
            &trade_token,
            account_maps,
        )?;
    }
    Ok(())
}
