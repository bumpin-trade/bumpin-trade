use solana_program::pubkey::Pubkey;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::position_processor::PositionProcessor;
use crate::state::infrastructure::user_order::{OrderType, UserOrder};
use crate::state::infrastructure::user_position::UserPosition;
use crate::state::infrastructure::user_token::UserToken;
use crate::state::market_map::MarketMap;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::pool_map::PoolMap;
use crate::state::state::State;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;
use crate::validate;
use solana_program::msg;

pub struct UserProcessor<'a> {
    pub(crate) user: &'a mut User,
}

impl<'a> UserProcessor<'a> {
    pub fn sub_user_token_amount(&self, user_token: &mut UserToken, mut amount: u128) {
        user_token.sub_token_amount(amount)?;
        for mut user_position in self.user.user_positions {
            if user_position.cross_margin && user_position.margin_mint.eq(&user_token.token_mint) && amount > 0 {
                let reduce_amount = user_position.reduce_position_portfolio_balance(amount)?;
                amount = amount.safe_sub(reduce_amount)?;
            }
        }
    }
    pub fn get_user_cross_position_value(&self, state: &State, market_map: &MarketMap, pool_map: &PoolMap, price_map: &mut OracleMap) -> BumpResult<(u128, i128, i128, u128)> {
        let mut total_im_usd = 0u128;
        let mut total_un_pnl_usd = 0i128;
        let mut total_position_fee = 0i128;
        let mut total_position_mm = 0u128;


        for mut user_position in self.user.user_positions {
            let price_data = price_map.get_price_data(&user_position.index_mint)?;
            let market = market_map.get_ref(&user_position.symbol)?;
            let pool = pool_map.get_ref(&user_position.margin_mint)?;

            let position_processor = PositionProcessor { position: &mut user_position };

            total_un_pnl_usd = total_un_pnl_usd.safe_add(position_processor.get_position_un_pnl_usd(price_data.price)?)?;
            total_position_fee = total_position_fee.safe_add(position_processor.get_position_fee(&market, &pool, price_map)?)?;
            total_im_usd = total_im_usd.safe_add(user_position.initial_margin_usd)?;
            total_position_mm = total_position_mm.safe_add(position_processor.get_position_mm(&market, state)?)?;
            drop(position_processor);
        }
        Ok((total_im_usd, total_un_pnl_usd, total_position_fee, total_position_mm))
    }
    pub fn get_total_used_value(&self, trade_token_map: &TradeTokenMap, oracle_map: &mut OracleMap) -> BumpResult<u128> {
        let mut total_used_value = 0u128;
        for user_token in self.user.user_tokens {
            let trade_token = trade_token_map.get_trade_token(&user_token.token_mint)?;
            let oracle_price = oracle_map.get_price_data(&user_token.token_mint)?;
            total_used_value = total_used_value.safe_add(user_token.get_token_used_value(&trade_token, &oracle_price)?)?;
        }
        if self.user.hold > 0 {
            total_used_value = total_used_value.safe_add(self.user.hold)?;
        }
        Ok(total_used_value)
    }
    pub fn get_portfolio_net_value(&self, trade_token_map: &TradeTokenMap, oracle_map: &mut OracleMap) -> BumpResult<u128> {
        let mut total_token_net_value = 0u128;
        for user_token in self.user.user_tokens {
            let trade_token = trade_token_map.get_trade_token(&user_token.token_mint)?;
            let oracle_price = oracle_map.get_price_data(&user_token.token_mint)?;
            total_token_net_value.safe_add(user_token.get_token_net_value(&trade_token, oracle_price)?)?;

            drop(trade_token);
        }
        Ok(total_token_net_value)
    }
    pub fn get_available_value(&mut self, oracle_map: &mut OracleMap, trade_token_map: &mut TradeTokenMap) -> BumpResult<i128> {
        let mut total_net_value = 0u128;
        let mut total_used_value = 0u128;
        let mut total_borrowing_value = 0u128;

        let mut total_im_from_portfolio_value = 0u128;
        let mut total_un_pnl_value = 0i128;
        let mut total_mm_usd_value = 0u128;


        for user_token in self.user.user_tokens {
            let trade_token = trade_token_map.get_trade_token(&user_token.token_mint)?;
            let oracle_price_data = oracle_map.get_price_data(&user_token.token_mint)?;

            let token_net_value = user_token.get_token_net_value(&trade_token, &oracle_price_data)?;
            total_net_value = total_net_value.safe_add(token_net_value)?;

            let token_used_value = user_token.get_token_used_value(&trade_token, &oracle_price_data)?;
            total_used_value = total_used_value.safe_add(token_used_value)?;

            let token_borrowing_value = user_token.get_token_borrowing_value(&trade_token, &oracle_price_data);
            total_borrowing_value = total_borrowing_value.safe_add(token_borrowing_value?)?;
        }

        for mut user_position in self.user.user_positions {
            if user_position.cross_margin {
                let trade_token = trade_token_map.get_trade_token(&user_position.margin_mint)?;
                let oracle_price_data = oracle_map.get_price_data(&user_position.index_mint)?;
                total_im_from_portfolio_value = total_im_from_portfolio_value.
                    safe_add(user_position.initial_margin_usd_from_portfolio)?;

                let position_un_pnl = user_position.get_position_un_pnl(&trade_token, oracle_price_data.price, true)?;
                total_un_pnl_value = total_un_pnl_value.safe_add(position_un_pnl)?;

                total_mm_usd_value = total_mm_usd_value.safe_add(user_position.mm_usd)?;
            }
        }
        let available_value = total_net_value.
            safe_add(total_im_from_portfolio_value)?.
            safe_add(self.user.hold.cast()?)?.cast::<i128>()?.
            safe_sub(total_used_value.cast()?)?.
            safe_add(if total_un_pnl_value > 0 { 0i128 } else { total_un_pnl_value })?.
            safe_sub(total_im_from_portfolio_value.cast()?)?.
            safe_sub(total_borrowing_value.cast()?)?;
        Ok(available_value)
    }

