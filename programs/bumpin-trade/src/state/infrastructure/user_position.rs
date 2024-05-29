use anchor_lang::zero_copy;
use solana_program::pubkey::Pubkey;
use crate::errors::BumpResult;
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;

#[zero_copy(unsafe)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct UserPosition {
    pub position_key: Pubkey,
    pub symbol: [u8; 32],
    pub is_long: bool,
    pub cross_margin: bool,
    pub authority: Pubkey,
    pub margin_mint: Pubkey,
    pub index_mint: Pubkey,
    pub position_size: u128,
    pub entry_price: u128,
    pub leverage: u128,
    pub initial_margin: u128,
    pub initial_margin_usd: u128,
    pub initial_margin_usd_from_portfolio: u128,
    pub mm_usd: u128,
    pub hold_pool_amount: u128,
    pub open_fee_in_usd: u128,
    pub realized_borrowing_fee: u128,
    pub realized_borrowing_fee_in_usd: u128,
    pub open_borrowing_fee_per_token: u128,
    pub realized_funding_fee: i128,
    pub realized_funding_fee_in_usd: i128,
    pub open_funding_fee_amount_per_size: i128,
    pub close_fee_in_usd: u128,
    pub last_update_time: u128,
    pub realized_pnl: i128,
    pub status: PositionStatus,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum PositionStatus {
    #[default]
    INIT,
    USING,
}

impl UserPosition {
    pub fn add_position_size(&mut self, position_size: u128) {
        self.position_size = self.position_size.safe_add(position_size)?;
    }

    pub fn sub_position_size(&mut self, position_size: u128) {
        self.position_size = self.position_size.safe_sub(position_size)?;
    }

    pub fn set_entry_price(&mut self, entry_price: u128) {
        self.entry_price = entry_price;
    }

    pub fn set_initial_margin(&mut self, initial_margin: u128) {
        self.initial_margin = initial_margin;
    }

    pub fn set_initial_margin_usd(&mut self, initial_margin_usd: u128) {
        self.initial_margin_usd = initial_margin_usd;
    }

    pub fn set_leverage(&mut self, leverage: u128) {
        self.leverage = leverage;
    }

    pub fn set_realized_pnl(&mut self, realized_pnl: i128) {
        self.realized_pnl = realized_pnl;
    }

    pub fn set_open_borrowing_fee_per_token(&mut self, open_borrowing_fee_per_token: u128) {
        self.open_borrowing_fee_per_token = open_borrowing_fee_per_token;
    }

    pub fn set_open_funding_fee_amount_per_size(&mut self, open_funding_fee_amount_per_size: i128) {
        self.open_funding_fee_amount_per_size = open_funding_fee_amount_per_size;
    }

    pub fn add_initial_margin(&mut self, initial_margin: u128) {
        self.initial_margin = self.initial_margin.safe_add(initial_margin)?;
    }

    pub fn add_realized_borrowing_fee(&mut self, realized_borrowing_fee: u128) {
        self.realized_borrowing_fee = self.realized_borrowing_fee.safe_add(realized_borrowing_fee)?;
    }

    pub fn add_realized_funding_fee(&mut self, realized_funding_fee: i128) {
        self.realized_funding_fee = self.realized_funding_fee.safe_add(realized_funding_fee)?;
    }

    pub fn add_realized_funding_fee_in_usd(&mut self, realized_funding_fee_in_usd: i128) {
        self.realized_funding_fee_in_usd = self.realized_funding_fee_in_usd.safe_add(realized_funding_fee_in_usd)?;
    }

    pub fn add_realized_borrowing_fee_in_usd(&mut self, realized_borrowing_fee_in_usd: u128) {
        self.realized_borrowing_fee_in_usd = self.realized_borrowing_fee_in_usd.safe_add(realized_borrowing_fee_in_usd)?;
    }

    pub fn sub_initial_margin(&mut self, initial_margin: u128) {
        self.initial_margin = self.initial_margin.safe_sub(initial_margin)?;
    }

    pub fn add_initial_margin_usd(&mut self, initial_margin_usd: u128) {
        self.initial_margin_usd = self.initial_margin_usd.safe_add(initial_margin_usd)?;
    }

    pub fn sub_initial_margin_usd(&mut self, initial_margin_usd: u128) {
        self.initial_margin_usd = self.initial_margin_usd.safe_sub(initial_margin_usd)?;
    }

    pub fn add_initial_margin_usd_from_portfolio(&mut self, initial_margin_usd_from_portfolio: u128) {
        self.initial_margin_usd_from_portfolio = self.initial_margin_usd_from_portfolio.safe_add(initial_margin_usd_from_portfolio)?;
    }

    pub fn sub_initial_margin_usd_from_portfolio(&mut self, initial_margin_usd_from_portfolio: u128) {
        self.initial_margin_usd_from_portfolio = self.initial_margin_usd_from_portfolio.safe_sub(initial_margin_usd_from_portfolio)?;
    }

    pub fn set_initial_margin_usd_from_portfolio(&mut self, initial_margin_usd_from_portfolio: u128) {
        self.initial_margin_usd_from_portfolio = initial_margin_usd_from_portfolio;
    }

