use crate::errors::BumpResult;
use crate::instructions::cal_utils;
use crate::math::casting::Cast;
use crate::math::constants::RATE_PRECISION;
use crate::math::safe_math::SafeMath;
use crate::state::market::Market;
use crate::state::oracle_map::OracleMap;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;
use anchor_lang::prelude::*;
use bumpin_trade_attribute::bumpin_zero_copy_unsafe;

#[bumpin_zero_copy_unsafe]
pub struct UserPosition {
    pub position_size: u128,
    pub entry_price: u128,
    pub leverage: u32,
    pub initial_margin: u128,
    pub initial_margin_usd: u128,
    pub initial_margin_usd_from_portfolio: u128,
    pub mm_usd: u128,
    pub hold_pool_amount: u128,
    pub open_fee: u128,
    pub open_fee_in_usd: u128,
    pub realized_borrowing_fee: u128,
    pub realized_borrowing_fee_in_usd: u128,
    pub open_borrowing_fee_per_token: u128,
    pub realized_funding_fee: i128,
    pub realized_funding_fee_in_usd: i128,
    pub open_funding_fee_amount_per_size: i128,
    pub close_fee_in_usd: u128,
    pub last_update_time: i64,
    pub realized_pnl: i128,
    pub user_key: Pubkey,
    pub margin_mint_key: Pubkey,
    pub index_mint_key: Pubkey,
    pub position_key: Pubkey,
    pub symbol: [u8; 32],
    pub is_long: bool,
    pub cross_margin: bool,
    pub status: PositionStatus,
    pub padding: [u8; 13],
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Default, PartialEq, Debug, Eq)]
pub enum PositionStatus {
    #[default]
    INIT,
    USING,
}

impl UserPosition {
    pub fn add_position_size(&mut self, position_size: u128) -> BumpResult<()> {
        self.position_size = self.position_size.safe_add(position_size)?;
        Ok(())
    }

    pub fn sub_position_size(&mut self, position_size: u128) -> BumpResult<()> {
        self.position_size = self.position_size.safe_sub(position_size)?;
        Ok(())
    }

    pub fn set_entry_price(&mut self, entry_price: u128) -> BumpResult {
        self.entry_price = entry_price;
        Ok(())
    }

    pub fn set_initial_margin(&mut self, initial_margin: u128) -> BumpResult {
        self.initial_margin = initial_margin;
        Ok(())
    }

    pub fn set_initial_margin_usd(&mut self, initial_margin_usd: u128) -> BumpResult {
        self.initial_margin_usd = initial_margin_usd;
        Ok(())
    }

    pub fn set_leverage(&mut self, leverage: u32) -> BumpResult {
        self.leverage = leverage;
        Ok(())
    }

    pub fn set_realized_pnl(&mut self, realized_pnl: i128) -> BumpResult {
        self.realized_pnl = realized_pnl;
        Ok(())
    }

    pub fn set_open_borrowing_fee_per_token(
        &mut self,
        open_borrowing_fee_per_token: u128,
    ) -> BumpResult {
        self.open_borrowing_fee_per_token = open_borrowing_fee_per_token;
        Ok(())
    }

    pub fn set_open_funding_fee_amount_per_size(
        &mut self,
        open_funding_fee_amount_per_size: i128,
    ) -> BumpResult {
        self.open_funding_fee_amount_per_size = open_funding_fee_amount_per_size;
        Ok(())
    }

    pub fn add_initial_margin(&mut self, initial_margin: u128) -> BumpResult<()> {
        self.initial_margin = self.initial_margin.safe_add(initial_margin)?;
        Ok(())
    }

    pub fn add_realized_borrowing_fee(&mut self, realized_borrowing_fee: u128) -> BumpResult<()> {
        self.realized_borrowing_fee =
            self.realized_borrowing_fee.safe_add(realized_borrowing_fee)?;
        Ok(())
    }

    pub fn add_realized_funding_fee(&mut self, realized_funding_fee: i128) -> BumpResult<()> {
        self.realized_funding_fee = self.realized_funding_fee.safe_add(realized_funding_fee)?;
        Ok(())
    }

