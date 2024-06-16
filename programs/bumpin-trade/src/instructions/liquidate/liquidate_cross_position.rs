use std::iter::Peekable;
use std::slice::Iter;

use crate::errors::BumpErrorCode;
use crate::instructions::cal_utils;
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::math::casting::Cast;
use crate::math::constants::{RATE_PRECISION, SMALL_RATE_PRECISION};
use crate::math::safe_math::SafeMath;
use crate::processor::market_processor::MarketProcessor;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::pool_processor::PoolProcessor;
use crate::processor::position_processor;
use crate::processor::position_processor::{DecreasePositionParams, PositionProcessor};
use crate::processor::user_processor::UserProcessor;
use crate::state::state::State;
use crate::state::user::User;
use crate::validate;

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct LiquidateCrossPosition<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    pub user: AccountLoader<'info, User>,

    pub keeper_signer: Signer<'info>,
}

pub fn handle_liquidate_cross_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, LiquidateCrossPosition>,
) -> Result<()> {
    Ok(())
    // let user = &mut ctx.accounts.user.load_mut()?;
    // let state = &ctx.accounts.state;
    //
    // let remaining_accounts: &mut Peekable<Iter<'info, AccountInfo<'info>>> =
    //     &mut ctx.remaining_accounts.iter().peekable();
    //
    // let AccountMaps { market_map, trade_token_map, mut oracle_map, pool_mint_map, .. } =
    //     load_maps(remaining_accounts)?;
    //
    // let mut user_processor = UserProcessor { user };
    // user_processor.cancel_all_cross_orders()?;
    //
    // for user_position in user_processor.user.user_positions {
    //     //only cross margin position support
    //     if !user_position.cross_margin {
    //         continue;
    //     }
    //
    //     let pool = &mut pool_mint_map.get_mut_ref(&user_position.margin_mint)?;
    //     let mut pool_processor = PoolProcessor { pool };
    //     pool_processor.update_pool_borrowing_fee_rate()?;
    //     drop(pool_processor);
    //
    //     let market = &mut market_map.get_mut_ref(&user_position.symbol)?;
    //     let mut market_processor = MarketProcessor { market };
    //     market_processor.update_market_funding_fee_rate(&ctx.accounts.state, &mut oracle_map)?;
    //     drop(market_processor);
    // }
    //
    // let portfolio_net_value =
    //     user_processor.get_portfolio_net_value(&trade_token_map, &mut oracle_map)?;
    // let used_value = user_processor.get_total_used_value(&trade_token_map, &mut oracle_map)?;
    // let (total_im_usd, total_un_pnl_usd, total_position_fee, total_position_mm, total_size) =
    //     user_processor.get_user_cross_position_value(
    //         &ctx.accounts.state,
    //         &market_map,
    //         &pool_mint_map,
    //         &mut oracle_map,
    //     )?;
    // let cross_net_value = portfolio_net_value
    //     .safe_add(total_im_usd)?
    //     .safe_add(user.hold)?
    //     .cast::<i128>()?
    //     .safe_add(total_un_pnl_usd)?
    //     .safe_sub(used_value.cast()?)?
    //     .safe_sub(total_position_fee)?;
    //
    // let bankruptcy_mr = cal_utils::div_to_precision_i(
    //     cross_net_value,
    //     total_size.cast::<i128>()?,
    //     SMALL_RATE_PRECISION.cast::<i128>()?,
    // )?
    // .max(0i128);
    //
    // if cross_net_value <= 0 || cross_net_value.abs().cast::<u128>()? <= total_position_mm {
    //     for mut user_position in user_processor.user.user_positions {
    //         let mut position_processor = PositionProcessor { position: &mut user_position };
    //         //only cross margin position support
    //         if !position_processor.position.cross_margin {
    //             continue;
    //         }
    //
    //         let market = market_map.get_ref(&position_processor.position.symbol)?;
    //
    //         let index_price = oracle_map.get_price_data(&position_processor.position.index_mint).unwrap().price;
    //         let bankruptcy_price = cal_utils::format_to_ticker_size(
    //             if position_processor.position.is_long {
    //                 cal_utils::mul_small_rate_u(
    //                     index_price,
    //                     SMALL_RATE_PRECISION
    //                         .cast::<i128>()?
    //                         .safe_sub(bankruptcy_mr)?
    //                         .abs()
    //                         .cast::<u128>()?,
    //                 )?
    //             } else {
    //                 cal_utils::mul_small_rate_u(
    //                     index_price,
    //                     SMALL_RATE_PRECISION
    //                         .cast::<i128>()?
    //                         .safe_add(bankruptcy_mr)?
    //                         .abs()
    //                         .cast::<u128>()?,
    //                 )?
    //             },
    //             market.market_trade_config.tick_size,
    //             position_processor.position.is_long,
    //         )?;
    //
    //         validate!(bankruptcy_price > 0, BumpErrorCode::PriceIsNotAllowed)?;
    //         let liquidation_price = cal_utils::format_to_ticker_size(
    //             if position_processor.position.is_long {
    //                 cal_utils::div_rate_u(
    //                     bankruptcy_price,
    //                     RATE_PRECISION
    //                         .safe_sub(position_processor.get_position_mm(&market, state)?)?,
    //                 )?
    //             } else {
    //                 cal_utils::div_rate_u(
    //                     bankruptcy_price,
    //                     RATE_PRECISION
    //                         .safe_add(position_processor.get_position_mm(&market, state)?)?,
    //                 )?
    //             },
    //             market.market_trade_config.tick_size,
    //             position_processor.position.is_long,
    //         )?;
    //
    //
    //
    //         position_processor.decrease_position(DecreasePositionParams{
    //             order_id: 0,
    //             is_liquidation: false,
    //             is_cross_margin: false,
    //             margin_token: Default::default(),
    //             decrease_size: 0,
    //             execute_price: 0,
    //         }, &ctx.accounts.user, pool_mint_map.get_account_loader(&position_processor.position.margin_mint)?,
    //                                              pool_mint_map.get_account_loader(&market.stable_pool_mint)?,
    //                                              market_map.get_account_loader(&position_processor.position.symbol)?,
    //                                              &ctx.accounts.state,,,trade_token_map.get_account_loader(&position_processor.position.margin_mint)?,)?;
    //     }
    // }
    // Ok(())
}
