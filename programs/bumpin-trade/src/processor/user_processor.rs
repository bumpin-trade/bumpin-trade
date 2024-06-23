use anchor_lang::prelude::{Account, Program};
use anchor_spl::token::{Token, TokenAccount};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode::CouldNotFindUserToken;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::position_processor::PositionProcessor;
use crate::state::infrastructure::user_order::{OrderStatus, OrderType, PositionSide, UserOrder};
use crate::state::infrastructure::user_position::PositionStatus;
use crate::state::infrastructure::user_token::UserTokenStatus;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool_map::PoolMap;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::{User, UserTokenUpdateOrigin};
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
        token_mint: &Pubkey,
        oracle_map: &mut OracleMap,
        trade_token_map: &TradeTokenMap,
    ) -> BumpResult {
        let price_data = oracle_map.get_price_data(oracle)?;
        let withdraw_usd = price_data.price.safe_mul(amount)?;

        let available_value = self.get_available_value(oracle_map, trade_token_map)?;
        validate!(
            available_value.abs().cast::<u128>()? > withdraw_usd,
            BumpErrorCode::UserNotEnoughValue
        )?;
        self.user.sub_user_token_amount_ignore_used_amount(
            token_mint,
            amount,
            &UserTokenUpdateOrigin::WITHDRAW,
        )?;
        self.update_cross_position_balance(token_mint, amount, false)?;
        Ok(())
    }

    pub fn get_user_cross_position_value(
        &mut self,
        state: &State,
        market_map: &MarketMap,
        pool_map: &PoolMap,
        trade_token_map: &TradeTokenMap,
        price_map: &mut OracleMap,
    ) -> BumpResult<(u128, i128, i128, u128, u128)> {
        let mut total_im_usd = 0u128;
        let mut total_un_pnl_usd = 0i128;
        let mut total_position_fee = 0i128;
        let mut total_position_mm = 0u128;
        let mut total_size = 0u128;

        for mut user_position in &mut self.user.user_positions {
            if user_position.status.eq(&PositionStatus::INIT) {
                continue;
            }
            let index_trade_token = trade_token_map.get_trade_token(&user_position.index_mint)?;
            let trade_token = trade_token_map.get_trade_token(&user_position.margin_mint)?;
            let index_price = price_map.get_price_data(&index_trade_token.oracle)?.price;
            let margin_token_price = price_map.get_price_data(&trade_token.oracle)?.price;
            let market = market_map.get_ref(&user_position.symbol)?;
            let pool = pool_map.get_ref(&market.pool_key)?;
            total_im_usd = total_im_usd.safe_add(user_position.initial_margin_usd)?;

            let position_processor = PositionProcessor { position: &mut user_position };
            {
                total_un_pnl_usd = total_un_pnl_usd
                    .safe_add(position_processor.get_position_un_pnl_usd(index_price)?)?;
                total_position_fee =
                    total_position_fee.safe_add(position_processor.get_position_fee(
                        &market,
                        &pool,
                        margin_token_price,
                        trade_token.decimals,
                    )?)?;
                total_position_mm = total_position_mm
                    .safe_add(position_processor.get_position_mm(&market, state)?)?;
                total_size = total_size.safe_add(position_processor.position.position_size)?;
            }
            drop(position_processor);
        }
        Ok((total_im_usd, total_un_pnl_usd, total_position_fee, total_position_mm, total_size))
    }

    pub fn get_total_used_value(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
    ) -> BumpResult<u128> {
        let mut total_used_value = 0u128;
        for user_token in &self.user.user_tokens {
            if user_token.user_token_status.eq(&UserTokenStatus::INIT) {
                continue;
            }
            let trade_token = trade_token_map.get_trade_token(&user_token.token_mint)?;
            let oracle_price = oracle_map.get_price_data(&trade_token.oracle)?;
            total_used_value = total_used_value
                .safe_add(user_token.get_token_used_value(&trade_token, &oracle_price)?)?;
        }
        if self.user.hold > 0 {
            total_used_value = total_used_value.safe_add(self.user.hold)?;
        }
        Ok(total_used_value)
    }

    pub fn get_portfolio_net_value(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
    ) -> BumpResult<u128> {
        let total_token_net_value = 0u128;
        for user_token in &self.user.user_tokens {
            if user_token.user_token_status.eq(&UserTokenStatus::INIT) {
                continue;
            }
            let trade_token = trade_token_map.get_trade_token(&user_token.token_mint)?;
            let oracle_price = oracle_map.get_price_data(&trade_token.oracle)?;
            total_token_net_value
                .safe_add(user_token.get_token_net_value(&trade_token, oracle_price)?)?;

            drop(trade_token);
        }
        Ok(total_token_net_value)
    }

    pub fn get_available_value(
        &mut self,
        oracle_map: &mut OracleMap,
        trade_token_map: &TradeTokenMap,
    ) -> BumpResult<i128> {
        let mut total_net_value = 0u128;
        let mut total_used_value = 0u128;
        let mut total_borrowing_value = 0u128;

        let mut total_im_from_portfolio_value = 0u128;
        let mut total_un_pnl_value = 0i128;
        let mut total_mm_usd_value = 0u128;

        for user_token in &self.user.user_tokens {
            if user_token.user_token_status.eq(&UserTokenStatus::INIT) {
                continue;
            }
            let trade_token = trade_token_map.get_trade_token(&user_token.token_mint)?;
            let oracle_price_data = oracle_map.get_price_data(&trade_token.oracle)?;

            let token_net_value =
                user_token.get_token_net_value(&trade_token, &oracle_price_data)?;
            total_net_value = total_net_value.safe_add(token_net_value)?;

            let token_used_value =
                user_token.get_token_used_value(&trade_token, &oracle_price_data)?;
            total_used_value = total_used_value.safe_add(token_used_value)?;

            let token_borrowing_value = user_token.get_token_borrowing_value(&oracle_price_data)?;
            total_borrowing_value = total_borrowing_value.safe_add(token_borrowing_value)?;
        }

        let positions_count = self.user.user_positions.len();

        for i in 0..positions_count {
            let user_position = &mut self.user.user_positions[i];
            if user_position.status.eq(&PositionStatus::INIT) {
                continue;
            }

            let position_processor = PositionProcessor { position: user_position };
            let index_trade_token =
                trade_token_map.get_trade_token(&position_processor.position.index_mint)?;
            let (initial_margin_usd_from_portfolio, position_un_pnl, mm_usd) =
                position_processor.get_position_value(&index_trade_token, oracle_map)?;

            total_im_from_portfolio_value =
                total_im_from_portfolio_value.safe_add(initial_margin_usd_from_portfolio)?;
            total_un_pnl_value = total_un_pnl_value.safe_add(position_un_pnl)?;
            total_mm_usd_value = total_mm_usd_value.safe_add(mm_usd)?;

            drop(position_processor);
        }
        let available_value = total_net_value
            .safe_add(total_im_from_portfolio_value)?
            .safe_add(self.user.hold.cast()?)?
            .cast::<i128>()?
            .safe_sub(total_used_value.cast()?)?
            .safe_add(if total_un_pnl_value > 0 { 0i128 } else { total_un_pnl_value })?
            .safe_sub(total_im_from_portfolio_value.cast()?)?
            .safe_sub(total_borrowing_value.cast()?)?;
        Ok(available_value)
    }

    pub fn get_user_cross_net_value(
        &mut self,
        trade_token_map: &TradeTokenMap,
        mut oracle_map: &mut OracleMap,
        market_map: &MarketMap,
        pool_key_map: &PoolMap,
        state: &State,
    ) -> BumpResult<(i128, u128, u128)> {
        let portfolio_net_value =
            self.get_portfolio_net_value(&trade_token_map, &mut oracle_map)?;
        let used_value = self.get_total_used_value(&trade_token_map, &mut oracle_map)?;
        let (total_im_usd, total_un_pnl_usd, total_position_fee, total_position_mm, total_size) =
            self.get_user_cross_position_value(
                state,
                &market_map,
                &pool_key_map,
                &trade_token_map,
                &mut oracle_map,
            )?;

        let cross_net_value = portfolio_net_value
            .safe_add(total_im_usd)?
            .safe_add(self.user.hold)?
            .cast::<i128>()?
            .safe_add(total_un_pnl_usd)?
            .safe_sub(used_value.cast()?)?
            .safe_sub(total_position_fee)?;
        Ok((cross_net_value, total_position_mm, total_size))
    }

    pub fn update_cross_position_balance(
        &mut self,
        mint: &Pubkey,
        amount: u128,
        add_amount: bool,
    ) -> BumpResult<()> {
        let mut reduce_amount = amount;
        for user_position in self.user.user_positions.iter_mut() {
            if user_position.status.eq(&PositionStatus::INIT) {
                continue;
            }
            if user_position.cross_margin && user_position.margin_mint.eq(mint) && reduce_amount > 0
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
        order_id: u128,
        symbol: [u8; 32],
        margin_token: &Pubkey,
        is_cross_margin: bool,
    ) -> BumpResult<()> {
        for user_order in self.user.user_orders {
            if user_order.status.eq(&OrderStatus::INIT) {
                continue;
            }
            if user_order.order_id == order_id {
                continue;
            }
            if user_order.symbol == symbol
                && user_order.margin_mint.eq(margin_token)
                && user_order.order_type.eq(&OrderType::STOP)
                && user_order.cross_margin == is_cross_margin
            {
                self.user.delete_order(user_order.order_id)?;
            }
        }
        Ok(())
    }

    pub fn cancel_all_cross_orders(&mut self) -> BumpResult<()> {
        let user_orders_length = self.user.user_orders.len();
        for index in 0..user_orders_length {
            let order = self.user.user_orders[index];
            if order.status.eq(&OrderStatus::USING) && order.cross_margin {
                self.user.cancel_user_order(index)?;
            }
        }
        Ok(())
    }

    pub fn sub_token_with_liability(
        &mut self,
        token: &Pubkey,
        trade_token: &mut TradeToken,
        amount: u128,
    ) -> BumpResult<u128> {
        let mut liability = 0u128;
        let token_balance = self
            .user
            .user_tokens
            .iter_mut()
            .find(|mint| mint.token_mint.eq(token))
            .ok_or(CouldNotFindUserToken)?;
        if token_balance.amount >= amount {
            token_balance.amount = token_balance.amount.safe_sub(amount)?;
        } else if token_balance.amount > 0u128 {
            liability = amount.safe_sub(token_balance.amount)?;
            token_balance.liability = token_balance.liability.safe_add(liability)?;
            token_balance.used_amount = token_balance.used_amount.safe_add(liability)?;
            token_balance.amount = 0u128;
            trade_token.add_liability(liability)?;
        } else {
            token_balance.liability = token_balance.liability.safe_add(amount)?;
            token_balance.used_amount = token_balance.used_amount.safe_add(amount)?;
            liability = amount;
            trade_token.add_liability(amount)?;
        }
        Ok(liability)
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
        if order.position_side.eq(&PositionSide::INCREASE) && order.cross_margin {
            self.user.sub_order_hold_in_usd(order.order_margin)?;
        } else if order.position_side.eq(&PositionSide::INCREASE) && !order.cross_margin {
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