    pub fn add_realized_funding_fee_in_usd(
        &mut self,
        realized_funding_fee_in_usd: i128,
    ) -> BumpResult<()> {
        self.realized_funding_fee_in_usd =
            self.realized_funding_fee_in_usd.safe_add(realized_funding_fee_in_usd)?;
        Ok(())
    }

    pub fn add_realized_borrowing_fee_in_usd(
        &mut self,
        realized_borrowing_fee_in_usd: u128,
    ) -> BumpResult<()> {
        self.realized_borrowing_fee_in_usd =
            self.realized_borrowing_fee_in_usd.safe_add(realized_borrowing_fee_in_usd)?;
        Ok(())
    }

    pub fn sub_initial_margin(&mut self, initial_margin: u128) -> BumpResult<()> {
        self.initial_margin = self.initial_margin.safe_sub(initial_margin)?;
        Ok(())
    }

    pub fn add_initial_margin_usd(&mut self, initial_margin_usd: u128) -> BumpResult<()> {
        self.initial_margin_usd = self.initial_margin_usd.safe_add(initial_margin_usd)?;
        Ok(())
    }

    pub fn sub_initial_margin_usd(&mut self, initial_margin_usd: u128) -> BumpResult<()> {
        self.initial_margin_usd = self.initial_margin_usd.safe_sub(initial_margin_usd)?;
        Ok(())
    }

    pub fn add_initial_margin_usd_from_portfolio(
        &mut self,
        initial_margin_usd_from_portfolio: u128,
    ) -> BumpResult<()> {
        self.initial_margin_usd_from_portfolio =
            self.initial_margin_usd_from_portfolio.safe_add(initial_margin_usd_from_portfolio)?;
        Ok(())
    }

    pub fn sub_initial_margin_usd_from_portfolio(
        &mut self,
        initial_margin_usd_from_portfolio: u128,
    ) -> BumpResult<()> {
        self.initial_margin_usd_from_portfolio =
            self.initial_margin_usd_from_portfolio.safe_sub(initial_margin_usd_from_portfolio)?;
        Ok(())
    }

    pub fn set_initial_margin_usd_from_portfolio(
        &mut self,
        initial_margin_usd_from_portfolio: u128,
    ) -> BumpResult {
        self.initial_margin_usd_from_portfolio = initial_margin_usd_from_portfolio;
        Ok(())
    }

    pub fn set_close_fee_in_usd(&mut self, close_fee_in_usd: u128) -> BumpResult {
        self.close_fee_in_usd = close_fee_in_usd;
        Ok(())
    }

    pub fn set_position_size(&mut self, position_size: u128) -> BumpResult {
        self.position_size = position_size;
        Ok(())
    }

    pub fn add_hold_pool_amount(&mut self, hold_pool_amount: u128) -> BumpResult<()> {
        self.hold_pool_amount = self.hold_pool_amount.safe_add(hold_pool_amount)?;
        Ok(())
    }

    pub fn sub_hold_pool_amount(&mut self, hold_pool_amount: u128) -> BumpResult<()> {
        self.hold_pool_amount = self.hold_pool_amount.safe_add(hold_pool_amount)?;
        Ok(())
    }

    pub fn add_realized_pnl(&mut self, realized_pnl: i128) -> BumpResult<()> {
        self.realized_pnl = self.realized_pnl.safe_add(realized_pnl)?.cast::<i128>()?;
        Ok(())
    }

    pub fn sub_realized_borrowing_fee(&mut self, realized_borrowing_fee: u128) -> BumpResult<()> {
        self.realized_borrowing_fee =
            self.realized_borrowing_fee.safe_sub(realized_borrowing_fee)?;
        Ok(())
    }

    pub fn sub_realized_borrowing_fee_usd(
        &mut self,
        realized_borrowing_fee_in_usd: u128,
    ) -> BumpResult<()> {
        self.realized_borrowing_fee_in_usd =
            self.realized_borrowing_fee_in_usd.safe_sub(realized_borrowing_fee_in_usd)?;
        Ok(())
    }

    pub fn sub_realized_funding_fee(&mut self, realized_funding_fee: i128) -> BumpResult<()> {
        self.realized_funding_fee = self.realized_funding_fee.safe_sub(realized_funding_fee)?;
        Ok(())
    }

