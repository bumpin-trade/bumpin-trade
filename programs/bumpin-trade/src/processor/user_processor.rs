use anchor_lang::prelude::*;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::calculator;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::UserRewardsUpdateEvent;
use crate::state::infrastructure::user_position::PositionStatus;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::{User, UserTokenUpdateReason};
use crate::validate;
#[track_caller]
pub fn withdraw(
    user: &mut User,
    amount: u128,
    token_mint: &Pubkey,
    oracle_map: &mut OracleMap,
    trade_tokens: &TradeTokenMap,
) -> BumpResult {
    if user.cross_used()? {
        let trade_token = trade_tokens.get_trade_token_by_mint_ref(token_mint)?;
        let price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
        let withdraw_usd = calculator::token_to_usd_u(amount, trade_token.decimals, price)?;

        let available_value = user.get_available_value(trade_tokens, oracle_map)?;

        validate!(
            available_value.abs().cast::<u128>()? > withdraw_usd,
            BumpErrorCode::UserNotEnoughValue
        )?;
    }
    user.sub_user_token_amount_ignore_used_amount(
        token_mint,
        amount,
        &UserTokenUpdateReason::WITHDRAW,
    )?;
    update_cross_position_balance(user, token_mint, amount, false)?;
    Ok(())
}

pub fn update_cross_position_balance(
    user: &mut User,
    mint: &Pubkey,
    amount: u128,
    add_amount: bool,
) -> BumpResult<()> {
    let mut reduce_amount = amount;
    for user_position in user.positions.iter_mut() {
        if user_position.status.eq(&PositionStatus::INIT) {
            continue;
        }
        if user_position.is_portfolio_margin
            && user_position.margin_mint_key.eq(mint)
            && reduce_amount > 0
        {
            if add_amount {
                let change_amount = user_position.add_position_portfolio_balance(reduce_amount)?;
                reduce_amount = reduce_amount.safe_sub(change_amount)?;
            } else {
                let change_amount =
                    user_position.reduce_position_portfolio_balance(reduce_amount)?;
                reduce_amount = reduce_amount.safe_sub(change_amount)?;
            }
        }

        if reduce_amount == 0u128 {
            break;
        }
    }
    Ok(())
}

pub fn update_account_fee_reward(stake_pool: &mut Pool, user: &mut User) -> BumpResult {
    let user_key = user.key;

    let user_stake = user.get_user_stake_mut_ref(&stake_pool.key)?;

    let fee_reward = stake_pool.fee_reward;
    // if user_stake.user_rewards.open_rewards_per_stake_token
    //     != fee_reward.cumulative_rewards_per_stake_token
    //     && user_stake.staked_share > 0
    //     && fee_reward
    //         .cumulative_rewards_per_stake_token
    //         .safe_sub(user_stake.user_rewards.open_rewards_per_stake_token)?
    //         > fee_reward.get_rewards_delta_limit()?
    // {
    let realised_rewards_token_amount = stake_pool
        .fee_reward
        .cumulative_rewards_per_stake_token
        .safe_sub(user_stake.user_rewards.open_rewards_per_stake_token)?
        .safe_mul_per_rate(user_stake.staked_share)?;
    user_stake.add_user_rewards(realised_rewards_token_amount)?;
    user_stake.user_rewards.open_rewards_per_stake_token =
        fee_reward.cumulative_rewards_per_stake_token;
    // }
    let user_rewards = user_stake.user_rewards.clone();
    emit!(UserRewardsUpdateEvent { user_key, token_mint: stake_pool.key, user_rewards });
    Ok(())
}
