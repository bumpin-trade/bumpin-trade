use anchor_lang::prelude::Program;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::safe_math::SafeMath;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::utils;

pub fn rebalance_pool_unsettle<'a>(
    state: &Account<'a, State>,
    pool_account_loader: &AccountLoader<'a, Pool>,
    trade_token_account_loader: &AccountLoader<'a, TradeToken>,
    pool_vault: &Account<'a, TokenAccount>,
    trade_token_vault: &Account<'a, TokenAccount>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
) -> BumpResult {
    let mut pool =
        pool_account_loader.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadPoolData)?;
    let mut trade_token = trade_token_account_loader
        .load_mut()
        .map_err(|_e| BumpErrorCode::CouldNotLoadTradeTokenData)?;
    //balance fee_reward_unsettle
    let fee_reward_unsettle = pool.fee_reward.un_settle_fee_amount;
    utils::token::send_from_program_vault(
        token_program,
        &trade_token_vault,
        &pool_vault,
        bump_signer,
        state.bump_signer_nonce,
        fee_reward_unsettle,
    )
    .map_err(|_e| BumpErrorCode::InvalidTransfer)?;
    pool.fee_reward.sub_un_settle_amount(fee_reward_unsettle)?;
    //balance pool unsettle
    if pool.balance.un_settle_amount >= trade_token.total_liability {
        let transfer_amount =
            pool.balance.un_settle_amount.safe_sub(trade_token.total_liability)?;
        utils::token::send_from_program_vault(
            token_program,
            &trade_token_vault,
            &pool_vault,
            bump_signer,
            state.bump_signer_nonce,
            transfer_amount,
        )
        .map_err(|_e| BumpErrorCode::InvalidTransfer)?;
        pool.sub_unsettle(transfer_amount)?;
        trade_token.total_liability = 0u128;
    } else {
        let transfer_amount =
            trade_token.total_liability.safe_sub(pool.balance.un_settle_amount)?;
        utils::token::send_from_program_vault(
            token_program,
            &pool_vault,
            &trade_token_vault,
            bump_signer,
            state.bump_signer_nonce,
            transfer_amount,
        )
        .map_err(|_e| BumpErrorCode::InvalidTransfer)?;
        pool.balance.un_settle_amount = 0u128;
        trade_token.sub_total_liability(transfer_amount)?;
    }
    Ok(())
}

pub fn rebalance_rewards<'a>(
    state: &Account<'a, State>,
    pool_account_loader: &AccountLoader<'a, Pool>,
    pool_vault: &Account<'a, TokenAccount>,
    trade_token_vault: &Account<'a, TokenAccount>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
) -> BumpResult {
    let mut pool =
        pool_account_loader.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadPoolData)?;
    //balance fee_reward_unsettle
    let fee_reward_unsettle = pool.fee_reward.un_settle_fee_amount;
    utils::token::send_from_program_vault(
        token_program,
        &trade_token_vault,
        &pool_vault,
        bump_signer,
        state.bump_signer_nonce,
        fee_reward_unsettle,
    )
    .map_err(|_e| BumpErrorCode::InvalidTransfer)?;
    pool.fee_reward.sub_un_settle_amount(fee_reward_unsettle)
}

pub fn auto_rebalance<'a>(
    state: &Account<'a, State>,
    pool_account_loader: &AccountLoader<'a, Pool>,
    trade_token_account_loader: &AccountLoader<'a, TradeToken>,
    pool_vault: &Account<'a, TokenAccount>,
    trade_token_vault: &Account<'a, TokenAccount>,
    _stable_pool_vault: &Account<'a, TokenAccount>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
) -> BumpResult {
    rebalance_pool_unsettle(
        state,
        pool_account_loader,
        trade_token_account_loader,
        pool_vault,
        trade_token_vault,
        bump_signer,
        token_program,
    )?;
    rebalance_rewards(
        state,
        pool_account_loader,
        pool_vault,
        trade_token_vault,
        bump_signer,
        token_program,
    )?;
    Ok(())
}
