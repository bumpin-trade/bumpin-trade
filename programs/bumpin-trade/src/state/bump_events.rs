use anchor_lang::event;
use anchor_lang::prelude::*;

use crate::instructions::DepositOrigin;
use crate::state::infrastructure::fee_reward::FeeReward;
use crate::state::infrastructure::pool_borrowing_fee::BorrowingFee;
use crate::state::infrastructure::user_order::UserOrder;
use crate::state::infrastructure::user_position::UserPosition;
use crate::state::infrastructure::user_stake::{UserRewards, UserStake};
use crate::state::infrastructure::user_token::UserToken;
use crate::state::pool::PoolBalance;
use crate::state::user::UserTokenUpdateReason;

#[event]
pub struct InitUserEvent {
    pub user_key: Pubkey,
    pub authority: Pubkey,
}

#[event]
pub struct DepositEvent {
    pub user_key: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u128,
    pub deposit_origin: DepositOrigin,
}

#[event]
pub struct WithdrawEvent {
    pub user_key: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u128,
}

#[event]
pub struct StakeOrUnStakeEvent {
    pub user_key: Pubkey,
    pub token_mint: Pubkey,
    pub change_supply_amount: u128,
    pub user_stake: UserStake,
}

#[event]
pub struct UserRewardsUpdateEvent {
    pub user_key: Pubkey,
    pub token_mint: Pubkey,
    pub user_rewards: UserRewards,
}

#[event]
pub struct UserTokenBalanceUpdateEvent {
    pub user_key: Pubkey,
    pub token_mint: Pubkey,
    pub pre_user_token: UserToken,
    pub user_token: UserToken,
    pub update_origin: UserTokenUpdateReason,
}

#[event]
pub struct UserHoldUpdateEvent {
    pub user_key: Pubkey,
    pub pre_hold_amount: u128,
    pub hold_amount: u128,
}

#[event]
pub struct AddOrDeleteUserOrderEvent {
    pub user_key: Pubkey,
    pub order: UserOrder,
    pub is_add: bool,
}

#[event]
pub struct UpdateUserPositionEvent {
    pub pre_position: UserPosition,
    pub position: UserPosition,
}

#[event]
pub struct AddOrDeleteUserPositionEvent {
    pub position: UserPosition,
    pub is_add: bool,
}

#[event]
pub struct AddOrDecreaseMarginEvent {
    pub user_key: Pubkey,
    pub position: UserPosition,
    pub pre_position: UserPosition,
    pub is_add: bool,
}

#[event]
pub struct PoolUpdateEvent {
    pub pool_key: Pubkey,
    pub pool_mint: Pubkey,
    pub pool_index: u16,

    //current info
    pub pool_balance: PoolBalance,
    pub borrowing_fee: BorrowingFee,
    pub fee_reward: FeeReward,
    pub total_supply: u128,
    pub pnl: i128,
    pub apr: u128,
    pub insurance_fund_amount: u128,

    //pre info
    pub pre_pool_balance: PoolBalance,
    pub pre_borrowing_fee: BorrowingFee,
    pub pre_fee_reward: FeeReward,
    pub pre_total_supply: u128,
    pub pre_pnl: i128,
    pub pre_apr: u128,
    pub pre_insurance_fund_amount: u128,
}
