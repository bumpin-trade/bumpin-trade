use anchor_lang::prelude::Program;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use std::panic::Location;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{token_to_usd_u, usd_to_token_u};
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::market::Market;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::utils::token;
use crate::validate;

pub fn rebalance_pool_unsettle<'a>(
    state: &Account<'a, State>,
    pool_account_loader: &AccountLoader<'a, Pool>,
    trade_token_account_loader: &AccountLoader<'a, TradeToken>,
    pool_vault: &Account<'a, TokenAccount>,
    trade_token_vault: &Account<'a, TokenAccount>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
    market_map: &MarketMap,
) -> BumpResult {
    let mut pool =
        pool_account_loader.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadPoolData)?;
    let trade_token = trade_token_account_loader
        .load()
        .map_err(|_e| BumpErrorCode::CouldNotLoadTradeTokenData)?;
    //balance fee_reward_unsettle
    let fee_reward_unsettle = pool.fee_reward.un_settle_fee_amount;
    if fee_reward_unsettle > 0u128 {
        token::send_from_program_vault(
            token_program,
            &trade_token_vault,
            &pool_vault,
            bump_signer,
            state.bump_signer_nonce,
            fee_reward_unsettle,
        )
            .map_err(|_e| BumpErrorCode::InvalidTransfer)?;
        pool.fee_reward.sub_un_settle_amount(fee_reward_unsettle)?;
        pool.fee_reward.add_fee_amount(fee_reward_unsettle)?;
    }
    validate!(
        pool.balance.un_settle_amount >= trade_token.total_liability,
        BumpErrorCode::PoolUnsettleSmallThanTokenLiability
    )?;

    //balance pool unsettle
    let transfer_amount = pool.balance.un_settle_amount.safe_sub(trade_token.total_liability)?;
    token::send_from_program_vault(
        token_program,
        &trade_token_vault,
        &pool_vault,
        bump_signer,
        state.bump_signer_nonce,
        transfer_amount,
    )
        .map_err(|_e| BumpErrorCode::InvalidTransfer)?;
    pool.sub_unsettle(transfer_amount)?;
    pool.add_amount(transfer_amount)?;
    if pool.stable {
        //settle market stable_loss_unsettle
        let markets = market_map.get_all_market()?;
        let mut market_loaded = vec![];
        for market_loader in markets {
            let market = market_loader.load_mut().map_err(|e| {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!("Could not load trade_token at {}:{}", caller.file(), caller.line());
                BumpErrorCode::CouldNotLoadMarketData
            })?;
            if pool.key.eq(&market.pool_key) || pool.key.eq(&market.stable_pool_key) {
                market_loaded.push(market);
            }
        }
        validate!(
            pool.market_number == market_loaded.len() as u16,
            BumpErrorCode::MarketNumberNotEqual2Pool
        )?;
        let mut remaining_amount = transfer_amount;
        for mut market in market_loaded {
            if !market.share_short || market.stable_unsettle_loss == 0u128 {
                continue;
            }
            if remaining_amount == 0u128 { break; }
            remaining_amount = market.sub_unsettle_stable_loss(transfer_amount)?;
        }
    }
    Ok(())
}

pub fn rebalance_market_stable_loss<'a>(
    state: &Account<'a, State>,
    pool_loader: &AccountLoader<'a, Pool>,
    pool_vault: &Account<'a, TokenAccount>,
    stable_pool_vault: &Account<'a, TokenAccount>,
    trade_token_loader: &AccountLoader<'a, TradeToken>,
    stable_trade_token_loader: &AccountLoader<'a, TradeToken>,
    keeper_trade_token_vault: &Account<'a, TokenAccount>,
    keeper_stable_trade_token_vault: &Account<'a, TokenAccount>,
    keeper_signer: &Signer<'a>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
    oracle_map: &mut OracleMap,
    market_account_loader: &AccountLoader<'a, Market>,
) -> BumpResult {
    let mut market =
        market_account_loader.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadMarketData)?;
    let mut pool = pool_loader.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadPoolData)?;
    let trade_token =
        trade_token_loader.load().map_err(|_e| BumpErrorCode::CouldNotLoadTradeTokenData)?;
    let stable_trade_token =
        stable_trade_token_loader.load().map_err(|_e| BumpErrorCode::CouldNotLoadTradeTokenData)?;

    validate!(market.stable_loss != 0i128, BumpErrorCode::RebalanceMarketStableLossIgnore)?;

    //rebalance stable_loss, pool to pool
    if market.stable_loss > 0i128 {
        // stable pool transfer to pool
        let stable_loss = market.stable_loss;
        let stable_trade_token_price =
            oracle_map.get_price_data(&stable_trade_token.oracle_key)?.price;
        let stable_loss_value = token_to_usd_u(
            stable_loss.abs().cast()?,
            stable_trade_token.decimals,
            stable_trade_token_price,
        )?;
        let trade_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
        let trade_token_amount =
            usd_to_token_u(stable_loss_value, trade_token.decimals, trade_token_price)?;

        //receive token from keeper
        token::receive(
            token_program,
            keeper_trade_token_vault,
            pool_vault,
            keeper_signer,
            trade_token_amount,
        )
        .map_err(|_e| BumpErrorCode::TransferFailed)?;
        pool.add_amount(trade_token_amount)?;

        //transfer stable_token to keeper
        token::send_from_program_vault(
            token_program,
            stable_pool_vault,
            keeper_stable_trade_token_vault,
            bump_signer,
            state.bump_signer_nonce,
            stable_loss.abs().cast()?,
        )
        .map_err(|_e| BumpErrorCode::InvalidTransfer)?;

        market.add_stable_loss(stable_loss.abs())?
    } else if market.stable_loss < 0i128 {
        //pool transfer to stable pool
        let stable_loss = market.stable_loss;
        let stable_trade_token_price =
            oracle_map.get_price_data(&stable_trade_token.oracle_key)?.price;
        let stable_loss_value = token_to_usd_u(
            stable_loss.abs().cast()?,
            stable_trade_token.decimals,
            stable_trade_token_price,
        )?;
        let trade_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
        let trade_token_amount =
            usd_to_token_u(stable_loss_value, trade_token.decimals, trade_token_price)?;

        //receive stable_token from keeper
        token::receive(
            token_program,
            keeper_stable_trade_token_vault,
            stable_pool_vault,
            keeper_signer,
            stable_loss.abs().cast()?,
        )
        .map_err(|_e| BumpErrorCode::TransferFailed)?;

        //transfer token to keeper
        token::send_from_program_vault(
            token_program,
            pool_vault,
            keeper_trade_token_vault,
            bump_signer,
            state.bump_signer_nonce,
            trade_token_amount,
        )
        .map_err(|_e| BumpErrorCode::InvalidTransfer)?;
        pool.sub_amount(trade_token_amount)?;
        market.add_stable_loss(stable_loss.abs())?
    }
    Ok(())
}
