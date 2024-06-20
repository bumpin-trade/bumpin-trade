use std::cell::Ref;
use std::collections::BTreeMap;
use std::panic::Location;

use anchor_lang::prelude::AccountLoader;
use anchor_lang::Discriminator;
use arrayref::array_ref;
use solana_program::account_info::AccountInfo;
use solana_program::msg;
use solana_program::pubkey::Pubkey;

use crate::errors::BumpErrorCode::{
    CouldNotLoadTradeTokenData, InvalidTradeTokenAccount, TradeTokenNotFind,
};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::state::trade_token::TradeToken;
use crate::traits::Size;
use crate::validate;

pub struct TradeTokenMap<'a>(pub BTreeMap<Pubkey, AccountLoader<'a, TradeToken>>);

impl<'a> TradeTokenMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_all_trade_token(&self) -> BumpResult<Vec<TradeToken>> {
        let mut trade_tokens = Vec::new();
        for trade_token_loader in self.0.values() {
            let trade_token = trade_token_loader
                .load()
                .map_err(|e| {
                    let caller = Location::caller();
                    msg!("{:?}", e);
                    msg!("Could not load trade_token at {}:{}", caller.file(), caller.line());
                    CouldNotLoadTradeTokenData
                })?
                .clone();
            trade_tokens.push(trade_token);
        }
        Ok(trade_tokens)
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_trade_token(&self, mint: &Pubkey) -> BumpResult<Ref<TradeToken>> {
        let loader = match self.0.get(mint) {
            None => {
                let caller = Location::caller();
                msg!("Could not find trade_token {} at {}:{}", mint, caller.file(), caller.line());
                return Err(TradeTokenNotFind);
            },
            Some(loader) => loader,
        };
        match loader.load() {
            Ok(trade_token) => Ok(trade_token),
            Err(e) => {
                let caller = Location::caller();
                msg!("{:?}", e);
                msg!("Could not load trade_token {} at {}:{}", mint, caller.file(), caller.line());
                Err(CouldNotLoadTradeTokenData)
            },
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_account_loader(&self, mint: &Pubkey) -> BumpResult<&AccountLoader<'a, TradeToken>> {
        let loader = match self.0.get(mint) {
            None => {
                let caller = Location::caller();
                msg!("Could not find trade_token {} at {}:{}", mint, caller.file(), caller.line());
                return Err(TradeTokenNotFind);
            },
            Some(loader) => loader,
        };
        Ok(loader)
    }
    pub fn load(
        remaining_accounts: &'a [AccountInfo<'a>],
        admin: &Pubkey,
    ) -> BumpResult<TradeTokenMap<'a>> {
        let mut trade_token_vec: TradeTokenMap = TradeTokenMap(BTreeMap::new());
        let trade_token_discriminator = TradeToken::discriminator();
        for account_info in remaining_accounts.iter() {
            // validate!(account_info.owner.eq(admin), CouldNotLoadTradeTokenData)?;
            let data = account_info.try_borrow_data().or(Err(CouldNotLoadTradeTokenData))?;

            let expected_data_len = TradeToken::SIZE;
            if data.len() < expected_data_len {
                break;
            }
            let account_discriminator = array_ref![data, 0, 8];
            if account_discriminator != &trade_token_discriminator {
                continue;
            }

            let trade_token_mint = Pubkey::from(*array_ref![data, 8, 32]);
            let account_loader: AccountLoader<'a, TradeToken> =
                AccountLoader::try_from(account_info).or(Err(InvalidTradeTokenAccount))?;

            trade_token_vec.0.insert(trade_token_mint, account_loader);
        }
        Ok(trade_token_vec)
    }
}
