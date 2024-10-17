use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::errors::BumpErrorCode::{
    CouldNotFindUserOrder, CouldNotFindUserPosition, CouldNotFindUserStake, CouldNotFindUserToken,
};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::calculator;
use crate::math::casting::Cast;
use crate::math::constants::RATE_PRECISION;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::{
    AddOrDeleteUserOrderEvent, UserHoldUpdateEvent, UserTokenBalanceUpdateEvent,
};
use crate::state::infrastructure::user_order::{
    OrderSide, OrderStatus, OrderType, PositionSide, UserOrder,
};
use crate::state::infrastructure::user_position::{PositionStatus, UserPosition};
use crate::state::infrastructure::user_stake::{UserStake, UserStakeStatus};
use crate::state::infrastructure::user_token::{UserToken, UserTokenStatus};
use crate::state::oracle_map::OracleMap;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::traits::Size;
use crate::utils::{pda, token};
use crate::validate;

#[account(zero_copy(unsafe))]
#[derive(Eq, PartialEq, Debug)]
#[repr(C)]
pub struct User {
    pub key: Pubkey,
    pub next_order_id: u64,
    pub next_liquidation_id: u64,
    pub hold: u128,
    pub tokens: [UserToken; 10],
    pub stakes: [UserStake; 10],
    pub positions: [UserPosition; 8],
    pub orders: [UserOrder; 8],
    pub authority: Pubkey,
    pub created_at: i64,
    pub user_status: UserStatus,
    pub padding: [u8; 7],
}

