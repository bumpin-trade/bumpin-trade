use anchor_lang::prelude::*;

use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

use crate::errors::BumpResult;
use crate::instructions::calculator;
use crate::math::constants::RATE_PRECISION;
use crate::math::safe_math::SafeMath;
use crate::state::oracle::OraclePriceData;
use crate::state::trade_token::TradeToken;

#[bumpin_zero_copy_unsafe]
pub struct UserToken {
    pub amount: u128,
    pub used_amount: u128,
    pub liability_amount: u128,
    pub token_mint_key: Pubkey,
    pub user_token_status: UserTokenStatus,
    pub padding: [u8; 15],
    pub reserve_padding: [u8; 16],
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum UserTokenStatus {
    #[default]
    INIT,
    USING,
}

impl UserToken {
    pub fn add_amount(&mut self, amount: u128) -> BumpResult<Self> {
        let before = self.clone();
        self.amount = self.amount.safe_add(amount)?;
        Ok(before)
    }
    pub fn sub_amount(&mut self, amount: u128) -> BumpResult {
        self.amount = self.amount.safe_sub(amount)?;
        Ok(())
    }
    pub fn sub_used_amount(&mut self, amount: u128) -> BumpResult {
        self.used_amount = self.used_amount.safe_sub(amount)?;
        Ok(())
    }

    pub fn add_used_amount(&mut self, amount: u128) -> BumpResult {
        self.used_amount = self.used_amount.safe_add(amount)?;
        Ok(())
    }

    pub fn get_token_net_value(
        &self,
        trade_token: &TradeToken,
        oracle_price: u128,
    ) -> BumpResult<u128> {
        if self.amount > self.used_amount {
            let token_net_value = calculator::token_to_usd_u(
                self.amount.safe_sub(self.used_amount)?,
                trade_token.decimals,
                oracle_price,
            )?
            .safe_mul_rate(trade_token.discount as u128)?;
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
            let token_used_value = calculator::token_to_usd_u(
                self.used_amount.safe_sub(self.amount)?,
                trade_token.decimals,
                oracle_price_data.price,
            )?
            .safe_mul_rate(if trade_token.liquidation_factor > 0 {
                (trade_token.liquidation_factor as u128).safe_add(RATE_PRECISION)?
            } else {
                RATE_PRECISION
            })?;
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
        trade_token: &TradeToken,
    ) -> BumpResult<u128> {
        if self.used_amount.safe_sub(self.liability_amount)? < self.amount {
            return Ok(0u128);
        }

        let borrowing_amount =
            self.used_amount.safe_sub(self.liability_amount)?.safe_sub(self.amount)?;

        let token_borrowing_value = calculator::token_to_usd_u(
            borrowing_amount,
            trade_token.decimals,
            oracle_price_data.price,
        )?;
        Ok(token_borrowing_value)
    }
}
