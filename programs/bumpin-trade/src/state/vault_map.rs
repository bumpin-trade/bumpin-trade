use std::collections::BTreeMap;
use std::iter::Peekable;
use std::ops::Deref;
use std::panic::Location;
use std::slice::Iter;
use anchor_lang::{Discriminator, Key};
use anchor_lang::prelude::Account;
use anchor_spl::token;
use anchor_spl::token::TokenAccount;

use arrayref::array_ref;
use solana_program::account_info::AccountInfo;
use solana_program::msg;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode::{
    CouldNotLoadTradeTokenData, InvalidTradeTokenAccount, TradeTokenNotFind,
};
use crate::errors::BumpResult;
use crate::math::safe_unwrap::SafeUnwrap;


pub struct VaultMap<'a>(pub BTreeMap<Pubkey, Account<'a, TokenAccount>>);

impl<'a> VaultMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_account(&self, mint: &Pubkey) -> BumpResult<&Account<'a, TokenAccount>> {
        let account = match self.0.get(mint) {
            None => {
                let caller = Location::caller();
                msg!("Could not find trade_token {} at {}:{}", mint, caller.file(), caller.line());
                return Err(TradeTokenNotFind);
            }
            Some(loader) => loader,
        };
        Ok(account)
    }

    pub fn load<'c>(
        account_info_iter: &'c mut Peekable<Iter<'a, AccountInfo<'a>>>,
    ) -> BumpResult<VaultMap<'a>> {
        let mut token_account_map: VaultMap = VaultMap(BTreeMap::new());
        while let Some(account_info) = account_info_iter.next() {
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
}
