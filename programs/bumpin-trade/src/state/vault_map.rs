use std::collections::BTreeMap;
use std::panic::Location;

use anchor_lang::prelude::Account;
use anchor_lang::prelude::*;
use anchor_lang::Key;
use anchor_spl::token;
use anchor_spl::token::TokenAccount;

use crate::errors::BumpErrorCode::{
    CouldNotLoadTradeTokenData, InvalidTradeTokenAccount, TradeTokenNotFind,
};
use crate::errors::BumpResult;

pub struct VaultMap<'a>(pub BTreeMap<Pubkey, Account<'a, TokenAccount>>);

impl<'a> VaultMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_account(&self, account_key: &Pubkey) -> BumpResult<&Account<'a, TokenAccount>> {
        let account = match self.0.get(account_key) {
            None => {
                let caller = Location::caller();
                msg!(
                    "Could not find trade_token {} at {}:{}",
                    account_key,
                    caller.file(),
                    caller.line()
                );
                return Err(TradeTokenNotFind);
            },
            Some(loader) => loader,
        };
        Ok(account)
    }

    pub fn load(remaining_accounts: &'a [AccountInfo<'a>]) -> BumpResult<VaultMap<'a>> {
        let mut token_account_map: VaultMap = VaultMap(BTreeMap::new());
        for account_info in remaining_accounts.iter() {
            let data = account_info.try_borrow_data().or(Err(CouldNotLoadTradeTokenData))?;

            let expected_data_len = TokenAccount::LEN;
            if data.len() < expected_data_len {
                continue;
            }

            if account_info.owner != &token::ID {
                continue;
            }

            let account: Account<'a, TokenAccount> =
                Account::try_from(account_info).or(Err(InvalidTradeTokenAccount))?;

            token_account_map.0.insert(account.key(), account);
        }
        Ok(token_account_map)
    }

    pub fn load_vec(
        remaining_accounts: &'a [AccountInfo<'a>],
    ) -> BumpResult<Vec<Account<'a, TokenAccount>>> {
        let mut token_account_vec: Vec<Account<'a, TokenAccount>> = Vec::new();
        let mut index = 0usize;
        for account_info in remaining_accounts.iter() {
            let data = account_info.try_borrow_data().or(Err(CouldNotLoadTradeTokenData))?;

            let expected_data_len = TokenAccount::LEN;
            if data.len() < expected_data_len {
                continue;
            }

            if account_info.owner != &token::ID {
                continue;
            }

            let account: Account<'a, TokenAccount> =
                Account::try_from(account_info).or(Err(InvalidTradeTokenAccount))?;

            token_account_vec.insert(index, account);
            index += 1usize;
        }
        Ok(token_account_vec)
    }
}
