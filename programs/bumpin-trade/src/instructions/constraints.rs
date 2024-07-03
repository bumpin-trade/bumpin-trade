use anchor_lang::accounts::account_loader::AccountLoader;
use anchor_lang::accounts::signer::Signer;

use crate::state::user::{User, UserStatus};

pub fn can_sign_for_user(user: &AccountLoader<User>, signer: &Signer) -> anchor_lang::Result<bool> {
    user.load().map(|user| user.authority.eq(signer.key))
}

pub fn is_normal(user: &AccountLoader<User>) -> anchor_lang::Result<bool> {
    user.load().map(|user| user.user_status.eq(&UserStatus::NORMAL))
}
