use crate::errors::BumpErrorCode;
use crate::state::state::State;
use crate::state::user::{User, UserStatus};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    _user_authority_key: Pubkey
)]
pub struct UpdateUserStatus<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        has_one = keeper_signer,
        bump,
    )]
    pub state: Account<'info, State>,

    #[account(
        seeds = [b"user", _user_authority_key.as_ref()],
        bump,
    )]
    pub user: AccountLoader<'info, User>,

    pub keeper_signer: Signer<'info>,
}

pub fn handle_update_user_status(
    ctx: Context<UpdateUserStatus>,
    user_status: UserStatus,
    _user_authority_key: Pubkey,
) -> Result<()> {
    let mut user =
        ctx.accounts.user.load_mut().map_err(|_e| BumpErrorCode::CouldNotLoadUserData)?;
    user.user_status = user_status;
    Ok(())
}
