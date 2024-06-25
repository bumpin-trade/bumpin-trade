use std::collections::BTreeMap;

use anchor_lang::prelude::Program;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::swap;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::optional_accounts::AccountMaps;
use crate::utils;

pub fn rebalance_pool_unsettle<'a>(
    account_maps: &mut AccountMaps<'a>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
) -> BumpResult {
    let trade_token_map = &account_maps.trade_token_map;
    let pool_map = &account_maps.pool_map;
    let vault_map = &account_maps.vault_map;
    let trade_token_vec = trade_token_map.get_all_trade_token()?;
    let mut pool_vec = pool_map.get_all_pool()?;

    let mut pool_unsettle_map = BTreeMap::<Pubkey, u128>::new();
    for pool in &mut pool_vec {
        if !pool.stable {
            match pool_unsettle_map.get_mut(&pool.pool_mint) {
                Some(un_settle_amount) => {
                    let total_unsettle_amount =
                        un_settle_amount.safe_add(pool.pool_balance.un_settle_amount)?;
                    pool_unsettle_map.insert(pool.pool_mint, total_unsettle_amount);
                },
                None => {
                    pool_unsettle_map.insert(pool.pool_mint, pool.pool_balance.un_settle_amount);
                },
            }
        }

        if pool.fee_reward.un_settle_fee_amount > 0 {
            let pool_vault = vault_map.get_account(&pool.pool_mint_vault)?;
            let trade_token = trade_token_map.get_trade_token(&pool.pool_mint)?;
            let trade_token_vault = vault_map.get_account(&trade_token.trade_token_vault)?;

            utils::token::receive(
                token_program,
                trade_token_vault,
                pool_vault,
                bump_signer,
                pool.fee_reward.un_settle_fee_amount,
            )
            .map_err(|_e| {
                return BumpErrorCode::InvalidTransfer;
            })?;

            pool.fee_reward.sub_un_settle_amount(pool.fee_reward.un_settle_fee_amount)?
        }
    }

    for trade_token in &trade_token_vec {
        match pool_unsettle_map.get(&trade_token.mint) {
            Some(total_unsettle_amount) => {
                if trade_token.total_liability < *total_unsettle_amount {
                    let mut transfer_amount: u128 =
                        total_unsettle_amount.safe_sub(trade_token.total_liability)?;
                    if transfer_amount > 0 {
                        let trade_token_vault =
                            vault_map.get_account(&trade_token.trade_token_vault)?;
                        for pool in &mut pool_vec {
                            if pool.pool_mint.eq(&trade_token.mint) && transfer_amount > 0 {
                                let pool_transfer_amount =
                                    if pool.pool_balance.un_settle_amount > transfer_amount {
                                        transfer_amount
                                    } else {
                                        pool.pool_balance.un_settle_amount
                                    };

                                let pool_vault = vault_map.get_account(&pool.pool_mint_vault)?;

                                utils::token::receive(
                                    token_program,
                                    trade_token_vault,
                                    pool_vault,
                                    bump_signer,
                                    pool_transfer_amount,
                                )
                                .map_err(|_e| {
                                    return BumpErrorCode::InvalidTransfer;
                                })?;

                                pool.sub_unsettle(pool_transfer_amount)?;
                                transfer_amount = transfer_amount.safe_sub(pool_transfer_amount)?;
                            }
                        }
                    }
                }
            },
            _ => {},
        }
    }

    for pool in &mut pool_vec {
        if pool.stable {
            let trade_token = trade_token_map.get_trade_token(&pool.pool_mint)?;
            if trade_token.total_liability < pool.pool_balance.un_settle_amount {
                let transfer_amount =
                    pool.pool_balance.un_settle_amount - trade_token.total_liability;

                let stable_pool_vault = vault_map.get_account(&pool.pool_mint_vault)?;
                let trade_token_vault = vault_map.get_account(&trade_token.trade_token_vault)?;

                utils::token::receive(
                    token_program,
                    trade_token_vault,
                    stable_pool_vault,
                    bump_signer,
                    transfer_amount,
                )
                .map_err(|_e| {
                    return BumpErrorCode::InvalidTransfer;
                })?;

                pool.sub_unsettle(transfer_amount)?;
            }
        }
    }
    Ok(())
}

pub fn rebalance_stable_pool<'a>(
    account_maps: &mut AccountMaps<'a>,
    _bump_signer: &AccountInfo<'a>,
    _token_program: &Program<'a, Token>,
) -> BumpResult {
    let pool_map = &account_maps.pool_map;
    let pool_vec = pool_map.get_all_pool()?;
    for mut pool in pool_vec {
        if pool.stable_balance.amount >= pool.stable_balance.loss_amount {
            pool.sub_amount(pool.stable_balance.loss_amount)?;
            pool.sub_loss_amount(pool.stable_balance.loss_amount)?;

            let _transfer_amount =
                pool.stable_balance.amount.safe_sub(pool.stable_balance.loss_amount)?;
            let swap_amount = swap::jup_swap()?;

            pool.add_amount(swap_amount)?;
            pool.add_pnl(swap_amount.cast()?)?;
        } else {
            pool.sub_amount(pool.stable_balance.amount)?;
            pool.sub_loss_amount(pool.stable_balance.amount)?;

            let _transfer_amount =
                pool.stable_balance.loss_amount.safe_sub(pool.stable_balance.amount)?;
            let swap_amount = swap::jup_swap()?;

            pool.sub_amount(swap_amount)?;
            pool.add_pnl(-swap_amount.cast::<i128>()?)?;
        }
    }
    Ok(())
}

pub fn rebalance_rewards<'a>(
    account_maps: &mut AccountMaps<'a>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
) -> BumpResult {
    let pool_map = &account_maps.pool_map;
    let pool_vec = pool_map.get_all_pool()?;
    let vault_map = &account_maps.vault_map;
    let trade_token_map = &account_maps.trade_token_map;
    for mut pool in pool_vec {
        if pool.fee_reward.un_settle_fee_amount > 0 {
            let stable_pool_vault = vault_map.get_account(&pool.pool_mint_vault)?;
            let trade_token = trade_token_map.get_trade_token(&pool.pool_mint)?;
            let trade_token_vault = vault_map.get_account(&trade_token.trade_token_vault)?;

            utils::token::receive(
                token_program,
                trade_token_vault,
                stable_pool_vault,
                bump_signer,
                pool.fee_reward.un_settle_fee_amount,
            )
            .map_err(|_e| {
                return BumpErrorCode::InvalidTransfer;
            })?;

            pool.fee_reward.sub_un_settle_amount(pool.fee_reward.un_settle_fee_amount)?;
        }
    }
    Ok(())
}

pub fn auto_rebalance<'a>(
    account_maps: &mut AccountMaps<'a>,
    bump_signer: &AccountInfo<'a>,
    token_program: &Program<'a, Token>,
) -> BumpResult {
    rebalance_pool_unsettle(account_maps, bump_signer, token_program)?;
    rebalance_rewards(account_maps, bump_signer, token_program)?;
    rebalance_stable_pool(account_maps, bump_signer, token_program)?;
    Ok(())
}