    pub fn set_close_fee_in_usd(&mut self, close_fee_in_usd: u128) {
        self.close_fee_in_usd = close_fee_in_usd;
    }

    pub fn set_position_size(&mut self, position_size: u128) {
        self.position_size = position_size;
    }

    pub fn add_hold_pool_amount(&mut self, hold_pool_amount: u128) {
        self.hold_pool_amount = self.hold_pool_amount.safe_add(hold_pool_amount)?;
    }

    pub fn sub_hold_pool_amount(&mut self, hold_pool_amount: u128) {
        self.hold_pool_amount = self.hold_pool_amount.safe_add(hold_pool_amount)?;
    }

    pub fn add_realized_pnl(&mut self, realized_pnl: i128) {
        self.realized_pnl = self.realized_pnl.cast::<i128>()?.safe_add(realized_pnl.cast::<i128>()?)?.cast::<i128>()?;
    }

    pub fn sub_realized_borrowing_fee(&mut self, realized_borrowing_fee: u128) {
        self.realized_borrowing_fee = self.realized_borrowing_fee.safe_sub(realized_borrowing_fee)?;
    }

    pub fn sub_realized_borrowing_fee_usd(&mut self, realized_borrowing_fee_in_usd: u128) {
        self.realized_borrowing_fee_in_usd = self.realized_borrowing_fee_in_usd.safe_sub(realized_borrowing_fee_in_usd)?;
    }

    pub fn sub_realized_funding_fee(&mut self, realized_funding_fee: i128) {
        self.realized_funding_fee = self.realized_funding_fee.cast::<i128>()?.safe_sub(realized_funding_fee.cast::<i128>()?)?.cast::<i128>()?;
    }

    pub fn sub_realized_funding_fee_usd(&mut self, realized_funding_fee_in_usd: i128) {
        self.realized_funding_fee_in_usd = self.realized_funding_fee_in_usd.cast::<i128>()?.safe_sub(realized_funding_fee_in_usd.cast::<i128>()?)?.cast::<i128>()?;
    }

    pub fn sub_close_fee_usd(&mut self, close_fee_in_usd: u128) {
        self.close_fee_in_usd = self.close_fee_in_usd.safe_sub(close_fee_in_usd)?;
    }

    pub fn set_last_update(&mut self, last_update: u128) {
        self.last_update_time = last_update;
    }

    pub fn set_position_key(&mut self, position_key: Pubkey) {
        self.position_key = position_key;
    }

    pub fn set_authority(&mut self, authority: Pubkey) {
        self.authority = authority;
    }

    pub fn set_index_mint(&mut self, index_mint: Pubkey) {
        self.index_mint = index_mint;
    }

    pub fn set_symbol(&mut self, symbol: [u8; 32]) {
        self.symbol = symbol;
    }

    pub fn set_margin_mint(&mut self, margin_mint: Pubkey) {
        self.margin_mint = margin_mint;
    }

    pub fn set_is_long(&mut self, is_long: bool) {
        self.is_long = is_long;
    }

    pub fn set_cross_margin(&mut self, cross_margin: bool) {
        self.cross_margin = cross_margin;
    }

    pub fn set_status(&mut self, status: PositionStatus) {
        self.status = status;
    }

    pub fn add_position_portfolio_balance(&mut self, amount: u128) -> BumpResult<u128> {
        if self.initial_margin_usd == self.initial_margin_usd_from_portfolio {
            Ok(0)
        } else {
            let borrow_margin = self.initial_margin_usd
                .safe_sub(self.initial_margin_usd_from_portfolio)?
                .safe_mul(self.initial_margin)?
                .safe_div(self.initial_margin_usd)?;

            let add_initial_amount = amount.min(borrow_margin)?;

            let add_initial_amount_usd = add_initial_amount
                .safe_mul(self.initial_margin_usd)?
                .safe_div(self.initial_margin)?;

            self.initial_margin_usd_from_portfolio = self.initial_margin_usd_from_portfolio.safe_add(add_initial_amount_usd)?;
            Ok(add_initial_amount)
        }
    }

    pub fn reduce_position_portfolio_balance(&mut self, amount: u128) -> BumpResult<u128> {
        let reduce_initial_margin_usd = amount
            .safe_mul(self.initial_margin_usd.cast()?)?
            .safe_div(self.initial_margin.cast()?)?;
        if self.initial_margin_usd_from_portfolio <= reduce_initial_margin_usd {
            self.initial_margin_usd_from_portfolio = 0;
            Ok(self.initial_margin_usd_from_portfolio
                .safe_mul(self.initial_margin)?
                .safe_div(self.initial_margin_usd)?
            )
        } else {
            self.initial_margin_usd_from_portfolio = self.initial_margin_usd_from_portfolio.safe_sub(reduce_initial_margin_usd)?;
            Ok(amount)
        }
    }

    fn get_percent_initial_margin_usd(&self, amount: u128) -> BumpResult<u128> {
        Ok()
    }
}