impl Default for User {
    fn default() -> Self {
        Self {
            next_order_id: 0,
            next_liquidation_id: 0,
            hold: 0,
            tokens: [UserToken::default(); 10],
            stakes: [UserStake::default(); 10],
            positions: [UserPosition::default(); 8],
            orders: [UserOrder::default(); 8],
            key: Pubkey::default(),
            authority: Pubkey::default(),
            created_at: calculator::current_time(),
            user_status: UserStatus::NORMAL,
            padding: [0u8; 7],
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub enum UserStatus {
    #[default]
    NORMAL,
    LIQUIDATION,
    DISABLE,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UserTokenUpdateReason {
    #[default]
    DEFAULT,
    DEPOSIT,
    WITHDRAW,
    SettleFee,
    SettlePnl,
    DecreasePosition,
    IncreasePosition,
    UpdateLeverage,
    CollectOpenFee,
    CollectCloseFee,
    TransferToStake,
    TransferFromStake,
    LiquidateLiability,
    Liquidation,
}

impl Size for User {
    const SIZE: usize = std::mem::size_of::<User>() + 8;
}

impl User {
    pub fn get_user_token_mut_ref(&mut self, token_mint: &Pubkey) -> BumpResult<&mut UserToken> {
        self.get_user_token_index(token_mint)
            .map(move |user_token_index| &mut self.tokens[user_token_index])
    }

    pub fn get_user_token_ref(&self, token_mint: &Pubkey) -> BumpResult<&UserToken> {
        self.get_user_token_index(token_mint).map(|user_token_index| &self.tokens[user_token_index])
    }

    pub fn force_get_user_token_mut_ref(
        &mut self,
        token_mint: &Pubkey,
    ) -> BumpResult<&mut UserToken> {
        self.get_user_token_index(token_mint)
            .or_else(|_| self.add_user_token(token_mint))
            .map(move |user_token_index| &mut self.tokens[user_token_index])
    }

    pub fn add_user_token(&mut self, token_mint: &Pubkey) -> BumpResult<usize> {
        let new_user_token_index = self.next_usable_user_token_index()?;

        let new_user_token = UserToken {
            user_token_status: UserTokenStatus::USING,
            token_mint_key: *token_mint,
            ..UserToken::default()
        };
        self.tokens[new_user_token_index] = new_user_token;
        Ok(new_user_token_index)
    }

    pub fn get_user_token_index(&self, token_mint: &Pubkey) -> BumpResult<usize> {
        self.tokens
            .iter()
            .position(|user_token| {
                user_token.user_token_status.eq(&UserTokenStatus::USING)
                    && user_token.token_mint_key.eq(token_mint)
            })
            .ok_or(CouldNotFindUserToken)
    }

    pub fn get_user_stake_mut_ref(&mut self, pool_key: &Pubkey) -> BumpResult<&mut UserStake> {
        self.get_user_stake_index(pool_key)
            .map(move |user_stake_index| &mut self.stakes[user_stake_index])
    }

    pub fn get_user_stake_share(&mut self, pool_key: &Pubkey) -> BumpResult<u128> {
        Ok(self.get_user_stake_ref(pool_key)?.staked_share)
    }

    pub fn get_user_stake_ref(&self, pool_key: &Pubkey) -> BumpResult<&UserStake> {
        self.get_user_stake_index(pool_key).map(|user_stake| &self.stakes[user_stake])
    }

    pub fn get_or_new_user_stake_ref_mut(
        &mut self,
        pool_key: &Pubkey,
    ) -> BumpResult<&mut UserStake> {
        self.get_user_stake_index(pool_key)
            .or_else(|_| self.add_user_stake(pool_key))
            .map(move |user_stake_index| &mut self.stakes[user_stake_index])
    }

    pub fn add_user_stake(&mut self, pool_key: &Pubkey) -> BumpResult<usize> {
        let new_user_stake_index = self.next_usable_stake_index()?;

        let new_user_stake = UserStake {
            pool_key: *pool_key,
            user_stake_status: UserStakeStatus::USING,
            ..UserStake::default()
        };
        self.stakes[new_user_stake_index] = new_user_stake;
        Ok(new_user_stake_index)
    }

    pub fn get_user_stake_index(&self, pool_key: &Pubkey) -> BumpResult<usize> {
        self.stakes
            .iter()
            .position(|user_stake| {
                user_stake.user_stake_status.eq(&UserStakeStatus::USING)
                    && user_stake.pool_key.eq(pool_key)
            })
            .ok_or(CouldNotFindUserStake)
    }

    pub fn get_user_order_ref(&self, order_id: u64) -> BumpResult<&UserOrder> {
        self.get_user_order_index(order_id).map(|user_order| &self.orders[user_order])
    }

    pub fn add_user_order(&mut self, user_order: &UserOrder) -> BumpResult<usize> {
        let new_user_order_index = self.next_usable_order_index()?;

        let new_user_order = *user_order;
        self.orders[new_user_order_index] = new_user_order;
        Ok(new_user_order_index)
    }

    pub fn get_user_order_index(&self, order_id: u64) -> BumpResult<usize> {
        self.orders
            .iter()
            .position(|user_order| {
                user_order.status.eq(&OrderStatus::USING) && user_order.order_id == order_id
            })
            .ok_or(CouldNotFindUserOrder)
    }

    pub fn get_user_position_mut_ref(
        &mut self,
        position_key: &Pubkey,
    ) -> BumpResult<&mut UserPosition> {
        self.get_user_position_index(position_key)
            .map(move |user_position_index| &mut self.positions[user_position_index])
    }

    pub fn get_user_position_ref(&self, position_key: &Pubkey) -> BumpResult<&UserPosition> {
        self.get_user_position_index(position_key)
            .map(|user_position| &self.positions[user_position])
    }

    pub fn add_user_position(&mut self, position_key: &Pubkey) -> BumpResult<usize> {
        let new_user_position_index = self.next_usable_position_index()?;

        let new_user_position = UserPosition {
            position_key: *position_key,
            user_key: self.key,
            status: PositionStatus::USING,
            ..UserPosition::default()
        };
        self.positions[new_user_position_index] = new_user_position;
        Ok(new_user_position_index)
    }

    pub fn get_user_position_index(&self, position_key: &Pubkey) -> BumpResult<usize> {
        self.positions
            .iter()
            .position(|user_position| {
                user_position.status.eq(&PositionStatus::USING)
                    && user_position.position_key.eq(&position_key)
            })
            .ok_or(CouldNotFindUserPosition)
    }

    pub fn force_get_user_position_mut_ref(
        &mut self,
        position_key: &Pubkey,
    ) -> BumpResult<&mut UserPosition> {
        self.get_user_position_index(position_key)
            .or_else(|_| self.add_user_position(position_key))
            .map(move |user_position_index| &mut self.positions[user_position_index])
    }

    pub fn sub_user_stake(&mut self, pool_key: &Pubkey, stake_amount: u128) -> BumpResult<()> {
        let user_stake = self.get_user_stake_mut_ref(pool_key)?;
        user_stake.sub_staked_share(stake_amount)?;
        Ok(())
    }

    pub fn sub_order_hold_in_usd(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.hold >= amount, BumpErrorCode::OrderHoldUsdSmallThanHoldUsd)?;
        let pre_hold = self.hold;
        self.hold = calculator::sub_u128(self.hold, amount)?;
        emit!(UserHoldUpdateEvent {
            user_key: self.key,
            pre_hold_amount: pre_hold,
            hold_amount: self.hold,
        });
        Ok(())
    }

    pub fn add_order_hold_in_usd(&mut self, amount: u128) -> BumpResult<()> {
        let pre_hold = self.hold;
        self.hold = calculator::add_u128(self.hold, amount)?;
        emit!(UserHoldUpdateEvent {
            user_key: self.key,
            pre_hold_amount: pre_hold,
            hold_amount: self.hold,
        });
        Ok(())
    }

    pub fn use_token(&mut self, token: &Pubkey, amount: u128, is_check: bool) -> BumpResult<u128> {
        let use_from_balance;
        let user_token = self.force_get_user_token_mut_ref(&token)?;
        if is_check {
            validate!(
                user_token.amount >= user_token.used_amount,
                BumpErrorCode::AmountNotEnough.into()
            )?;
        };
        if user_token.amount >= user_token.used_amount + amount {
            user_token.add_used_amount(amount)?;
            use_from_balance = amount;
        } else if user_token.amount > user_token.used_amount {
            use_from_balance = user_token.amount - user_token.used_amount;
            user_token.add_used_amount(amount)?;
        } else {
            user_token.add_used_amount(amount)?;
            use_from_balance = 0u128;
        }

        Ok(use_from_balance)
    }
    pub fn cross_used(&self) -> BumpResult<bool> {
        for token in &self.tokens {
            if token.liability_amount.gt(&0u128) {
                return Ok(true);
            }
        }
        for position in &self.positions {
            if position.is_portfolio_margin {
                return Ok(true);
            }
        }
        Ok(false)
    }
    pub fn un_use_token(&mut self, token: &Pubkey, amount: u128) -> BumpResult<()> {
        let user_token = self.get_user_token_mut_ref(token)?;
        validate!(user_token.used_amount >= amount, BumpErrorCode::AmountNotEnough.into())?;
        user_token.sub_used_amount(amount)?;
        Ok(())
    }

    pub fn make_order_is_allowed(
        &self,
        symbol: [u8; 32],
        is_portfolio_margin: bool,
        is_long: bool,
        program_id: &Pubkey,
    ) -> BumpResult<bool> {
        let position_key =
            pda::generate_position_key(&self.key, symbol, is_portfolio_margin, program_id)?;
        for position in &self.positions {
            if position.status.eq(&PositionStatus::INIT) {
                continue;
            }
            if position_key.eq(&position.position_key) {
                if is_long != position.is_long {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub fn next_usable_order_index(&self) -> BumpResult<usize> {
        for (index, order) in self.orders.iter().enumerate() {
            if order.status.eq(&OrderStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::NoMoreOrderSpace)
    }

    pub fn next_usable_user_token_index(&self) -> BumpResult<usize> {
        for (index, user_token) in self.tokens.iter().enumerate() {
            if user_token.user_token_status.eq(&UserTokenStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::NoMoreUserTokenSpace)
    }

    pub fn next_usable_position_index(&self) -> BumpResult<usize> {
        for (index, position) in self.positions.iter().enumerate() {
            if position.status.eq(&PositionStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::AmountNotEnough)
    }

    pub fn next_usable_stake_index(&self) -> BumpResult<usize> {
        for (index, user_stake) in self.stakes.iter().enumerate() {
            if user_stake.user_stake_status.eq(&UserStakeStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::AmountNotEnough)
    }

    pub fn add_order(&mut self, order: &UserOrder, index: usize) -> BumpResult {
        self.orders[index] = *order;
        emit!(AddOrDeleteUserOrderEvent { user_key: self.key, order: *order, is_add: true });
        Ok(())
    }

    pub fn delete_order(&mut self, order_id: u64) -> BumpResult {
        match self.get_user_order_index(order_id) {
            Ok(order_index) => {
                let user_key = self.key;
                let order = self.orders[order_index];
                self.orders[order_index] = UserOrder::default();
                emit!(AddOrDeleteUserOrderEvent { user_key, order, is_add: false });
            },
            Err(_e) => {
                //order not exist, do nothing
            },
        }
        Ok(())
    }

    pub fn delete_user_stake(&mut self, pool_key: &Pubkey) -> BumpResult {
        let index = self.get_user_stake_index(pool_key)?;
        self.stakes[index] = UserStake::default();
        Ok(())
    }

    pub fn cancel_stop_orders(
        &mut self,
        order_id: u64,
        symbol: [u8; 32],
        margin_token: &Pubkey,
        is_portfolio_margin: bool,
    ) -> BumpResult<()> {
        for user_order in self.orders {
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
                self.delete_order(user_order.order_id)?;
            }
        }
        Ok(())
    }

    pub fn delete_position(&mut self, position_key: &Pubkey) -> BumpResult {
        let position_index = self
            .positions
            .iter()
            .position(|user_position: &UserPosition| user_position.position_key.eq(&position_key))
            .ok_or(CouldNotFindUserPosition)?;
        self.positions[position_index] = UserPosition::default();
        Ok(())
    }
    pub fn get_order_leverage(
        &self,
        symbol: [u8; 32],
        order_side: OrderSide,
        is_portfolio_margin: bool,
        leverage: u32,
    ) -> BumpResult<u32> {
        for order in self.orders {
            if order.symbol == symbol
                && order.order_side.eq(&order_side)
                && order.position_side.eq(&PositionSide::DECREASE)
                && order.is_portfolio_margin == is_portfolio_margin
            {
                return Ok(order.leverage);
            }
        }
        Ok(leverage)
    }

    pub fn update_all_orders_leverage(
        &mut self,
        leverage: u32,
        symbol: [u8; 32],
        margin_token: &Pubkey,
        is_long: bool,
        is_portfolio_margin: bool,
    ) -> BumpResult {
        for user_order in &mut self.orders {
            if user_order.status.eq(&OrderStatus::INIT) {
                continue;
            }
            let is_long_order = user_order.order_side.eq(&OrderSide::LONG);
            if user_order.is_portfolio_margin == is_portfolio_margin
                && user_order.symbol == symbol
                && user_order.margin_mint_key.eq(margin_token)
                && ((is_long_order == is_long
                    && user_order.position_side.eq(&PositionSide::INCREASE))
                    || (is_long_order != user_order.position_side.eq(&PositionSide::DECREASE)))
            {
                user_order.set_leverage(leverage)
            }
        }
        Ok(())
    }

    pub fn update_all_position_from_portfolio_margin(
        &mut self,
        change_token_amount: i128,
        token_mint: &Pubkey,
    ) -> BumpResult<()> {
        let mut reduce_amount = change_token_amount;
        for position in &mut self.positions {
            if position.status.eq(&PositionStatus::INIT) {
                continue;
            }
            if position.margin_mint_key.eq(token_mint) && position.is_portfolio_margin {
                let change_amount;

                if change_token_amount > 0i128 {
                    let borrowing_margin = position
                        .initial_margin_usd
                        .safe_sub(position.initial_margin_usd_from_portfolio)?
                        .safe_mul(position.initial_margin)?
                        .safe_div(position.initial_margin_usd)?;
                    change_amount = change_token_amount.abs().cast::<u128>()?.min(borrowing_margin);
                    position.add_initial_margin_usd_from_portfolio(
                        change_amount
                            .safe_mul(position.initial_margin_usd)?
                            .safe_div(position.initial_margin)?,
                    )?;
                } else {
                    let add_borrow_margin_in_usd = change_token_amount
                        .abs()
                        .cast::<u128>()?
                        .safe_mul(position.initial_margin_usd)?
                        .safe_div(position.initial_margin)?;

                    if position.initial_margin_usd_from_portfolio <= add_borrow_margin_in_usd {
                        position.set_initial_margin_usd_from_portfolio(0u128)?;
                        change_amount = 0u128;
                    } else {
                        position.sub_initial_margin_usd_from_portfolio(add_borrow_margin_in_usd)?;
                        change_amount = change_token_amount.abs().cast::<u128>()?;
                    }
                }

                reduce_amount = if change_token_amount > 0i128 {
                    reduce_amount
                        .cast::<i128>()?
                        .safe_sub(change_amount.cast::<i128>()?)?
                        .cast::<i128>()?
                } else {
                    reduce_amount
                        .cast::<i128>()?
                        .safe_add(change_amount.cast::<i128>()?)?
                        .cast::<i128>()?
                };

                if reduce_amount == 0i128 {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn cancel_user_order(&mut self, user_order_index: usize) -> BumpResult {
        let user_order = self.orders[user_order_index];
        if user_order.is_portfolio_margin {
            self.sub_order_hold_in_usd(user_order.order_margin)?;
        }
        self.orders[user_order_index] = UserOrder::default();
        Ok(())
    }

    pub fn sub_user_token_amount(&mut self, mint: &Pubkey, mut amount: u128) -> BumpResult {
        for user_position in &mut self.positions {
            if user_position.status.eq(&PositionStatus::INIT) {
                continue;
            }
            if user_position.is_portfolio_margin
                && user_position.margin_mint_key.eq(mint)
                && amount > 0
            {
                let reduce_amount = user_position.reduce_position_portfolio_balance(amount)?;
                amount = amount.safe_sub(reduce_amount)?;
            }
        }
        let user_token = self.get_user_token_mut_ref(mint)?;
        user_token.sub_amount(amount)?;
        Ok(())
    }

    #[track_caller]
    pub fn sub_user_token_amount_ignore_used_amount(
        &mut self,
        token_mint: &Pubkey,
        amount: u128,
        user_token_update_origin: &UserTokenUpdateReason,
    ) -> BumpResult {
        let user_key = self.key;
        let user_token = self.get_user_token_mut_ref(token_mint)?;
        validate!(user_token.amount >= amount, BumpErrorCode::AmountNotEnough)?;
        validate!(
            user_token.amount >= user_token.used_amount.safe_add(amount)?,
            BumpErrorCode::AmountNotEnough
        )?;

        let pre_user_token = user_token.clone();

        user_token.sub_amount(amount)?;
        emit!(UserTokenBalanceUpdateEvent {
            user_key,
            token_mint: *token_mint,
            pre_user_token,
            user_token: user_token.clone(),
            update_origin: *user_token_update_origin,
        });
        Ok(())
    }

    pub fn repay_liability(
        &mut self,
        token_mint: &Pubkey,
        by: UserTokenUpdateReason,
    ) -> BumpResult<u128> {
        let user_key = self.key;
        let user_token = self.get_user_token_mut_ref(token_mint)?;
        if user_token.liability_amount > 0 && user_token.amount > 0 {
            let pre_user_token = *user_token;

            let repay_liability_amount = if user_token.amount >= user_token.liability_amount {
                user_token.liability_amount
            } else {
                user_token.amount
            };
            user_token.amount = user_token.amount.safe_sub(repay_liability_amount)?;
            user_token.liability_amount =
                user_token.liability_amount.safe_sub(repay_liability_amount)?;
            user_token.used_amount = user_token.used_amount.safe_sub(repay_liability_amount)?;
            emit!(UserTokenBalanceUpdateEvent {
                user_key,
                token_mint: *token_mint,
                pre_user_token,
                user_token: user_token.clone(),
                update_origin: by,
            });
            Ok(repay_liability_amount)
        } else {
            Ok(0)
        }
    }

    pub fn sub_token_with_liability(
        &mut self,
        token_mint: &Pubkey,
        trade_token: &mut TradeToken,
        amount: u128,
        user_token_update_origin: &UserTokenUpdateReason,
    ) -> BumpResult<u128> {
        let mut liability = 0u128;
        let user_key = self.key;
        let user_token = self.get_user_token_mut_ref(token_mint)?;
        let pre_user_token = *user_token;
        if user_token.amount >= amount {
            user_token.amount = user_token.amount.safe_sub(amount)?;
        } else if user_token.amount > 0u128 {
            liability = amount.safe_sub(user_token.amount)?;
            user_token.liability_amount = user_token.liability_amount.safe_add(liability)?;
            user_token.used_amount = user_token.used_amount.safe_add(liability)?;
            user_token.amount = 0u128;
        } else {
            user_token.liability_amount = user_token.liability_amount.safe_add(amount)?;
            user_token.used_amount = user_token.used_amount.safe_add(amount)?;
            liability = amount;
        }
        trade_token.add_total_liability(liability)?;
        emit!(UserTokenBalanceUpdateEvent {
            user_key,
            token_mint: *token_mint,
            pre_user_token,
            user_token: user_token.clone(),
            update_origin: *user_token_update_origin,
        });
        Ok(liability)
    }

    pub fn add_user_token_amount(
        &mut self,
        token_mint: &Pubkey,
        amount: u128,
        user_token_update_origin: &UserTokenUpdateReason,
    ) -> BumpResult {
        let user_key = self.key;
        let user_token = self.get_user_token_mut_ref(token_mint)?;
        let pre_user_token = user_token.clone();
        user_token.add_amount(amount)?;
        emit!(UserTokenBalanceUpdateEvent {
            user_key,
            token_mint: *token_mint,
            pre_user_token,
            user_token: user_token.clone(),
            update_origin: *user_token_update_origin,
        });
        Ok(())
    }

    pub fn get_portfolio_net_value(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
    ) -> BumpResult<u128> {
        let mut total_token_net_value = 0u128;
        for user_token in &self.tokens {
            if user_token.user_token_status.eq(&UserTokenStatus::INIT) {
                continue;
            }
            let trade_token =
                trade_token_map.get_trade_token_by_mint_ref(&user_token.token_mint_key)?;
            let oracle_price = oracle_map.get_price_data(&trade_token.feed_id)?.price;
            total_token_net_value = total_token_net_value
                .safe_add(user_token.get_token_net_value(&trade_token, oracle_price)?)?;
        }
        Ok(total_token_net_value)
    }

    pub fn get_available_value(
        &mut self,
        trade_token_map: &TradeTokenMap,
        mut oracle_map: &mut OracleMap,
    ) -> BumpResult<i128> {
        let cross_net_value = self.get_cross_net_value_and_pos_size(trade_token_map, oracle_map)?;

        let (_, total_un_pnl_usd, _total_position_mm, _, _total_im_from_portfolio_value) =
            self.get_user_cross_position_available_value(&mut oracle_map, trade_token_map)?;

        let (_, _total_token_borrowing_value) =
            self.get_total_used_value(&trade_token_map, &mut oracle_map)?;
        let pnl_usd = if total_un_pnl_usd.gt(&0i128) { total_un_pnl_usd } else { 0i128 };
        Ok(cross_net_value.0.safe_sub(pnl_usd)?.safe_sub(
            _total_im_from_portfolio_value.safe_add(_total_token_borrowing_value)?.cast()?,
        )?)
    }

    //this cross_net_value should sub total_pos_fee
    pub fn get_cross_net_value_and_pos_size(
        &mut self,
        trade_token_map: &TradeTokenMap,
        mut oracle_map: &mut OracleMap,
    ) -> BumpResult<(i128, u128)> {
        let portfolio_net_value =
            self.get_portfolio_net_value(&trade_token_map, &mut oracle_map)?;
        msg!(
            "============get_cross_net_value_and_pos_size,portfolio_net_value:{}",
            portfolio_net_value
        );
        let (used_value, _total_token_borrowing_value) =
            self.get_total_used_value(&trade_token_map, &mut oracle_map)?;
        let (
            total_im_usd,
            total_un_pnl_usd,
            _total_position_mm,
            total_size,
            _total_im_from_portfolio_value,
        ) = self.get_user_cross_position_available_value(&mut oracle_map, trade_token_map)?;
        let cross_net_value = portfolio_net_value
            .safe_add(total_im_usd)?
            .safe_add(self.hold)?
            .cast::<i128>()?
            .safe_add(total_un_pnl_usd)?
            .safe_sub(used_value.cast()?)?;
        Ok((cross_net_value, total_size))
    }

    pub fn get_user_cross_position_available_value(
        &self,
        price_map: &mut OracleMap,
        trade_token_map: &TradeTokenMap,
    ) -> BumpResult<(u128, i128, u128, u128, u128)> {
        let mut total_im_usd = 0u128;
        let mut total_im_usd_from_portfolio = 0u128;
        let mut total_un_pnl_usd = 0i128;
        let mut total_position_mm = 0u128;
        let mut total_size = 0u128;

        for user_position in &self.positions {
            if user_position.status.eq(&PositionStatus::INIT) && !user_position.is_portfolio_margin
            {
                continue;
            }

            total_im_usd = total_im_usd.safe_add(user_position.initial_margin_usd)?;
            let index_price = price_map.get_price_data(&user_position.index_mint_oracle)?.price;
            let trade_token =
                trade_token_map.get_trade_token_by_mint_ref(&user_position.margin_mint_key)?;
            let position_un_pnl_usd = user_position.get_position_un_pnl_usd(index_price)?;
            total_un_pnl_usd = total_un_pnl_usd.safe_add(if position_un_pnl_usd > 0 {
                calculator::mul_rate_i(position_un_pnl_usd, trade_token.discount.cast::<i128>()?)?
            } else {
                calculator::mul_rate_i(
                    position_un_pnl_usd,
                    RATE_PRECISION.safe_add(trade_token.liquidation_factor.cast()?)?.cast()?,
                )?
            })?;

            total_position_mm = total_position_mm.safe_add(user_position.mm_usd)?;
            total_size = total_size.safe_add(user_position.position_size)?;
            total_im_usd_from_portfolio = total_im_usd_from_portfolio
                .safe_add(user_position.initial_margin_usd_from_portfolio)?;
        }
        Ok((
            total_im_usd,
            total_un_pnl_usd,
            total_position_mm,
            total_size,
            total_im_usd_from_portfolio,
        ))
    }

    pub fn get_total_used_value(
        &self,
        trade_token_map: &TradeTokenMap,
        oracle_map: &mut OracleMap,
    ) -> BumpResult<(u128, u128)> {
        let mut total_used_value = 0u128;
        let mut total_token_borrowing_value = 0u128;
        for user_token in &self.tokens {
            if user_token.user_token_status.eq(&UserTokenStatus::INIT) {
                continue;
            }
            let trade_token =
                trade_token_map.get_trade_token_by_mint_ref(&user_token.token_mint_key)?;
            let oracle_price = oracle_map.get_price_data(&trade_token.feed_id)?;
            total_used_value = total_used_value
                .safe_add(user_token.get_token_used_value(&trade_token, &oracle_price)?)?;
            total_token_borrowing_value = total_token_borrowing_value
                .safe_add(user_token.get_token_borrowing_value(&oracle_price, &trade_token)?)?;
        }
        if self.hold > 0 {
            total_used_value = total_used_value.safe_add(self.hold)?;
            total_token_borrowing_value = total_token_borrowing_value.safe_add(self.hold)?;
        }
        Ok((total_used_value, total_token_borrowing_value))
    }

    pub fn cancel_all_cross_orders(&mut self) -> BumpResult<()> {
        let user_orders_length = self.orders.len();
        for index in 0..user_orders_length {
            let order = self.orders[index];
            if order.status.eq(&OrderStatus::USING) && order.is_portfolio_margin {
                self.cancel_user_order(index)?;
            }
        }
        Ok(())
    }

    pub fn cancel_order<'info>(
        &mut self,
        order: &UserOrder,
        token_program: &Program<'info, Token>,
        pool_vault: &Account<'info, TokenAccount>,
        user_token_account: Option<&Account<'info, TokenAccount>>,
        bump_signer: &AccountInfo<'info>,
        state: &Account<'info, State>,
    ) -> BumpResult<()> {
        self.delete_order(order.order_id)?;
        if order.position_side.eq(&PositionSide::INCREASE) && order.is_portfolio_margin {
            self.sub_order_hold_in_usd(order.order_margin)?;
        } else if order.position_side.eq(&PositionSide::INCREASE)
            && !order.is_portfolio_margin
            && user_token_account.is_some()
        {
            token::send_from_program_vault(
                token_program,
                pool_vault,
                user_token_account.unwrap(),
                bump_signer,
                state.bump_signer_nonce,
                order.order_margin,
            )
            .map_err(|_e| BumpErrorCode::TransferFailed)?;
        }
        Ok(())
    }
}
