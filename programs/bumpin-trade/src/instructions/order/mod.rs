pub mod cancel;
pub mod place;

use anchor_lang::prelude::*;
pub use cancel::*;
pub use place::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct CancelOrderParams {
    pub pool_index: u16,
    pub order_id: u64,
}
