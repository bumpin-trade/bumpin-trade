use crate::errors::BumpErrorCode::{CouldNotFindUserPosition, CouldNotFindUserToken};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::instructions::cal_utils;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::{
    AddUserOrderEvent, UserHoldUpdateEvent, UserTokenBalanceUpdateEvent,
};
use crate::state::infrastructure::user_order::{OrderSide, OrderStatus, PositionSide, UserOrder};
use crate::state::infrastructure::user_position::{PositionStatus, UserPosition};
use crate::state::infrastructure::user_stake::{UserStake, UserStakeStatus};
use crate::state::infrastructure::user_token::{UserToken, UserTokenStatus};
use crate::state::traits::Size;
use crate::utils::pda;
use crate::validate;
use anchor_lang::prelude::*;

#[account(zero_copy(unsafe))]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct User {
    pub user_key: Pubkey,
    pub authority: Pubkey,
    pub next_order_id: u128,
    pub next_liquidation_id: u128,
    pub hold: u128,
    pub user_tokens: [UserToken; 12], //Max 32
    pub user_stakes: [UserStake; 12], //Max 32
    pub user_positions: [UserPosition; 10],
    pub user_orders: [UserOrder; 10],
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UserTokenUpdateOrigin {
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
    pub fn get_user_token_mut(&mut self, mint: &Pubkey) -> BumpResult<Option<&mut UserToken>> {
        Ok(self.user_tokens.iter_mut().find(|user_token| {
            user_token.token_mint.eq(mint)
                && user_token.user_token_status.eq(&UserTokenStatus::USING)
        }))
    }
    pub fn get_user_token_ref(&self, mint: &Pubkey) -> BumpResult<Option<&UserToken>> {
        Ok(self.user_tokens.iter().find(|&user_token| {
            user_token.token_mint.eq(mint)
                && user_token.user_token_status.eq(&UserTokenStatus::USING)
        }))
    }

    pub fn sub_user_stake(&mut self, pool_key: &Pubkey, stake_amount: u128) -> BumpResult<()> {
        let user_stake_option = self.get_user_stake_mut(pool_key)?;
        match user_stake_option {
            None => {
                Err(CouldNotFindUserToken)?;
            },
            Some(user_stake) => {
                user_stake.sub_user_stake(stake_amount)?;
            },
        }
        Ok(())
    }

    pub fn get_user_stake_ref(&self, pool_key: &Pubkey) -> BumpResult<Option<&UserStake>> {
        Ok(self.user_stakes.iter().find(|user_stake| {
            user_stake.pool_key.eq(pool_key)
                && user_stake.user_stake_status.eq(&UserStakeStatus::USING)
        }))
    }

    pub fn get_user_stake_mut(&mut self, pool_key: &Pubkey) -> BumpResult<Option<&mut UserStake>> {
        let stake = self.user_stakes.iter_mut().find(|user_stake| {
            user_stake.pool_key.eq(pool_key)
                && user_stake.user_stake_status.eq(&UserStakeStatus::USING)
        });
        Ok(stake)
    }

    pub fn sub_order_hold_in_usd(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.hold >= amount, BumpErrorCode::AmountNotEnough.into())?;
        let pre_hold = self.hold;
        self.hold = cal_utils::sub_u128(self.hold, amount)?;
        emit!(UserHoldUpdateEvent {
            user_key: self.user_key,
            pre_hold_amount: pre_hold,
            hold_amount: self.hold,
        });
        Ok(())
    }

    pub fn add_order_hold_in_usd(&mut self, amount: u128) -> BumpResult<()> {
        let pre_hold = self.hold;
        self.hold = self.hold.safe_add(amount)?;
        emit!(UserHoldUpdateEvent {
            user_key: self.user_key,
            pre_hold_amount: pre_hold,
            hold_amount: self.hold,
        });
        Ok(())
    }

    pub fn use_token(
        &mut self,
        token: &Pubkey,
        amount: u128,
        user_token_account_key: &Pubkey,
        is_check: bool,
    ) -> BumpResult<u128> {
        let use_from_balance;
        let user_token_option = self.get_user_token_mut(&token)?;
        let user_token = match user_token_option {
            None => {
                let index = self.next_usable_user_token_index()?;
                //init user_token
                let new_token = &mut UserToken {
                    user_token_status: UserTokenStatus::USING,
                    token_mint: *token,
                    user_token_account_key: *user_token_account_key,
                    amount: 0,
                    used_amount: 0,
                    liability: 0,
                };
                self.add_user_token(new_token, index)?;
                self.get_user_token_mut(token)?.ok_or(CouldNotFindUserToken)?
            },
            Some(exist_user_token) => exist_user_token,
        };
        if is_check {
            validate!(
                user_token.amount >= user_token.used_amount,
                BumpErrorCode::AmountNotEnough.into()
            )?;
        };
        if user_token.amount >= user_token.used_amount + amount {
            user_token.add_token_used_amount(amount)?;
            use_from_balance = amount;
        } else if user_token.amount > user_token.used_amount {
            use_from_balance = user_token.amount - user_token.used_amount;
            user_token.add_token_used_amount(amount)?;
        } else {
            user_token.add_token_used_amount(amount)?;
            use_from_balance = 0u128;
        }

        Ok(use_from_balance)
    }

    pub fn un_use_token(&mut self, token: &Pubkey, amount: u128) -> BumpResult<()> {
        let token_balance = self.get_user_token_mut(token)?.ok_or(CouldNotFindUserToken)?;
        validate!(token_balance.used_amount > amount, BumpErrorCode::AmountNotEnough.into())?;
        token_balance.sub_token_used_amount(amount)?;
        Ok(())
    }

    pub fn find_position_by_seed(
        &mut self,
        user: &Pubkey,
        symbol: [u8; 32],
        is_cross_margin: bool,
        program_id: &Pubkey,
    ) -> BumpResult<&mut UserPosition> {
        let position_key = pda::generate_position_key(user, symbol, is_cross_margin, program_id)?;
        Ok(self
            .user_positions
            .iter_mut()
            .find(|position| position.position_key.eq(&position_key))
            .ok_or(&mut UserPosition::default())
            .unwrap())
    }

    pub fn find_position_by_key(&self, position_key: &Pubkey) -> BumpResult<&UserPosition> {
        Ok(self
            .user_positions
            .iter()
            .find(|user_position| user_position.position_key.eq(position_key))
            .ok_or(CouldNotFindUserPosition)?)
    }

    pub fn find_position_mut_by_key(
        &mut self,
        position_key: &Pubkey,
    ) -> BumpResult<&mut UserPosition> {
        Ok(self
            .user_positions
            .iter_mut()
            .find(|user_position| user_position.position_key.eq(position_key))
            .ok_or(CouldNotFindUserPosition)?)
    }

    pub fn find_position_ref_by_key(&mut self, position_key: &Pubkey) -> BumpResult<&UserPosition> {
        Ok(self
            .user_positions
            .iter_mut()
            .find(|user_position| user_position.position_key.eq(position_key))
            .ok_or(CouldNotFindUserPosition)?)
    }

    pub fn find_ref_order_by_id(&self, order_id: u128) -> BumpResult<&UserOrder> {
        let index = self.get_order_index_by_id(order_id);
        Ok(&self.user_orders[index])
    }
    pub fn find_mut_order_by_id(&mut self, order_id: u128) -> BumpResult<&mut UserOrder> {
        let index = self.get_order_index_by_id(order_id);
        Ok(&mut self.user_orders[index])
    }

    pub fn has_other_short_order(
        &self,
        symbol: [u8; 32],
        margin_token: Pubkey,
        is_cross_margin: bool,
    ) -> BumpResult<bool> {
        for order in self.user_orders {
            if order.symbol.eq(&symbol)
                && order.margin_mint.eq(&margin_token)
                && order.cross_margin.eq(&is_cross_margin)
                && order.position_side.eq(&PositionSide::INCREASE)
                && order.order_side.eq(&OrderSide::SHORT)
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn has_other_order(&self, order_id: u128) -> BumpResult<bool> {
        for order in self.user_orders {
            if order.order_id == order_id {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn next_usable_order_index(&self) -> BumpResult<usize> {
        for (index, order) in self.user_orders.iter().enumerate() {
            if order.status.eq(&OrderStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::NoMoreOrderSpace)
    }

    pub fn next_usable_user_token_index(&self) -> BumpResult<usize> {
        for (index, user_token) in self.user_tokens.iter().enumerate() {
            if user_token.user_token_status.eq(&UserTokenStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::NoMoreOrderSpace)
    }

    pub fn next_usable_position_index(&self) -> BumpResult<usize> {
        for (index, position) in self.user_positions.iter().enumerate() {
            if position.status.eq(&PositionStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::AmountNotEnough)
    }

    pub fn next_usable_stake_index(&self) -> BumpResult<usize> {
        for (index, user_stake) in self.user_stakes.iter().enumerate() {
            if user_stake.user_stake_status.eq(&UserStakeStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::AmountNotEnough)
    }

    pub fn add_position(&mut self, position: &UserPosition, index: usize) -> BumpResult {
        self.user_positions[index] = *position;
        Ok(())
    }

    pub fn add_order(&mut self, order: &UserOrder, index: usize) -> BumpResult {
        self.user_orders[index] = *order;
        emit!(AddUserOrderEvent { user_key: self.user_key, order: *order });
        Ok(())
    }

    pub fn add_user_stake(&mut self, user_stake: &UserStake, index: usize) -> BumpResult {
        self.user_stakes[index] = *user_stake;
        Ok(())
    }

    pub fn add_user_token(&mut self, user_token: &UserToken, index: usize) -> BumpResult {
        self.user_tokens[index] = *user_token;
        Ok(())
    }

    pub fn delete_order(&mut self, order_id: u128) -> BumpResult {
        let order_index = self.get_order_index_by_id(order_id);
        self.user_orders[order_index] = UserOrder::default();
        Ok(())
    }

    pub fn delete_user_stake(&mut self, pool_key: &Pubkey) -> BumpResult {
        let order_index = self.get_user_stake_index_by_id(pool_key);
        self.user_tokens[order_index] = UserToken::default();
        Ok(())
    }

    pub fn delete_position(
        &mut self,
        symbol: [u8; 32],
        is_cross_margin: bool,
        program_id: &Pubkey,
    ) -> BumpResult {
        let position_key =
            pda::generate_position_key(&self.authority, symbol, is_cross_margin, program_id)?;
        let position_index = self
            .user_positions
            .iter()
            .position(|user_position: &UserPosition| user_position.position_key.eq(&position_key))
            .ok_or(CouldNotFindUserPosition)?;
        self.user_positions[position_index] = UserPosition::default();
        Ok(())
    }

    pub fn get_order_leverage(
        &self,
        symbol: [u8; 32],
        order_side: OrderSide,
        is_cross_margin: bool,
        leverage: u128,
    ) -> BumpResult<u128> {
        for order in self.user_orders {
            if order.symbol == symbol
                && order.order_side.eq(&order_side)
                && order.position_side.eq(&PositionSide::DECREASE)
                && order.cross_margin == is_cross_margin
            {
                return Ok(order.leverage);
            }
        }
        Ok(leverage)
    }

    pub fn update_all_orders_leverage(
        &mut self,
        leverage: u128,
        symbol: [u8; 32],
        margin_token: &Pubkey,
        is_long: bool,
        is_cross_margin: bool,
    ) -> BumpResult {
        for mut user_order in self.user_orders {
            if user_order.status.eq(&OrderStatus::INIT) {
                continue;
            }
            let is_long_order = user_order.order_side.eq(&OrderSide::LONG);
            if user_order.cross_margin == is_cross_margin
                && user_order.symbol == symbol
                && user_order.margin_mint.eq(margin_token)
                && ((is_long_order == is_long
                    && user_order.position_side.eq(&PositionSide::INCREASE))
                    || (is_long_order != user_order.position_side.eq(&PositionSide::DECREASE)))
            {
                user_order.set_leverage(leverage)
            }
        }
        Ok(())
    }

    pub fn cancel_user_order(&mut self, user_order_index: usize) -> BumpResult {
        let user_order = self.user_orders[user_order_index];
        if user_order.cross_margin {
            self.sub_order_hold_in_usd(user_order.order_margin)?;
        }
        self.user_orders[user_order_index] = UserOrder::default();
        Ok(())
    }
    fn get_order_index_by_id(&self, order_id: u128) -> usize {
        self.user_orders
            .iter()
            .position(|user_order| user_order.order_id == order_id)
            .ok_or(BumpErrorCode::OrderNotExist)
            .unwrap()
    }

    fn get_user_stake_index_by_id(&self, pool_key: &Pubkey) -> usize {
        self.user_stakes
            .iter()
            .position(|user_stake| user_stake.pool_key.eq(pool_key))
            .ok_or(BumpErrorCode::OrderNotExist)
            .unwrap()
    }

    pub fn sub_user_token_amount(&mut self, mint: &Pubkey, mut amount: u128) -> BumpResult {
        for user_position in &mut self.user_positions {
            if user_position.status.eq(&PositionStatus::INIT) {
                continue;
            }
            if user_position.cross_margin && user_position.margin_mint.eq(mint) && amount > 0 {
                let reduce_amount = user_position.reduce_position_portfolio_balance(amount)?;
                amount = amount.safe_sub(reduce_amount)?;
            }
        }
        let user_token = self.get_user_token_mut(mint)?.ok_or(CouldNotFindUserToken)?;
        user_token.sub_token_amount(amount)?;
        Ok(())
    }

    pub fn sub_user_token_amount_ignore_used_amount(
        &mut self,
        token_mint: &Pubkey,
        amount: u128,
        user_token_update_origin: &UserTokenUpdateOrigin,
    ) -> BumpResult {
        let user_key = self.user_key;
        let user_token = self.get_user_token_mut(token_mint)?.ok_or(CouldNotFindUserToken)?;
        validate!(user_token.amount >= amount, BumpErrorCode::AmountNotEnough)?;
        validate!(
            user_token.amount >= user_token.used_amount.safe_add(amount)?,
            BumpErrorCode::AmountNotEnough
        )?;

        let pre_user_token = user_token.clone();

        user_token.sub_token_amount(amount)?;
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
        user_token_update_origin: &UserTokenUpdateOrigin,
    ) -> BumpResult<u128> {
        let user_key = self.user_key;
        let user_token = self.get_user_token_mut(token_mint)?.ok_or(CouldNotFindUserToken)?;
        if user_token.liability > 0 && user_token.amount > 0 {
            let pre_user_token = user_token.clone();

            let repay_liability_amount = if user_token.amount >= user_token.liability {
                user_token.liability
            } else {
                user_token.amount
            };
            user_token.amount = user_token.amount.safe_sub(repay_liability_amount)?;
            user_token.liability = user_token.liability.safe_sub(repay_liability_amount)?;
            user_token.used_amount = user_token.used_amount.safe_sub(repay_liability_amount)?;
            emit!(UserTokenBalanceUpdateEvent {
                user_key,
                token_mint: *token_mint,
                pre_user_token,
                user_token: user_token.clone(),
                update_origin: *user_token_update_origin,
            });
            Ok(repay_liability_amount)
        } else {
            Ok(0)
        }
    }

    pub fn add_token(
        &mut self,
        token_mint: &Pubkey,
        amount: u128,
        user_token_update_origin: &UserTokenUpdateOrigin,
    ) -> BumpResult {
        let user_key = self.user_key;
        let user_token = self.get_user_token_mut(token_mint)?.ok_or(CouldNotFindUserToken)?;
        let pre_user_token = user_token.clone();
        user_token.add_token_amount(amount)?;
        emit!(UserTokenBalanceUpdateEvent {
            user_key,
            token_mint: *token_mint,
            pre_user_token,
            user_token: user_token.clone(),
            update_origin: *user_token_update_origin,
        });
        Ok(())
    }
}
