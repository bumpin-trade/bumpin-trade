use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode;
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::constants::{RATE_PRECISION, SMALL_RATE_PRECISION};
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::position_processor::{DecreasePositionParams, PositionProcessor};
use crate::processor::user_processor::UserProcessor;
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
    let user = &mut ctx.accounts.user.load_mut()?;
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

    let mut user_processor = UserProcessor { user };
    user_processor.cancel_all_cross_orders()?;

    for user_position in &user_processor.user.user_positions {
        //only cross margin position support
        if !user_position.cross_margin {
            continue;
        }

        let market = market_map.get_ref(&user_position.symbol)?;
        let pool = &mut pool_key_map.get_mut_ref(&market.pool_key)?;
        let mut pool_processor = PoolProcessor { pool };
        pool_processor.update_pool_borrowing_fee_rate()?;
        drop(pool_processor);

        let market = &mut market_map.get_mut_ref(&user_position.symbol)?;
        let mut market_processor = MarketProcessor { market };
        let oracle = &trade_token_map.get_trade_token(&user_position.margin_mint)?.oracle;
        market_processor.update_market_funding_fee_rate(
            &ctx.accounts.state,
            oracle_map.get_price_data(oracle).unwrap().price,
        )?;
        drop(market_processor);
    }

    let (cross_net_value, total_position_mm, total_size) = user_processor
        .get_user_cross_net_value(
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
        for user_position in &mut user_processor.user.user_positions {
            let mut position_processor = PositionProcessor { position: user_position };
            //only cross margin position support
            if !position_processor.position.cross_margin {
                continue;
            }

            let market = market_map.get_ref(&position_processor.position.symbol)?;
            let index_trade_token =
                trade_token_map.get_trade_token(&position_processor.position.index_mint)?;

            let index_price = oracle_map.get_price_data(&index_trade_token.oracle).unwrap().price;
            let bankruptcy_price = cal_utils::format_to_ticker_size(
                if position_processor.position.is_long {
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
                position_processor.position.is_long,
            )?;

            validate!(bankruptcy_price > 0, BumpErrorCode::PriceIsNotAllowed)?;
            let liquidation_price = cal_utils::format_to_ticker_size(
                if position_processor.position.is_long {
                    cal_utils::div_rate_u(
                        bankruptcy_price,
                        RATE_PRECISION
                            .safe_sub(position_processor.get_position_mm(&market, state)?)?,
                    )?
                } else {
                    cal_utils::div_rate_u(
                        bankruptcy_price,
                        RATE_PRECISION
                            .safe_add(position_processor.get_position_mm(&market, state)?)?,
                    )?
                },
                market.market_trade_config.tick_size,
                position_processor.position.is_long,
            )?;

            let pool = pool_key_map.get_ref(&market.pool_key)?;
            let stable_pool = pool_key_map.get_ref(&market.stable_pool_key)?;
            let trade_token =
                trade_token_map.get_trade_token(&position_processor.position.margin_mint)?;

            position_processor.decrease_position(
                DecreasePositionParams {
                    order_id: 0,
                    is_liquidation: true,
                    is_cross_margin: true,
                    margin_token: position_processor.position.margin_mint,
                    decrease_size: position_processor.position.position_size,
                    execute_price: liquidation_price,
                },
                &ctx.accounts.user,
                pool_key_map.get_account_loader(&market.pool_key)?,
                pool_key_map.get_account_loader(&market.stable_pool_key)?,
                market_map.get_account_loader(&position_processor.position.symbol)?,
                &ctx.accounts.state,
                None,
                if position_processor.position.is_long {
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
                trade_token_map.get_account_loader(&position_processor.position.margin_mint)?,
                vault_map.get_account(&pda::generate_trade_token_vault_key(
                    trade_token.token_index,
                    ctx.program_id,
                )?)?,
                &ctx.accounts.bump_signer,
                &ctx.accounts.token_program,
                &ctx.program_id,
                &mut oracle_map,
            )?;
        }
    }
    Ok(())
}
