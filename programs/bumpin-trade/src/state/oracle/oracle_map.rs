use std::collections::BTreeMap;
use std::iter::Peekable;
use std::slice::Iter;

use anchor_lang::Key;
use solana_program::account_info::AccountInfo;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode::OracleNotFound;
use crate::errors::BumpResult;
use crate::ids::pyth_program;
use crate::math::safe_unwrap::SafeUnwrap;
use crate::state::oracle::oracle::{get_oracle_price, OraclePriceData};

pub struct AccountInfoAndOracleSource<'a> {
    /// CHECK: ownders are validated in OracleMap::load
    pub account_info: AccountInfo<'a>,
}

pub struct OracleMap<'a> {
    oracles: BTreeMap<Pubkey, AccountInfoAndOracleSource<'a>>,
    price_data: BTreeMap<Pubkey, OraclePriceData>,
}

impl<'a> OracleMap<'a> {
    pub fn contains(&self, pubkey: &Pubkey) -> bool {
        self.oracles.contains_key(pubkey) || pubkey == &Pubkey::default()
    }

    pub fn get_price_data(&mut self, pubkey: &Pubkey) -> BumpResult<&OraclePriceData> {
        if self.price_data.contains_key(pubkey) {
            return self.price_data.get(pubkey).safe_unwrap().clone();
        }

        let account_info = match self.oracles.get(pubkey) {
            Some(AccountInfoAndOracleSource {
                     account_info
                 }) => account_info,
            None => {
                msg!("oracle pubkey not found in oracle_map: {}", pubkey);
                return Err(OracleNotFound);
            }
        };
        let price_result = get_oracle_price(account_info)?;
        self.price_data.insert(*pubkey, price_result);

        self.price_data.get(pubkey).safe_unwrap()
    }

    pub fn load<'c>(
        account_info_iter: &'c mut Peekable<Iter<AccountInfo<'a>>>) -> anchor_lang::Result<OracleMap<'a>> {
        let mut oracles: BTreeMap<Pubkey, AccountInfoAndOracleSource<'a>> = BTreeMap::new();

        while let Some(account_info) = account_info_iter.peek() {
            if account_info.owner == &pyth_program::id() {
                let account_info = account_info_iter.next().safe_unwrap()?;
                let pubkey = account_info.key();

                oracles.insert(
                    pubkey,
                    AccountInfoAndOracleSource {
                        account_info: account_info.clone()
                    },
                );
                continue;
            }
            break;
        }


        Ok(OracleMap {
            oracles,
            price_data: BTreeMap::new(),
        })
    }
}

#[cfg(test)]
impl<'a> OracleMap<'a> {
    pub fn empty() -> OracleMap<'a> {
        OracleMap {
            oracles: BTreeMap::new(),
            price_data: BTreeMap::new(),
        }
    }
}
