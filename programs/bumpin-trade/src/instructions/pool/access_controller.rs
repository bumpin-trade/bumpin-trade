use anchor_lang::prelude::AccountLoader;

use crate::errors::BumpErrorCode::StakePaused;
use crate::errors::BumpResult;
use crate::state::pool::{Pool, PoolStatus};

pub fn pool_stake_not_paused(stake_pool: &AccountLoader<Pool>) -> BumpResult<()> {
    if stake_pool.load()?.pool_status.eq(&PoolStatus::StakePaused)
    {
        return Err(StakePaused);
    }
    Ok(())
}