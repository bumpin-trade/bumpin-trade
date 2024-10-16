use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::{calculator, LiquidateIsolatePositionParams};
use crate::math::casting::Cast;
use crate::math::constants::{RATE_PRECISION, SMALL_RATE_PRECISION};
use crate::math::safe_math::SafeMath;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::processor::position_processor;
use crate::processor::position_processor::DecreasePositionParams;
use crate::state::infrastructure::user_position::UserPosition;
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::user::User;
use crate::utils::pda;
use crate::validate;

#[derive(Accounts)]
#[instruction(
    _user_authority_key: Pubkey
)]
pub struct LiquidateCrossPosition<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_key,
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        mut,
        seeds = [b"user", _user_authority_key.as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        constraint = state.keeper_key.eq(& keeper_key.key())
    )]
    pub keeper_key: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_liquidate_cross_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, LiquidateCrossPosition<'c>>,
) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let state = &ctx.accounts.state;
    let remaining_accounts = ctx.remaining_accounts;

    let AccountMaps { market_map, trade_token_map, mut oracle_map, pool_map, vault_map, .. } =
        load_maps(remaining_accounts)?;

    user.cancel_all_cross_orders()?;

    let mut pos_infos: Vec<PosInfos> = Vec::new();
    for position in &user.positions {
        //only cross margin position support
        if !position.is_portfolio_margin {
            continue;
        }
        let infos = get_position_info(position)?;
        pos_infos.push(infos)
    }
    for pos_info in &pos_infos {
        let mut market = market_map.get_mut_ref(&pos_info.symbol)?;
        let mut pool = if pos_info.is_long {
            pool_map.get_mut_ref(&market.pool_key)?
        } else {
            pool_map.get_mut_ref(&market.stable_pool_key)?
        };
        pool.update_pool_borrowing_fee_rate()?;

        let trade_token = &trade_token_map.get_trade_token_by_mint_ref(&pos_info.margin_mint)?;
        market.update_market_funding_fee_rate(
            &ctx.accounts.state,
            oracle_map.get_price_data(&trade_token.oracle_key)?.price,
        )?;
        msg!(
            "========handle_liquidate_cross_position44444, market.longPerToken:{}",
            market.funding_fee.long_funding_fee_amount_per_size
        );
    }
    let (total_position_mm, total_position_fee) = {
        let mut total_mm = 0u128;
        let mut total_pos_fee = 0i128;
        for position in &user.positions {
            //only cross margin position support
            if !position.is_portfolio_margin {
                continue;
            }
            total_mm = total_mm.safe_add(position.mm_usd)?;
            let market = market_map.get_ref(&position.symbol)?;
            let pool = pool_map.get_ref(if position.is_long {
                &market.pool_key
            } else {
                &market.stable_pool_key
            })?;
            let trade_token =
                trade_token_map.get_trade_token_by_mint_ref(&position.margin_mint_key)?;
            let margin_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
            total_pos_fee = total_pos_fee.safe_add(position.get_position_fee(
                &market,
                &pool,
                margin_token_price,
                trade_token.decimals,
            )?)?;
        }
        (total_mm, total_pos_fee)
    };

    msg!(
        "========handle_liquidate_cross_position,total_mm:{}, total_pos_fee:{}",
        total_position_mm,
        total_position_fee
    );

    let (mut cross_net_value, total_size) =
        user.get_cross_net_value_and_pos_size(&trade_token_map, &mut oracle_map)?;
    msg!(
        "========handle_liquidate_cross_position,cross_net_value without posFee:{}",
        cross_net_value,
    );
    cross_net_value = cross_net_value.safe_sub(total_position_fee)?;
    let bankruptcy_mr = calculator::div_to_precision_i(
        cross_net_value,
        total_size.cast::<i128>()?,
        SMALL_RATE_PRECISION.cast::<i128>()?,
    )?;
    msg!(
        "========handle_liquidate_cross_position,cross_net_value:{}, total_position_mm:{}",
        cross_net_value,
        total_position_mm
    );
    validate!(
        cross_net_value <= 0i128 || cross_net_value.abs().cast::<u128>()? <= total_position_mm,
        BumpErrorCode::LiquidatePositionIgnore
    )?;
    for pos_info in &pos_infos {
        //only cross margin position support
        if !pos_info.is_portfolio_margin {
            continue;
        }

        let mut market = market_map.get_mut_ref(&pos_info.symbol)?;

        let index_price = oracle_map.get_price_data(&pos_info.index_mint)?.price;
        let bankruptcy_price = calculator::format_to_ticker_size(
            if pos_info.is_long {
                calculator::mul_small_rate_u(
                    index_price,
                    SMALL_RATE_PRECISION
                        .cast::<i128>()?
                        .safe_sub(bankruptcy_mr)?
                        .abs()
                        .cast::<u128>()?,
                )?
            } else {
                calculator::mul_small_rate_u(
                    index_price,
                    SMALL_RATE_PRECISION
                        .cast::<i128>()?
                        .safe_add(bankruptcy_mr)?
                        .abs()
                        .cast::<u128>()?,
                )?
            },
            market.config.tick_size,
            pos_info.is_long,
        )?;

        validate!(bankruptcy_price > 0, BumpErrorCode::LiquidationErrorWithBankruptcyPriceZero)?;
        let mm_rate = calculator::get_mm_rate(
            market.config.maximum_leverage,
            state.maximum_maintenance_margin_rate,
        )?;
        let liquidation_price = calculator::format_to_ticker_size(
            if pos_info.is_long {
                calculator::div_rate_u(bankruptcy_price, RATE_PRECISION.safe_sub(mm_rate)?)?
            } else {
                calculator::div_rate_u(bankruptcy_price, RATE_PRECISION.safe_add(mm_rate)?)?
            },
            market.config.tick_size,
            pos_info.is_long,
        )?;

        let pool = pool_map.get_ref(&market.pool_key)?;
        let stable_pool = pool_map.get_ref(&market.stable_pool_key)?;
        let trade_token = trade_token_map.get_trade_token_by_mint_ref(&pos_info.margin_mint)?;

        let pool_key = market.pool_key;
        let stable_pool_key = market.stable_pool_key;
        position_processor::decrease_position(
            DecreasePositionParams {
                order_id: 0,
                is_liquidation: true,
                is_portfolio_margin: true,
                margin_token: pos_info.margin_mint,
                decrease_size: pos_info.position_size,
                execute_price: liquidation_price,
            },
            &mut user,
            market.deref_mut(),
            pool_map.get_mut_ref(&pool_key)?.deref_mut(),
            pool_map.get_mut_ref(&stable_pool_key)?.deref_mut(),
            &ctx.accounts.state,
            None,
            if pos_info.is_long {
                vault_map.get_account(&pda::generate_pool_vault_key(pool.index, ctx.program_id)?)?
            } else {
                vault_map.get_account(&pda::generate_pool_vault_key(
                    stable_pool.index,
                    ctx.program_id,
                )?)?
            },
            trade_token_map.get_trade_token_by_mint_ref_mut(&pos_info.margin_mint)?.deref_mut(),
            vault_map.get_account(&pda::generate_trade_token_vault_key(
                trade_token.index,
                ctx.program_id,
            )?)?,
            &ctx.accounts.bump_signer,
            &ctx.accounts.token_program,
            &mut oracle_map,
            &pos_info.user_key,
        )?;
    }
    Ok(())
}

