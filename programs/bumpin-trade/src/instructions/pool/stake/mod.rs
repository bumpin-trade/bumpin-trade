use anchor_lang::prelude::*;

pub mod portfolio;
pub mod wallet;

pub use portfolio::*;
pub use wallet::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct StakeParams {
    pub request_token_amount: u128,
    pub pool_index: u16,
    pub trade_token_index: u16,
}
