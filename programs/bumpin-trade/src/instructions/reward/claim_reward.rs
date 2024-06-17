use crate::state::state::State;
use crate::state::user::User;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use solana_program::account_info::AccountInfo;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Account<'info, State>,

    pub user: AccountLoader<'info, User>,

    /// CHECK: ?
    pub bump_signer: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let user = ctx.accounts.user.load()?;

    Ok(())
}
