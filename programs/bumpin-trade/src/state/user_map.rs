use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;
use std::panic::Location;

use anchor_lang::prelude::AccountLoader;
use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use arrayref::array_ref;
use solana_program::msg;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::state::traits::Size;
use crate::state::user::User;
use crate::utils::pda;

pub struct UserMap<'a>(pub BTreeMap<Pubkey, AccountLoader<'a, User>>);

impl<'a> UserMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_ref(&self, user_key: &Pubkey) -> BumpResult<Ref<User>> {
        let loader = match self.0.get(&user_key) {
            None => {
                let caller = Location::caller();
                msg!("Could not find user {} at {}:{}", user_key, caller.file(), caller.line());
                return Err(BumpErrorCode::UserNotFound);
            },
            Some(loader) => loader,
        };
        match loader.load() {
            Ok(user) => Ok(user),
            Err(e) => {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!("Could not load pool {} at {}:{}", user_key, caller.file(), caller.line());
                Err(BumpErrorCode::CouldNotLoadUserData)
            },
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_account_loader(&self, user_key: &Pubkey) -> BumpResult<AccountLoader<'a, User>> {
        let loader = match self.0.get(&user_key) {
            None => {
                let caller = Location::caller();
                msg!("Could not find user {} at {}:{}", user_key, caller.file(), caller.line());
                return Err(BumpErrorCode::UserNotFound);
            },
            Some(loader) => loader.clone(),
        };
        Ok(loader)
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_mut_ref(&self, user_key: &Pubkey) -> BumpResult<RefMut<User>> {
        let loader = match self.0.get(&user_key) {
            None => {
                let caller = Location::caller();
                msg!("Could not find user {} at {}:{}", user_key, caller.file(), caller.line());
                return Err(BumpErrorCode::UserNotFound);
            },
            Some(loader) => loader,
        };
        match loader.load_mut() {
            Ok(user) => Ok(user),
            Err(e) => {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!("Could not load pool {} at {}:{}", user_key, caller.file(), caller.line());
                Err(BumpErrorCode::CouldNotLoadUserData)
            },
        }
    }
    pub fn load(
        remaining_accounts: &'a [AccountInfo<'a>],
        program_id: &Pubkey,
    ) -> BumpResult<UserMap<'a>> {
        let mut user_map = UserMap(BTreeMap::new());
        let user_discriminator = User::discriminator();
        for account_info in remaining_accounts.iter() {
            let user_account_pda = pda::generate_user_pda(account_info.owner, program_id)?;
            if !account_info.key.eq(&user_account_pda) {
                continue;
            }

            let data =
                account_info.try_borrow_data().or(Err(BumpErrorCode::CouldNotLoadUserData))?;

            let expected_data_len = User::SIZE;
            if data.len() < expected_data_len {
                break;
            }
            let account_discriminator = array_ref![data, 0, 8];
            if account_discriminator != &user_discriminator {
                continue;
            }

            let user_key = Pubkey::from(*array_ref![data, 8, 32]);
            let account_loader: AccountLoader<'a, User> = AccountLoader::try_from(account_info)
                .or(Err(BumpErrorCode::CouldNotLoadUserData))?;

            user_map.0.insert(user_key, account_loader);
        }
        Ok(user_map)
    }
}
