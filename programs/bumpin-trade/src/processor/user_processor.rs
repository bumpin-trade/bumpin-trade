use anchor_lang::prelude::*;
use anchor_lang::prelude::{Account, Program};
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::infrastructure::user_order::{OrderStatus, OrderType, PositionSide, UserOrder};
use crate::state::infrastructure::user_position::PositionStatus;
use crate::state::oracle_map::OracleMap;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::{User, UserTokenUpdateReason};
use crate::utils::token;
use crate::validate;

pub struct UserProcessor<'a> {
    pub(crate) user: &'a mut User,
}

impl<'a> UserProcessor<'a> {
    pub fn withdraw(
        &mut self,
        amount: u128,
        oracle: &Pubkey,
        trade_token: &TradeToken,
        token_mint: &Pubkey,
        oracle_map: &mut OracleMap,
        trade_token_map: &TradeTokenMap,
    ) -> BumpResult {
        let price = oracle_map.get_price_data(oracle)?.price;
        let withdraw_usd = cal_utils::token_to_usd_u(amount, trade_token.decimals, price)?;

        let available_value = self.user.get_available_value(oracle_map, trade_token_map)?;
        validate!(
            available_value.abs().cast::<u128>()? > withdraw_usd,
            BumpErrorCode::UserNotEnoughValue
        )?;
        self.user.sub_user_token_amount_ignore_used_amount(
            token_mint,
            amount,
            &UserTokenUpdateReason::WITHDRAW,
        )?;
        self.update_cross_position_balance(token_mint, amount, false)?;
        Ok(())
    }

    pub fn update_cross_position_balance(
        &mut self,
        mint: &Pubkey,
        amount: u128,
        add_amount: bool,
    ) -> BumpResult<()> {
        let mut reduce_amount = amount;
        for user_position in self.user.positions.iter_mut() {
            if user_position.status.eq(&PositionStatus::INIT) {
                continue;
            }
            if user_position.is_portfolio_margin
                && user_position.margin_mint_key.eq(mint)
                && reduce_amount > 0
            {
                if add_amount {
                    let change_amount =
                        user_position.add_position_portfolio_balance(reduce_amount)?;
                    reduce_amount = reduce_amount.safe_sub(change_amount)?;
                } else {
                    let change_amount =
                        user_position.reduce_position_portfolio_balance(reduce_amount)?;
                    reduce_amount = reduce_amount.safe_sub(change_amount)?;
                }
            }

            if reduce_amount == 0u128 {
                break;
            }
        }
        Ok(())
    }

    pub fn cancel_stop_orders(
        &mut self,
        order_id: u64,
        symbol: [u8; 32],
        margin_token: &Pubkey,
        is_portfolio_margin: bool,
    ) -> BumpResult<()> {
        for user_order in self.user.orders {
            if user_order.status.eq(&OrderStatus::INIT) {
                continue;
            }
            if user_order.order_id == order_id {
                continue;
            }
            if user_order.symbol == symbol
                && user_order.margin_mint_key.eq(margin_token)
                && user_order.order_type.eq(&OrderType::STOP)
                && user_order.is_portfolio_margin == is_portfolio_margin
            {
                self.user.delete_order(user_order.order_id)?;
            }
        }
        Ok(())
    }

    pub fn cancel_all_cross_orders(&mut self) -> BumpResult<()> {
        let user_orders_length = self.user.orders.len();
        for index in 0..user_orders_length {
            let order = self.user.orders[index];
            if order.status.eq(&OrderStatus::USING) && order.is_portfolio_margin {
                self.user.cancel_user_order(index)?;
            }
        }
        Ok(())
    }

    pub fn cancel_order<'info>(
        &mut self,
        order: &UserOrder,
        token_program: &Program<'info, Token>,
        pool_vault: &Account<'info, TokenAccount>,
        user_token_account: &Account<'info, TokenAccount>,
        bump_signer: &AccountInfo<'info>,
        state: &Account<'info, State>,
    ) -> BumpResult<()> {
        self.user.delete_order(order.order_id)?;
        if order.position_side.eq(&PositionSide::INCREASE) && order.is_portfolio_margin {
            self.user.sub_order_hold_in_usd(order.order_margin)?;
        } else if order.position_side.eq(&PositionSide::INCREASE) && !order.is_portfolio_margin {
            token::send_from_program_vault(
                token_program,
                pool_vault,
                user_token_account,
                bump_signer,
                state.bump_signer_nonce,
                order.order_margin,
            )
            .map_err(|_e| BumpErrorCode::TransferFailed)?;
        }
        Ok(())
    }
}
