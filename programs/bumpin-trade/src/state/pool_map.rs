use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;
use std::panic::Location;

use anchor_lang::prelude::AccountLoader;
use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use arrayref::array_ref;

use crate::errors::BumpErrorCode::CouldNotLoadPoolData;
use crate::errors::BumpResult;
use crate::state::pool::Pool;
use crate::traits::Size;

pub struct PoolMap<'a>(pub BTreeMap<Pubkey, AccountLoader<'a, Pool>>);

impl<'a> PoolMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_all_pool_loader(&self) -> BumpResult<Vec<&AccountLoader<'a, Pool>>> {
        let mut pool_vec = Vec::new();
        for pool_loader in self.0.values() {
            pool_vec.push(pool_loader);
        }
        Ok(pool_vec)
    }
    #[track_caller]
    #[inline(always)]
    pub fn get_ref(&self, pool_key: &Pubkey) -> BumpResult<Ref<Pool>> {
        let loader = match self.0.get(&pool_key) {
            None => {
                let caller = Location::caller();
                msg!("Could not find pool {} at {}:{}", pool_key, caller.file(), caller.line());
                return Err(CouldNotLoadPoolData);
            },
            Some(loader) => loader,
        };
        match loader.load() {
            Ok(pool) => Ok(pool),
            Err(e) => {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!("Could not load pool {} at {}:{}", pool_key, caller.file(), caller.line());
                Err(CouldNotLoadPoolData)
            },
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_mut_ref(&self, pool_key: &Pubkey) -> BumpResult<RefMut<Pool>> {
        let loader = match self.0.get(&pool_key) {
            None => {
                let caller = Location::caller();
                msg!("Could not find pool {} at {}:{}", pool_key, caller.file(), caller.line());
                return Err(CouldNotLoadPoolData);
            },
            Some(loader) => loader,
        };
        match loader.load_mut() {
            Ok(pool) => Ok(pool),
            Err(e) => {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!("Could not load pool {} at {}:{}", pool_key, caller.file(), caller.line());
                Err(CouldNotLoadPoolData)
            },
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_account_loader(&self, pool_key: &Pubkey) -> BumpResult<&AccountLoader<'a, Pool>> {
        let loader = match self.0.get(&pool_key) {
            None => {
                let caller = Location::caller();
                msg!("Could not find pool {} at {}:{}", pool_key, caller.file(), caller.line());
                return Err(CouldNotLoadPoolData);
            },
            Some(loader) => loader,
        };
        Ok(loader)
    }

    pub fn load(remaining_accounts: &'a [AccountInfo<'a>]) -> BumpResult<PoolMap<'a>> {
        let mut pool_map = PoolMap(BTreeMap::new());
        let pool_discriminator = Pool::discriminator();
        for account_info in remaining_accounts.iter() {
            if !account_info.owner.eq(&crate::id()) {
                continue;
            }

            if let Ok(data) = account_info.try_borrow_data() {
                let expected_data_len = Pool::SIZE;
                if data.len() < expected_data_len {
                    continue;
                }
                let account_discriminator = array_ref![data, 0, 8];
                if account_discriminator != &pool_discriminator {
                    continue;
                }

                let pool_key = Pubkey::from(*array_ref![data, 8, 32]);
                let account_loader: AccountLoader<'a, Pool> =
                    AccountLoader::try_from(account_info).or(Err(CouldNotLoadPoolData))?;

                pool_map.0.insert(pool_key, account_loader);
            }
        }
        Ok(pool_map)
    }
}
