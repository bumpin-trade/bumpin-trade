use anchor_lang::accounts::account_loader::AccountLoader;
use anchor_lang::accounts::signer::Signer;

use crate::state::user::User;

pub fn can_sign_for_user(user: &AccountLoader<User>, signer: &Signer) -> anchor_lang::Result<bool> {
    user.load().map(|user| {
        user.authority.eq(signer.key)
    })
}


