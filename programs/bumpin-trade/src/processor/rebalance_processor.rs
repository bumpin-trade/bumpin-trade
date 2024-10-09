use anchor_lang::prelude::Program;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use std::ops::DerefMut;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::safe_math::SafeMath;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::pool_map::PoolMap;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::vault_map::VaultMap;
use crate::{utils, validate};

pub fn rebalance_pool_unsettle<'a>(
    state: &Account<'a, State>,
    pool_account_loader: &AccountLoader<'a, Pool>,
    trade_token_account_loader: &AccountLoader<'a, TradeToken>,
    pool_vault: &Account<'a, TokenAccount>,
    trade_token_vault: &Account<'a, TokenAccount>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
    _oracle_map: &mut OracleMap,
    market_map: &MarketMap,
    trade_token_map: &TradeTokenMap,
    pool_map: &PoolMap,
    vault_map: &VaultMap<'a>,
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

    if !pool.stable {
        //settle stable pool
        let markets = market_map.get_all_market()?;
        let mut market_loaded = vec![];
        for market_loader in markets {
            let market =
                market_loader.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadMarketData)?;
            if pool.key.eq(&market.pool_key) || pool.key.eq(&market.stable_pool_key) {
                market_loaded.push(market);
            }
        }
        validate!(
            pool.market_number == market_loaded.len() as u16,
            BumpErrorCode::MarketNumberNotEqual2Pool
        )?;
        for mut market in market_loaded {
            if !market.share_short {
                continue;
            }

            //rebalance stable_loss, pool to pool
            if market.stable_loss > 0i128 {
                // stable pool transfer to pool
                // todo need do swap
                let stable_loss = market.stable_loss;
                market.deref_mut().add_stable_loss(stable_loss.abs())?
            } else {
                //pool transfer to stable pool
                // todo need do swap
                let stable_loss = market.stable_loss;
                market.deref_mut().add_stable_loss(stable_loss.abs())?
            }

            //rebalance stable_loss_unsettle, portfolio to pool
            if market.stable_unsettle_loss > 0u128 {
                let stable_trade_pool = pool_map.get_ref(&market.stable_pool_key)?;
                let stable_pool_vault = vault_map.get_account(&stable_trade_pool.pool_vault_key)?;
                let stable_trade_token =
                    trade_token_map.get_trade_token_by_mint_ref(&market.stable_pool_mint_key)?;
                let stable_trade_token_vault =
                    vault_map.get_account(&stable_trade_token.vault_key)?;
                let transfer_amount = market.stable_unsettle_loss;
                utils::token::send_from_program_vault(
                    token_program,
                    &stable_trade_token_vault,
                    &stable_pool_vault,
                    bump_signer,
                    state.bump_signer_nonce,
                    transfer_amount,
                )
                .map_err(|_e| BumpErrorCode::InvalidTransfer)?;

                market.deref_mut().sub_unsettle_stable_loss(transfer_amount)?;
            }
        }
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
