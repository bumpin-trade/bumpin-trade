use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::processor::optional_accounts::AccountMaps;
use crate::utils;
use anchor_lang::prelude::{Program, Signer};
use anchor_spl::token::Token;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use std::collections::BTreeMap;

pub fn re_balance_pool_unsettle(
    account_maps: &mut AccountMaps,
    bump_signer: &AccountInfo,
    token_program: &Program<Token>,
) -> BumpResult {
    let trade_token_map = &account_maps.trade_token_map;
    let pool_map = &account_maps.pool_map;
    let vault_map = &account_maps.vault_map;
    let trade_token_vec = trade_token_map.get_all_trade_token()?;
    let pool_vec = pool_map.get_all_pool()?;

    let mut pool_unsettle_map = BTreeMap::<Pubkey, u128>::new();
    for mut pool in pool_vec {
        if !pool.stable {
            match pool_unsettle_map.get_mut(&pool.pool_mint) {
                Some(un_settle_amount) => {
                    let total_unsettle_amount =
                        un_settle_amount.safe_add(pool.pool_balance.un_settle_amount)?;
                    pool_unsettle_map.insert(pool.pool_mint, total_unsettle_amount);
                }
                None => {
                    pool_unsettle_map.insert(pool.pool_mint, pool.pool_balance.un_settle_amount);
                }
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
            )?;

            pool.fee_reward.sub_un_settle_amount(pool.fee_reward.un_settle_fee_amount)?
        }
    }

    for trade_token in trade_token_vec {
        match pool_unsettle_map.get(&trade_token.mint) {
            Some(total_unsettle_amount) => {
                if trade_token.total_liability < *total_unsettle_amount {
                    let mut transfer_amount: u128 = *total_unsettle_amount.safe_sub(
                        trade_token.total_liability)?;
                    if transfer_amount > 0 {
                        let trade_token_vault = vault_map.get_account(&trade_token.trade_token_vault)?;
                        for mut pool in pool_vec {
                            if pool.pool_mint.eq(&trade_token.mint) && transfer_amount > 0 {
                                let pool_transfer_amount = if pool.pool_balance.un_settle_amount > transfer_amount { transfer_amount } else { pool.pool_balance.un_settle_amount };

                                let pool_vault = vault_map.get_account(&pool.pool_mint_vault)?;

                                utils::token::receive(
                                    token_program,
                                    trade_token_vault,
                                    pool_vault,
                                    bump_signer,
                                    pool_transfer_amount,
                                )?;

                                pool.sub_unsettle(pool_transfer_amount)?;
                                transfer_amount = transfer_amount.safe_sub(pool_transfer_amount)?;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    for mut pool in pool_vec {
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
                )?;

                pool.sub_unsettle(transfer_amount)?;
            }
        }
    }
    Ok(())
}


pub fn auto_re_balance(account_maps: &mut AccountMaps,
                       bump_signer: &AccountInfo,
                       token_program: &Program<Token>, ) -> BumpResult {
    re_balance_pool_unsettle(account_maps, bump_signer, token_program)?;


    Ok(())
}
