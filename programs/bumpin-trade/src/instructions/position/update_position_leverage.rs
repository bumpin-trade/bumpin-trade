use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::instructions::{cal_utils, UpdatePositionMarginParams};
use crate::math::safe_math::SafeMath;
use crate::processor::position_processor::PositionProcessor;
use crate::processor::user_processor::UserProcessor;
use crate::state::market::Market;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::utils::token;
use crate::{can_sign_for_user, validate};
use crate::errors::BumpErrorCode;
use crate::math::casting::Cast;

#[derive(Accounts)]
pub struct UpdatePositionLeverage<'info> {
    #[account(
        mut,
        constraint = can_sign_for_user(& user_account, & authority) ?
    )]
    pub user_account: AccountLoader<'info, User>,
    pub authority: Signer<'info>,
    pub trade_token: AccountLoader<'info, TradeToken>,
    pub pool: AccountLoader<'info, Pool>,
    pub state: Account<'info, State>,
    pub market: AccountLoader<'info, Market>,
    #[account(
        mut,
        constraint = & pool_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub pool_vault: Account<'info, TokenAccount>,
    pub bump_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UpdatePositionLeverageParams {
    pub symbol: [u8; 32],
    pub is_long: bool,
    pub is_cross_margin: bool,
    pub leverage: u128,
    pub add_margin_amount: u128,
}

pub fn handle_update_position_leverage(ctx: Context<UpdatePositionLeverage>, params: UpdatePositionLeverageParams) -> anchor_lang::Result<()> {
    let mut user = ctx.accounts.user_account.load_mut()?;
    let trade_token = ctx.accounts.trade_token.load_mut()?;
    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();
    let mut oracle_map = OracleMap::load(remaining_accounts_iter)?;
    let mut trade_token_map = TradeTokenMap::load(remaining_accounts_iter)?;
    let mut pool = ctx.accounts.pool.load_mut()?;
    let market = ctx.accounts.market.load_mut()?;
    validate!(params.leverage <= market.market_trade_config.max_leverage, BumpErrorCode::AmountNotEnough.into());

    let position_key = user.generate_position_key(&user.authority, params.symbol, &trade_token.mint, params.is_cross_margin, &ctx.program_id)?;
    let mut position = user.find_position_mut_by_key(&position_key)?;
    validate!(position.leverage != params.leverage, BumpErrorCode::AmountNotEnough.into());
    let mut position_processor = PositionProcessor { position: &mut position };

    let token_price = oracle_map.get_price_data(&trade_token.mint)?.price;


    if position.position_size != 0u128 {
        if position.leverage > params.leverage {
            let mut add_margin_amount = 0u128;
            let mut add_initial_margin_from_portfolio = 0u128;
            if position.cross_margin {
                position.set_leverage(params.leverage)?;
                let new_initial_margin_in_usd = cal_utils::div_rate_u(position.position_size, position.leverage)?;
                let add_margin_in_usd = if new_initial_margin_in_usd > position.initial_margin_usd { new_initial_margin_in_usd.safe_sub(position.initial_margin_usd)? } else { 0u128 };
                let mut user_processor = UserProcessor { user: &mut user };
                let cross_available_value = user_processor.get_available_value(&mut oracle_map, &mut trade_token_map)?;
                validate!(add_margin_in_usd.cast::<i128>()? < cross_available_value, BumpErrorCode::AmountNotEnough.into());

                add_margin_amount = cal_utils::usd_to_token_u(add_margin_in_usd, trade_token.decimals, token_price)?;
                let available_amount = user.get_user_token_ref(&trade_token.mint)?.get_token_available_amount()?;
                add_initial_margin_from_portfolio = cal_utils::token_to_usd_u(add_margin_amount.min(available_amount), trade_token.decimals, token_price)?;
                user.use_token(&trade_token.mint, add_margin_amount, false)?;
            } else {
                add_margin_amount = params.add_margin_amount;
            }
            position_processor.execute_add_position_margin(&UpdatePositionMarginParams {
                position_key,
                is_add: true,
                update_margin_amount: add_margin_amount,
            }, &trade_token, &mut oracle_map, &mut pool)?;
            if !params.is_cross_margin {
                token::receive(
                    &ctx.accounts.token_program,
                    &ctx.accounts.user_token_account,
                    &ctx.accounts.pool_vault,
                    &ctx.accounts.authority,
                    params.add_margin_amount,
                )?;
            }
        } else {
            position.set_leverage(params.leverage)?;
            let reduce_margin = position.initial_margin_usd.safe_sub(cal_utils::div_rate_u(position.position_size, position.leverage)?)?;
            let reduce_margin_amount = position_processor.execute_reduce_position_margin(&UpdatePositionMarginParams {
                position_key,
                is_add: false,
                update_margin_amount: reduce_margin,
            }, false, &trade_token, &mut oracle_map, &mut pool, &market, &ctx.accounts.state)?;
            if position.cross_margin {
                user.un_use_token(&position.margin_mint, reduce_margin_amount)?;
            } else {
                token::send_from_program_vault(
                    &ctx.accounts.token_program,
                    &ctx.accounts.pool_vault,
                    &ctx.accounts.user_token_account,
                    &ctx.accounts.bump_signer,
                    ctx.accounts.state.bump_signer_nonce,
                    reduce_margin_amount,
                )?;
            }
        }
    }
    Ok(())
}