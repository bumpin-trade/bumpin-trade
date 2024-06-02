use anchor_lang::prelude::*;
use crate::errors::BumpErrorCode::{CouldNotFindUserPosition, CouldNotFindUserStake, CouldNotFindUserToken};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::state::infrastructure::user_order::{OrderSide, OrderStatus, PositionSide, UserOrder};
use crate::state::infrastructure::user_position::{PositionStatus, UserPosition};
use crate::state::infrastructure::user_stake::{UserRewards, UserStake};
use crate::state::infrastructure::user_token::UserToken;
use crate::state::traits::Size;
use crate::validate;
use solana_program::msg;
use crate::instructions::cal_utils;


#[account(zero_copy(unsafe))]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct User {
    pub user_key: Pubkey,
    pub authority: Pubkey,
    pub next_order_id: u128,
    pub next_liquidation_id: u128,
    pub hold: u128,
    pub user_tokens: [UserToken; 10],
    pub user_stakes: [UserStake; 10],
    pub user_positions: [UserPosition; 24],
    pub user_orders: [UserOrder; 16],
    pub user_rewards: [UserRewards; 10],
}

impl Size for User {
    const SIZE: usize = std::mem::size_of::<User>() + 8;
}

impl User {
    pub fn get_user_token_mut(&mut self, mint: &Pubkey) -> BumpResult<&mut UserToken> {
        Ok(self.user_tokens.iter_mut()
            .find(|user_token| user_token.token_mint.eq(mint))
            .ok_or(CouldNotFindUserToken)?)
    }
    pub fn get_user_token_ref(&self, mint: &Pubkey) -> BumpResult<&UserToken> {
        Ok(self.user_tokens.iter()
            .find(|&user_token| user_token.token_mint.eq(mint))
            .ok_or(CouldNotFindUserToken)?)
    }

    pub fn get_user_stake_mut(&mut self, pool_index: usize) -> BumpResult<&mut UserStake> {
        Ok(self.user_stakes.get_mut(pool_index).ok_or(CouldNotFindUserStake)?)
    }
    pub fn get_user_stake_ref(&mut self, pool_index: usize) -> BumpResult<&UserStake> {
        Ok(self.user_stakes.get(pool_index).ok_or(CouldNotFindUserStake)?)
    }

    pub fn sub_order_hold_in_usd(&mut self, amount: u128) -> BumpResult<()> {
        validate!(self.hold >= amount,BumpErrorCode::AmountNotEnough.into());
        self.hold = cal_utils::sub_u128(self.hold, amount)?;
        Ok(())
    }

    pub fn add_order_hold_in_usd(&mut self, amount: u128) -> BumpResult<()> {
        self.hold += amount;
        Ok(())
    }

    pub fn use_token(&mut self, token: &Pubkey, amount: u128, is_check: bool) -> BumpResult<u128> {
        let use_from_balance;
        let token_balance = self.get_user_token_mut(token)?;
        if is_check {
            validate!(token_balance.amount >= token_balance.used_amount, BumpErrorCode::AmountNotEnough.into());
        };
        if token_balance.amount >= token_balance.used_amount + amount {
            token_balance.add_token_used_amount(amount);
            use_from_balance = amount;
        } else if token_balance.amount > token_balance.used_amount {
            use_from_balance = token_balance.amount - token_balance.used_amount;
            token_balance.add_token_used_amount(amount);
        } else {
            token_balance.add_token_used_amount(amount);
            use_from_balance = 0u128;
        }

        Ok(use_from_balance)
    }

    pub fn un_use_token(&mut self, token: &Pubkey, amount: u128) -> BumpResult<()> {
        let token_balance = self.get_user_token_mut(token)?;
        validate!(token_balance.used_amount > amount, BumpErrorCode::AmountNotEnough.into());
        token_balance.sub_token_used_amount(amount)?;
        Ok(())
    }

    pub fn find_position_by_seed(&self, user: &Pubkey, symbol: [u8; 32], token: &Pubkey, is_cross_margin: bool, program_id: &Pubkey) -> BumpResult<Option<&UserPosition>> {
        let position_key = self.generate_position_key(user, symbol, token, is_cross_margin, program_id)?;
        Ok(self.user_positions.iter().find(|position| position.position_key.eq(&position_key)))
    }

