use std::cell::RefMut;
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::slice::Iter;

use anchor_lang::Discriminator;
use anchor_lang::prelude::AccountLoader;
use arrayref::array_ref;
use solana_program::account_info::AccountInfo;
use solana_program::msg;

use crate::errors::{BumpErrorCode, BumpResult};
use crate::errors::BumpErrorCode::{CouldNotLoadTradeTokenData, TradeTokenNotFind};
use crate::math::safe_unwrap::SafeUnwrap;
use crate::state::market::Market;
use crate::traits::Size;

pub struct MarketMap<'a>(pub BTreeMap<[u8; 32], AccountLoader<'a, Market>>);

impl<'a> MarketMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_all_market(&self) -> BumpResult<BTreeMap<[u8; 32], Market>> {
        let market = self.0.iter().
            map(|account_loader| {
                account_loader.load()?
            }).collect();
        Ok(market)
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_mut_ref(&self, symbol: &[u8; 32]) -> BumpResult<RefMut<Market>> {
        let loader = match self.0.get(symbol) {
            None => {
                return Err(TradeTokenNotFind);
            }
            Some(loader) => loader
        };
        match loader.load_mut() {
            Ok(market) => Ok(market),
            Err(e) => {
                msg!("{:?}", e);
                Err(CouldNotLoadTradeTokenData)
            }
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_ref(&self, symbol: &[u8; 32]) -> BumpResult<RefMut<Market>> {
        let loader = match self.0.get(symbol) {
            None => {
                return Err(TradeTokenNotFind);
            }
            Some(loader) => loader
        };
        match loader.load_mut() {
            Ok(market) => Ok(market),
            Err(e) => {
                Err(CouldNotLoadTradeTokenData)
            }
        }
    }
}

impl<'a> MarketMap<'a> {
    pub fn load<'b, 'c>(
        account_info_iter: &'c mut Peekable<Iter<AccountInfo<'a>>>,
    ) -> BumpResult<MarketMap<'a>> {
        let mut perp_market_map: MarketMap = MarketMap(BTreeMap::new());
        let market_discriminator: [u8; 8] = Market::discriminator();
        while let Some(account_info) = account_info_iter.peek() {
            let data = account_info
                .try_borrow_data()
                .or(Err(BumpErrorCode::CouldNotLoadMarketData))?;

            let expected_data_len = Market::SIZE;
            if data.len() < expected_data_len {
                continue;
            }
            let account_discriminator = array_ref![data, 0, 8];
            if account_discriminator != &market_discriminator {
                //check the discriminator
                continue;
            }
            let symbol = array_ref![data, 8, 32];

            let account_info = account_info_iter.next().safe_unwrap()?;
            let account_loader: AccountLoader<Market> =
                AccountLoader::try_from(account_info).or(Err(BumpErrorCode::InvalidMarketAccount))?;
            perp_market_map.0.insert(*symbol, account_loader);
        }
        Ok(perp_market_map)
    }
}