    pub fn sub_realized_funding_fee_usd(
        &mut self,
        realized_funding_fee_in_usd: i128,
    ) -> BumpResult<()> {
        self.realized_funding_fee_in_usd =
            self.realized_funding_fee_in_usd.safe_sub(realized_funding_fee_in_usd)?;
        Ok(())
    }

    pub fn sub_close_fee_usd(&mut self, close_fee_in_usd: u128) -> BumpResult<()> {
        self.close_fee_in_usd = self.close_fee_in_usd.safe_sub(close_fee_in_usd)?;
        Ok(())
    }

    pub fn set_last_update(&mut self, last_update: i64) -> BumpResult {
        self.last_update_time = last_update;
        Ok(())
    }

    pub fn set_position_key(&mut self, position_key: Pubkey) -> BumpResult {
        self.position_key = position_key;
        Ok(())
    }

    pub fn set_user_key(&mut self, authority: Pubkey) -> BumpResult {
        self.user_key = authority;
        Ok(())
    }

    pub fn set_index_mint(&mut self, index_mint: Pubkey) -> BumpResult {
        self.index_mint_key = index_mint;
        Ok(())
    }

    pub fn set_symbol(&mut self, symbol: [u8; 32]) -> BumpResult {
        self.symbol = symbol;
        Ok(())
    }

    pub fn set_margin_mint(&mut self, margin_mint: Pubkey) -> BumpResult {
        self.margin_mint_key = margin_mint;
        Ok(())
    }

    pub fn set_is_long(&mut self, is_long: bool) -> BumpResult {
        self.is_long = is_long;
        Ok(())
    }

    pub fn set_cross_margin(&mut self, cross_margin: bool) -> BumpResult {
        self.cross_margin = cross_margin;
        Ok(())
    }

    pub fn set_status(&mut self, status: PositionStatus) -> BumpResult {
        self.status = status;
        Ok(())
    }

    pub fn add_position_portfolio_balance(&mut self, amount: u128) -> BumpResult<u128> {
        if self.initial_margin_usd == self.initial_margin_usd_from_portfolio {
            Ok(0)
        } else {
            let borrow_margin = self
                .initial_margin_usd
                .safe_sub(self.initial_margin_usd_from_portfolio)?
                .safe_mul(self.initial_margin)?
                .safe_div(self.initial_margin_usd)?;

            let add_initial_amount = amount.min(borrow_margin);

            let add_initial_amount_usd = add_initial_amount
                .safe_mul(self.initial_margin_usd)?
                .safe_div(self.initial_margin)?;

            self.initial_margin_usd_from_portfolio =
                self.initial_margin_usd_from_portfolio.safe_add(add_initial_amount_usd)?;
            Ok(add_initial_amount)
        }
    }

    pub fn reduce_position_portfolio_balance(&mut self, amount: u128) -> BumpResult<u128> {
        let reduce_initial_margin_usd = amount
            .safe_mul(self.initial_margin_usd.cast()?)?
            .safe_div(self.initial_margin.cast()?)?;
        if self.initial_margin_usd_from_portfolio <= reduce_initial_margin_usd {
            self.initial_margin_usd_from_portfolio = 0;
            Ok(self
                .initial_margin_usd_from_portfolio
                .safe_mul(self.initial_margin)?
                .safe_div(self.initial_margin_usd)?)
        } else {
            self.initial_margin_usd_from_portfolio =
                self.initial_margin_usd_from_portfolio.safe_sub(reduce_initial_margin_usd)?;
            Ok(amount)
        }
    }

    pub fn get_position_un_pnl_usd(&self, index_price: u128) -> BumpResult<i128> {
        if self.position_size == 0u128 {
            return Ok(0i128);
        };
        if self.is_long {
            Ok(self
                .position_size
                .cast::<i128>()?
                .safe_mul(index_price.cast::<i128>()?.safe_sub(self.entry_price.cast::<i128>()?)?)?
                .safe_div(self.entry_price.cast::<i128>()?)?)
        } else {
            Ok(self
                .position_size
                .cast::<i128>()?
                .safe_mul(self.entry_price.cast::<i128>()?.safe_sub(index_price.cast::<i128>()?)?)?
                .safe_div(self.entry_price.cast::<i128>()?)?)
        }
    }

    pub fn get_position_mm(&self, market: &Market, state: &State) -> BumpResult<u128> {
        Ok(cal_utils::get_mm(
            self.position_size,
            market.market_trade_config.max_leverage,
            state.maximum_maintenance_margin_rate,
        )?)
    }

