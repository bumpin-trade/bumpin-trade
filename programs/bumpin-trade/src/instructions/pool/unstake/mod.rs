use anchor_lang::prelude::*;

pub mod wallet;

pub use wallet::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UnStakeParams {
    pub share: u128,
    pub pool_index: u16,
    pub trade_token_index: u16,
}
