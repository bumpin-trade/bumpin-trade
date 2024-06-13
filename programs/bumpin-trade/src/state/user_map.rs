use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::panic::Location;
use std::slice::Iter;

use anchor_lang::Discriminator;
use anchor_lang::prelude::AccountLoader;
use arrayref::array_ref;
use solana_program::account_info::AccountInfo;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode::{CouldNotLoadUserData, UserNotFound};
use crate::errors::BumpResult;
use crate::math::safe_unwrap::SafeUnwrap;
use crate::state::traits::Size;
use crate::state::user::User;

pub struct UserMap<'a>(pub BTreeMap<Pubkey, AccountLoader<'a, User>>);

impl<'a> UserMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_ref(&self, user_key: &Pubkey) -> BumpResult<Ref<User>> {
        let loader = match self.0.get(&user_key) {
            None => {
                let caller = Location::caller();
                msg!(
                    "Could not find user {} at {}:{}",
                    user_key,
                    caller.file(),
                    caller.line()
                );
                return Err(UserNotFound);
            }
            Some(loader) => loader
        };
        match loader.load() {
            Ok(user) => Ok(user),
            Err(e) => {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!(
                    "Could not load pool {} at {}:{}",
                    user_key,
                    caller.file(),
                    caller.line()
                );
                Err(CouldNotLoadUserData)
            }
        }
    }


    #[track_caller]
    #[inline(always)]
    pub fn get_account_loader(&self, user_key: &Pubkey) -> BumpResult<&'a AccountLoader<User>> {
        let loader = match self.0.get(&user_key) {
            None => {
                let caller = Location::caller();
                msg!(
                    "Could not find user {} at {}:{}",
                    user_key,
                    caller.file(),
                    caller.line()
                );
                return Err(UserNotFound);
            }
            Some(loader) => loader
        };
        Ok(loader)
    }


    #[track_caller]
    #[inline(always)]
    pub fn get_mut_ref(&self, user_key: &Pubkey) -> BumpResult<RefMut<User>> {
        let loader = match self.0.get(&user_key) {
            None => {
                let caller = Location::caller();
                msg!(
                    "Could not find user {} at {}:{}",
                    user_key,
                    caller.file(),
                    caller.line()
                );
                return Err(UserNotFound);
            }
            Some(loader) => loader
        };
        match loader.load_mut() {
            Ok(user) => Ok(user),
            Err(e) => {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!(
                    "Could not load pool {} at {}:{}",
                    user_key,
                    caller.file(),
                    caller.line()
                );
                Err(CouldNotLoadUserData)
            }
        }
    }
    pub fn load(account_info_iter: &mut Peekable<Iter<'a, AccountInfo<'a>>>) -> BumpResult<UserMap<'a>> {
        let mut user_map = UserMap(BTreeMap::new());
        let user_discriminator = User::discriminator();
        while let Some(account_info) = account_info_iter.peek() {
            let data = account_info
                .try_borrow_data()
                .or(Err(CouldNotLoadUserData))?;

            let expected_data_len = User::SIZE;
            if data.len() < expected_data_len {
                break;
            }
            let account_discriminator = array_ref![data, 0, 8];
            if account_discriminator != &user_discriminator {
                continue;
            }

            let user_key = Pubkey::from(*array_ref![data, 8, 32]);
            let account_info = account_info_iter.next().safe_unwrap()?;
            let account_loader: AccountLoader<'a, User> = AccountLoader::try_from(account_info).or(Err(CouldNotLoadUserData))?;

            user_map.0.insert(user_key, account_loader);
        }
        Ok(user_map)
    }
}