    pub fn get_position_un_pnl_token(
        &self,
        trade_token: &TradeToken,
        mint_token_price: u128,
        index_price: u128,
    ) -> BumpResult<i128> {
        if self.position_size == 0u128 {
            return Ok(0i128);
        };
        let un_pnl_usd = self.get_position_un_pnl_usd(index_price)?;
        Ok(cal_utils::usd_to_token_i(un_pnl_usd, trade_token.decimals, mint_token_price)?)
    }

    pub fn get_position_fee(
        &self,
        market: &Market,
        pool: &Pool,
        margin_mint_price: u128,
        trade_token_decimals: u16,
    ) -> BumpResult<i128> {
        let mut funding_fee_total_usd = self.realized_funding_fee_in_usd;
        let mut borrowing_fee_total_usd = self.realized_borrowing_fee_in_usd;

        let funding_fee_amount_per_size = if self.is_long {
            market.funding_fee.long_funding_fee_amount_per_size
        } else {
            market.funding_fee.short_funding_fee_amount_per_size
        };
        let funding_fee = cal_utils::mul_small_rate_i(
            self.position_size.cast::<i128>()?,
            funding_fee_amount_per_size.safe_sub(self.open_funding_fee_amount_per_size)?,
        )?;

        if self.is_long {
            let funding_fee_usd =
                cal_utils::token_to_usd_i(funding_fee, trade_token_decimals, margin_mint_price)?;
            funding_fee_total_usd = funding_fee_total_usd.safe_add(funding_fee_usd)?;
        } else {
            funding_fee_total_usd = funding_fee_total_usd.safe_add(funding_fee)?;
        }

        let initial_margin_leverage = cal_utils::mul_small_rate_u(
            self.initial_margin,
            (self.leverage as u128).safe_sub(RATE_PRECISION)?, //TODO: why sub?
        )?;
        let borrowing_fee = cal_utils::mul_small_rate_u(
            pool.borrowing_fee
                .cumulative_borrowing_fee_per_token
                .safe_sub(self.open_borrowing_fee_per_token)?,
            initial_margin_leverage,
        )?;
        borrowing_fee_total_usd = borrowing_fee_total_usd.safe_add(cal_utils::token_to_usd_u(
            borrowing_fee,
            trade_token_decimals,
            margin_mint_price,
        )?)?;
        Ok(funding_fee_total_usd
            .safe_add(borrowing_fee_total_usd.cast()?)?
            .safe_add(self.close_fee_in_usd.cast()?)?)
    }

    pub fn get_position_value(
        &self,
        index_trade_token: &TradeToken,
        oracle_map: &mut OracleMap,
    ) -> BumpResult<(u128, i128, u128)> {
        if self.cross_margin {
            let index_price_data = oracle_map.get_price_data(&index_trade_token.oracle_key)?;

            let position_un_pnl = self.get_position_un_pnl_usd(index_price_data.price)?;

            Ok((self.initial_margin_usd_from_portfolio, position_un_pnl, self.mm_usd))
        } else {
            Ok((0u128, 0i128, 0u128))
        }
    }

    pub fn get_liquidation_price(
        &self,
        market: &Market,
        pool: &Pool,
        state: &State,
        margin_token_price: u128,
        margin_token_decimals: u16,
    ) -> BumpResult<u128> {
        let mm_usd = self.get_position_mm(market, state)?;
        let position_fee_usd =
            self.get_position_fee(market, pool, margin_token_price, margin_token_decimals)?;
        let position_value = if self.is_long {
            position_fee_usd.safe_add(
                self.position_size.safe_sub(self.initial_margin_usd)?.safe_add(mm_usd)?.cast()?,
            )?
        } else {
            self.position_size
                .safe_add(self.initial_margin_usd)?
                .safe_sub(mm_usd)?
                .cast::<i128>()?
                .safe_sub(position_fee_usd)?
        };
        if position_value < 0 {
            Ok(0)
        } else {
            let liquidation_price = position_value
                .cast::<u128>()?
                .safe_mul(self.entry_price)?
                .safe_div(self.position_size)?;
            Ok(liquidation_price)
        }
    }
}