fn get_position_info(position: &UserPosition) -> BumpResult<PosInfos> {
    Ok(PosInfos {
        is_portfolio_margin: position.is_portfolio_margin,
        symbol: position.symbol,
        index_mint: position.index_mint_oracle,
        is_long: position.is_long,
        margin_mint: position.margin_mint_key,
        position_size: position.position_size,
        user_key: position.user_key,
    })
}

struct PosInfos {
    pub is_portfolio_margin: bool,
    pub symbol: [u8; 32],
    pub index_mint: Pubkey,
    pub is_long: bool,
    pub margin_mint: Pubkey,
    pub position_size: u128,
    pub user_key: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    params: LiquidateIsolatePositionParams
)]
pub struct LiquidateIsolatePosition<'info> {
    #[account(
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_key,
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        mut,
        seeds = [b"user", params.user_authority_key.key().as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        constraint = user_token_account.owner.eq(& user.load() ?.authority)
        && (pool_vault.mint.eq(& user_token_account.mint) || stable_pool_vault.mint.eq(& user_token_account.mint)),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"market", params.market_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [b"pool", params.pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = pool.load() ?.mint_key.eq(& market.load() ?.pool_mint_key)
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool", params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
        constraint = stable_pool.load() ?.mint_key.eq(& market.load() ?.stable_pool_mint_key)
    )]
    pub stable_pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), params.pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"pool_vault".as_ref(), params.stable_pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub stable_pool_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"trade_token", params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"trade_token_vault".as_ref(), params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = state.keeper_key.eq(& keeper_key.key())
    )]
    pub keeper_key: Signer<'info>,

    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_liquidate_isolate_position<'a, 'b, 'c: 'info, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, LiquidateIsolatePosition>,
    params: LiquidateIsolatePositionParams,
) -> Result<()> {
    let mut user = ctx.accounts.user.load_mut()?;
    let remaining_accounts = ctx.remaining_accounts;
    let mut market = ctx.accounts.market.load_mut()?;
    let mut trade_token = ctx.accounts.trade_token.load_mut()?;
    let mut oracle_map = OracleMap::load(remaining_accounts)?;
    let mut base_token_pool = ctx.accounts.pool.load_mut()?;
    let mut stable_pool = ctx.accounts.stable_pool.load_mut()?;
    let position_key = params.position_key;
    let (is_long, margin_mint, position_size, liquidation_price) = cal_liquidation_price(
        &position_key,
        &user,
        base_token_pool.deref_mut(),
        stable_pool.deref_mut(),
        &ctx.accounts.state,
        &trade_token,
        market.deref_mut(),
        &mut oracle_map,
    )?;
    msg!("===========handle_liquidate_isolate_position, liquidation_price:{}", liquidation_price);
    let position = user.get_user_position_ref(&position_key)?;
    validate!(!position.is_portfolio_margin, BumpErrorCode::OnlyIsolatePositionAllowed)?;

    let index_price = oracle_map.get_price_data(&position.index_mint_oracle)?;
    msg!("===========handle_liquidate_isolate_position, index_price:{}", index_price.price);
    if liquidation_price == 0u128
        || index_price.price == 0u128
        || (is_long && index_price.price > liquidation_price)
        || (!is_long && index_price.price < liquidation_price)
    {
        Err(BumpErrorCode::LiquidatePositionIgnore)?;
    }

    position_processor::decrease_position(
        DecreasePositionParams {
            order_id: 0,
            is_liquidation: true,
            is_portfolio_margin: false,
            margin_token: margin_mint,
            decrease_size: position_size,
            execute_price: liquidation_price,
        },
        &mut user,
        &mut market,
        &mut base_token_pool,
        &mut stable_pool,
        &ctx.accounts.state,
        Some(&ctx.accounts.user_token_account),
        if is_long { &ctx.accounts.pool_vault } else { &ctx.accounts.stable_pool_vault },
        trade_token.deref_mut(),
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.bump_signer,
        &ctx.accounts.token_program,
        &mut oracle_map,
        &position_key,
    )?;
    Ok(())
}

fn cal_liquidation_price(
    position_key: &Pubkey,
    user: &User,
    base_token_pool: &mut Pool,
    stable_pool: &mut Pool,
    state: &State,
    trade_token: &TradeToken,
    market: &mut Market,
    oracle_map: &mut OracleMap,
) -> BumpResult<(bool, Pubkey, u128, u128)> {
    let user_position = user.get_user_position_ref(position_key)?;
    let pool = if user_position.is_long { base_token_pool } else { stable_pool };

    validate!(!user_position.is_portfolio_margin, BumpErrorCode::OnlyIsolatePositionAllowed)?;
    market.update_market_funding_fee_rate(
        state,
        oracle_map.get_price_data(&trade_token.oracle_key)?.price,
    )?;

    pool.update_pool_borrowing_fee_rate()?;

    let margin_token_price = oracle_map.get_price_data(&trade_token.oracle_key)?.price;
    let liquidation_price = user_position.get_liquidation_price(
        market,
        pool,
        margin_token_price,
        trade_token.decimals,
    )?;
    Ok((
        user_position.is_long,
        user_position.margin_mint_key,
        user_position.position_size,
        liquidation_price,
    ))
}