    pub fn update_cross_position_balance(&mut self, mint: &Pubkey, amount: u128, add_amount: bool) {
        let mut reduce_amount = amount;
        for user_position in self.user.user_positions.iter_mut() {
            if user_position.cross_margin && user_position.margin_mint.eq(mint) && reduce_amount > 0 {
                if add_amount {
                    let change_amount = user_position.add_position_portfolio_balance(reduce_amount)?;
                    reduce_amount = reduce_amount.safe_sub(change_amount)?;
                } else {
                    let change_amount = user_position.reduce_position_portfolio_balance(reduce_amount)?;
                    reduce_amount = reduce_amount.safe_sub(change_amount)?;
                }
            }
        }
    }

    pub fn delete_position(&self, position_key: &Pubkey) -> BumpResult<> {
        let mut position_index = -1;
        for (index, position) in self.user.user_positions.iter().enumerate() {
            if position.position_key.eq(position_key) {
                position_index = index;
            }
        }
        if position_index == -1 {
            return Err(BumpErrorCode::AmountNotEnough);
        }
        self.user.user_positions[position_index] = UserPosition::default()?;
        Ok(())
    }

    pub fn cancel_stop_orders(&self, order_id: u128, symbol: [u8; 32], margin_token: &Pubkey, is_cross_margin: bool) -> BumpResult<()> {
        for user_order in self.user.user_orders {
            if user_order.order_id == order_id {
                continue;
            }
            if user_order.symbol == symbol && user_order.margin_token.eq(margin_token) && user_order.order_type.eq(&OrderType::STOP) && user_order.cross_margin == is_cross_margin {
                self.user.delete_order(user_order.order_id)?;
            }
        }
        Ok(())
    }

    pub fn cancel_all_orders(&self) -> BumpResult<()> {
        for (index, user_order) in self.user.user_orders.iter().enumerate() {
            if user_order.cross_margin {
                self.user.sub_order_hold_in_usd(user_order.order_margin);
            }
            self.user.user_orders[index] = UserOrder::default();
        }
        Ok(())
    }

    pub fn sub_token_with_liability(&mut self, token: &Pubkey, amount: u128) -> BumpResult<u128> {
        let mut liability = 0u128;
        validate!(self.user.user_tokens.map(|mint|mint.token_mint).contains(&token), BumpErrorCode::AmountNotEnough.into());
        let mut token_balance = self.user.user_tokens.iter_mut().find(|mint| mint.token_mint.eq(token))?;
        if token_balance.amount >= amount {
            token_balance.amount = token_balance.amount.safe_sub(amount)?;
        } else if token_balance.amount > 0u128 {
            liability = amount.safe_sub(token_balance.amount)?;
            token_balance.liability = token_balance.liability.safe_add(liability)?;
            token_balance.used_amount = token_balance.used_amount.safe_add(liability)?;
            token_balance.amount = 0u128;
        } else {
            token_balance.liability = token_balance.liability.safe_add(amount)?;
            token_balance.used_amount = token_balance.used_amount.safe_add(amount)?;
            liability = amount;
        }
        Ok(liability)
    }

    pub fn un_use_token(&mut self, token: &Pubkey, amount: u128) -> BumpResult<()> {
        validate!(self.user.user_tokens.map(|mint|mint.token_mint).contains(&token), BumpErrorCode::AmountNotEnough.into());
        let mut token_balance = self.user.user_tokens.iter_mut().find(|mint| mint.token_mint.eq(token))?;
        validate!(token_balance.used_amount >= amount, BumpErrorCode::AmountNotEnough.into());
        token_balance.used_amount = token_balance.used_amount.safe_sub(amount)?;
        Ok(())
    }

    pub fn add_token(&self, token: &Pubkey, amount: u128) {
        let user_token = self.user.get_user_token_mut(token)?;
        user_token.add_token_amount(amount);
    }
}

#[derive(Eq, Default, PartialEq, Debug)]
pub struct UpdateDecreasePositionResponse {}