use anchor_lang::event;
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

use crate::instructions::DepositOrigin;
use crate::state::infrastructure::user_stake::{UserRewards, UserStake};
use crate::state::infrastructure::user_token::UserToken;
use crate::state::user::UserTokenUpdateOrigin;

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
    pub change_stake_amount: u128,
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
    pub update_origin: UserTokenUpdateOrigin,
}
