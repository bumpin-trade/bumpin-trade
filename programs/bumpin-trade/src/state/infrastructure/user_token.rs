use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::state::oracle::OraclePriceData;
use crate::state::trade_token::TradeToken;

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub struct UserToken {
    pub user_token_status: UserTokenStatus,
    pub token_mint: Pubkey,
    pub user_token_account_key: Pubkey,
    pub amount: u128,
    pub used_amount: u128,
    pub liability: u128,
}

#[repr(C)]
#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UserTokenStatus {
    #[default]
    INIT,
    USING,
}

impl UserToken {
    pub fn add_token_amount(&mut self, amount: u128) -> BumpResult {
        self.amount = self.amount.safe_add(amount)?;
        Ok(())
    }
    pub fn sub_token_amount(&mut self, amount: u128) -> BumpResult {
        self.amount = self.amount.safe_sub(amount)?;
        Ok(())
    }
    pub fn sub_token_used_amount(&mut self, amount: u128) -> BumpResult {
        self.used_amount = self.used_amount.safe_sub(amount)?;
        Ok(())
    }

    pub fn add_token_used_amount(&mut self, amount: u128) -> BumpResult {
        self.used_amount = self.used_amount.safe_add(amount)?;
        Ok(())
    }

    pub fn get_token_net_value(
        &self,
        trade_token: &TradeToken,
        oracle_price_data: &OraclePriceData,
    ) -> BumpResult<u128> {
        if self.amount > self.used_amount {
            let token_net_value = self
                .amount
                .safe_sub(self.used_amount)?
                .safe_mul(oracle_price_data.price)?
                .safe_mul(trade_token.discount)?;
            return Ok(token_net_value);
        }
        Ok(0u128)
    }

    pub fn get_token_used_value(
        &self,
        trade_token: &TradeToken,
        oracle_price_data: &OraclePriceData,
    ) -> BumpResult<u128> {
        if self.amount < self.used_amount {
            let token_used_value = self
                .used_amount
                .cast::<u128>()?
                .safe_sub(self.amount.cast()?)?
                .safe_mul(oracle_price_data.price.cast()?)?
                .safe_mul(1u128.safe_add(trade_token.liquidation_factor.cast()?)?)?;
            return Ok(token_used_value);
        }
        Ok(0u128)
    }

    pub fn get_token_available_amount(&self) -> BumpResult<u128> {
        if self.amount > self.used_amount {
            return Ok(self.amount.safe_sub(self.used_amount)?);
        };
        Ok(0u128)
    }

    pub fn get_token_borrowing_value(
        &self,
        oracle_price_data: &OraclePriceData,
    ) -> BumpResult<u128> {
        let borrowing_amount = self.amount.safe_sub(self.used_amount)?.safe_sub(self.liability)?;

        if borrowing_amount > 0 {
            let token_borrowing_value =
                borrowing_amount.cast::<u128>()?.safe_mul(oracle_price_data.price.cast()?)?;
            return Ok(token_borrowing_value);
        }
        Ok(0u128)
    }
}
