use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::constants::{RATE_PRECISION, SMALL_RATE_PRECISION};
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::optional_accounts::{AccountMaps, load_maps};
use crate::processor::position_processor;
use crate::processor::position_processor::DecreasePositionParams;
use crate::processor::user_processor::UserProcessor;
use crate::state::infrastructure::user_position::UserPosition;
use crate::state::market_map::MarketMap;
use crate::state::state::State;
use crate::state::user::User;
use crate::utils::pda;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    _user_authority_key: Pubkey
)]
pub struct LiquidateCrossPosition<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        seeds = [b"user", _user_authority_key.as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    pub keeper_signer: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_liquidate_cross_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, LiquidateCrossPosition<'c>>,
) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let state = &ctx.accounts.state;

    let remaining_accounts = ctx.remaining_accounts;

    let AccountMaps {
        market_map,
        trade_token_map,
        mut oracle_map,
        pool_map: pool_key_map,
        vault_map,
        ..
    } = load_maps(remaining_accounts, &state.admin)?;

    let mut user_processor = UserProcessor { user: &mut user };
    user_processor.cancel_all_cross_orders()?;
    drop(user_processor);

    let mut pos_infos: Vec<PosInfos> = Vec::new();
    for position in &user.user_positions {
        let infos = get_position_info(position, &market_map, state)?;
        pos_infos.push(infos)
    }
    for pos_info in &pos_infos {
        //only cross margin position support
        if !pos_info.cross_margin {
            continue;
        }

        let market = market_map.get_ref(&pos_info.symbol)?;
        let mut pool = pool_key_map.get_mut_ref(&market.pool_key)?;
        pool.update_pool_borrowing_fee_rate()?;

        let market = &mut market_map.get_mut_ref(&pos_info.symbol)?;
        let mut market_processor = MarketProcessor { market };
        let oracle = &trade_token_map.get_trade_token(&pos_info.margin_mint)?.oracle;
        market_processor.update_market_funding_fee_rate(
            &ctx.accounts.state,
            oracle_map.get_price_data(oracle).unwrap().price,
        )?;
    }

    let (cross_net_value, total_position_mm, total_size) = user.get_user_cross_net_value(
        &trade_token_map,
        &mut oracle_map,
        &market_map,
        &pool_key_map,
        &state,
    )?;

    let bankruptcy_mr = cal_utils::div_to_precision_i(
        cross_net_value,
        total_size.cast::<i128>()?,
        SMALL_RATE_PRECISION.cast::<i128>()?,
    )?
    .max(0i128);

    if cross_net_value <= 0 || cross_net_value.abs().cast::<u128>()? <= total_position_mm {
        for pos_info in &pos_infos {
            //only cross margin position support
            if !pos_info.cross_margin {
                continue;
            }

            let market = market_map.get_ref(&pos_info.symbol)?;
            let index_trade_token = trade_token_map.get_trade_token(&pos_info.index_mint)?;

            let index_price = oracle_map.get_price_data(&index_trade_token.oracle).unwrap().price;
            let bankruptcy_price = cal_utils::format_to_ticker_size(
                if pos_info.is_long {
                    cal_utils::mul_small_rate_u(
                        index_price,
                        SMALL_RATE_PRECISION
                            .cast::<i128>()?
                            .safe_sub(bankruptcy_mr)?
                            .abs()
                            .cast::<u128>()?,
                    )?
                } else {
                    cal_utils::mul_small_rate_u(
                        index_price,
                        SMALL_RATE_PRECISION
                            .cast::<i128>()?
                            .safe_add(bankruptcy_mr)?
                            .abs()
                            .cast::<u128>()?,
                    )?
                },
                market.market_trade_config.tick_size,
                pos_info.is_long,
            )?;

            validate!(bankruptcy_price > 0, BumpErrorCode::PriceIsNotAllowed)?;
            let liquidation_price = cal_utils::format_to_ticker_size(
                if pos_info.is_long {
                    cal_utils::div_rate_u(bankruptcy_price, RATE_PRECISION.safe_sub(pos_info.mm)?)?
                } else {
                    cal_utils::div_rate_u(bankruptcy_price, RATE_PRECISION.safe_add(pos_info.mm)?)?
                },
                market.market_trade_config.tick_size,
                pos_info.is_long,
            )?;

            let pool = pool_key_map.get_ref(&market.pool_key)?;
            let stable_pool = pool_key_map.get_ref(&market.stable_pool_key)?;
            let trade_token = trade_token_map.get_trade_token(&pos_info.margin_mint)?;

            position_processor::decrease_position1(
                DecreasePositionParams {
                    order_id: 0,
                    is_liquidation: true,
                    is_cross_margin: true,
                    margin_token: pos_info.margin_mint,
                    decrease_size: pos_info.position_size,
                    execute_price: liquidation_price,
                },
                &mut user,
                market_map.get_mut_ref(&pos_info.symbol)?.deref_mut(),
                pool_key_map.get_mut_ref(&market.pool_key)?.deref_mut(),
                pool_key_map.get_mut_ref(&market.stable_pool_key)?.deref_mut(),
                &ctx.accounts.state,
                None,
                if pos_info.is_long {
                    vault_map.get_account(&pda::generate_pool_vault_key(
                        pool.pool_index,
                        ctx.program_id,
                    )?)?
                } else {
                    vault_map.get_account(&pda::generate_pool_vault_key(
                        stable_pool.pool_index,
                        ctx.program_id,
                    )?)?
                },
                trade_token_map.get_trade_token(&pos_info.margin_mint)?,
                vault_map.get_account(&pda::generate_trade_token_vault_key(
                    trade_token.token_index,
                    ctx.program_id,
                )?)?,
                &ctx.accounts.bump_signer,
                &ctx.accounts.token_program,
                &mut oracle_map,
                &pos_info.user_key,
            )?;
        }
    }
    Ok(())
}

fn get_position_info(
    position: &UserPosition,
    market_map: &MarketMap,
    state: &State,
) -> BumpResult<PosInfos> {
    let market = market_map.get_ref(&position.symbol)?;

    Ok(PosInfos {
        cross_margin: position.cross_margin,
        symbol: position.symbol,
        index_mint: position.index_mint,
        is_long: position.is_long,
        margin_mint: position.margin_mint,
        position_size: position.position_size,
        user_key: position.user_key,
        mm: position.get_position_mm(&market, state)?,
    })
}

struct PosInfos {
    pub cross_margin: bool,
    pub symbol: [u8; 32],
    pub index_mint: Pubkey,
    pub is_long: bool,
    pub margin_mint: Pubkey,
    pub position_size: u128,
    pub user_key: Pubkey,
    pub mm: u128,
}