    pub fn generate_position_key(&self, user: &Pubkey, symbol: [u8; 32], token: &Pubkey, is_cross_margin: bool, program_id: &Pubkey) -> BumpResult<Pubkey> {
        // Convert is_cross_margin to a byte array
        let is_cross_margin_bytes: &[u8] = if is_cross_margin { &[1] } else { &[0] };
        // Create the seeds array by concatenating the byte representations
        let seeds: &[&[u8]] = &[
            user.as_ref(),
            &symbol,
            token.as_ref(),
            is_cross_margin_bytes,
        ];

        // Find the program address
        let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
        Ok(address)
    }

    pub fn find_position_by_key(&self, position_key: &Pubkey) -> BumpResult<&UserPosition> {
        Ok(self.user_positions.iter()
            .find(|user_position| user_position.position_key.eq(position_key))
            .ok_or(CouldNotFindUserPosition)?)
    }

    pub fn find_position_mut_by_key(&mut self, position_key: &Pubkey) -> BumpResult<&mut UserPosition> {
        Ok(self.user_positions.iter_mut()
            .find(|user_position| user_position.position_key.eq(position_key))
            .ok_or(CouldNotFindUserPosition)?)
    }

    pub fn find_position_ref_by_key(&mut self, position_key: &Pubkey) -> BumpResult<&UserPosition> {
        Ok(self.user_positions.iter_mut()
            .find(|user_position| user_position.position_key.eq(position_key))
            .ok_or(CouldNotFindUserPosition)?)
    }

    pub fn find_ref_order_by_id(&mut self, order_id: u128) -> BumpResult<&UserOrder> {
        let index = self.get_order_index_by_id(order_id);
        Ok(&self.user_orders[index])
    }
    pub fn find_mut_order_by_id(&mut self, order_id: u128) -> BumpResult<&mut UserOrder> {
        let index = self.get_order_index_by_id(order_id);
        Ok(&mut self.user_orders[index])
    }

    pub fn has_other_short_order(&self, symbol: [u8; 32], margin_token: Pubkey, is_cross_margin: bool) -> BumpResult<bool> {
        for order in self.user_orders {
            if order.symbol.eq(&symbol) &&
                order.margin_token.eq(&margin_token) &&
                order.cross_margin.eq(&is_cross_margin) &&
                order.position_side.eq(&PositionSide::INCREASE) &&
                order.order_side.eq(&OrderSide::SHORT) {
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

    pub fn next_usable_position_index(&self) -> BumpResult<usize> {
        for (index, position) in self.user_positions.iter().enumerate() {
            if position.status.eq(&PositionStatus::INIT) {
                return Ok(index);
            }
        }
        Err(BumpErrorCode::AmountNotEnough)
    }


    pub fn add_position(&mut self, position: UserPosition, index: usize) -> BumpResult<> {
        self.user_positions[index] = position;
        Ok(())
    }

    pub fn add_order(&mut self, order: UserOrder, index: usize) -> BumpResult<> {
        self.user_orders[index] = order;
        Ok(())
    }

    pub fn delete_order(&mut self, order_id: u128) -> BumpResult<> {
        let order_index = self.get_order_index_by_id(order_id);
        self.user_orders[order_index] = UserOrder::default();
        Ok(())
    }

    pub fn get_order_leverage(&self, symbol: [u8; 32], order_side: OrderSide, is_cross_margin: bool, leverage: u128) -> BumpResult<u128> {
        for order in self.user_orders {
            if order.symbol == symbol &&
                order.order_side.eq(&order_side) &&
                order.position_side.eq(&PositionSide::DECREASE) &&
                order.cross_margin == is_cross_margin {
                return Ok(order.leverage);
            }
        }
        Ok(leverage)
    }


    pub fn update_all_orders_leverage(&mut self, leverage: u128, symbol: [u8; 32], margin_token: &Pubkey, is_long: bool, is_cross_margin: bool) -> BumpResult {
        for mut user_order in self.user_orders {
            let is_long_order = user_order.order_side.eq(&OrderSide::LONG);
            if user_order.cross_margin == is_cross_margin && user_order.symbol == symbol && user_order.margin_token.eq(margin_token) && ((is_long_order == is_long && user_order.position_side.eq(&PositionSide::INCREASE)) || (is_long_order != user_order.position_side.eq(&PositionSide::DECREASE))) {
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
        self.user_orders.iter().position(|user_order| user_order.order_id == order_id).ok_or(BumpErrorCode::OrderNotExist).unwrap()
    }
}

