pub mod cancel_perp_order;
pub mod place_perp_order;

use anchor_lang::prelude::*;
pub use cancel_perp_order::*;
pub use place_perp_order::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct CancelOrderParams {
    pub pool_index: u16,
    pub order_id: u64,
}
