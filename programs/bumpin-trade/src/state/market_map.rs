use std::cell::{Ref, RefMut};
use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use arrayref::array_ref;

use crate::errors::BumpErrorCode::{CouldNotLoadMarketData, MarketNotFind};
use crate::errors::{BumpErrorCode, BumpResult};
use crate::state::market::Market;
use crate::traits::Size;

pub struct MarketMap<'a>(pub BTreeMap<[u8; 32], AccountLoader<'a, Market>>);

impl<'a> MarketMap<'a> {
    #[track_caller]
    #[inline(always)]
    pub fn get_all_market(&self) -> BumpResult<Vec<&AccountLoader<'a, Market>>> {
        let mut markets = Vec::new();
        for market_loader in self.0.values() {
            markets.push(market_loader);
        }
        Ok(markets)
    }

    #[track_caller]
    #[inline(always)]
    pub fn get_mut_ref(&self, symbol: &[u8; 32]) -> BumpResult<RefMut<Market>> {
        let loader = match self.0.get(symbol) {
            None => {
                return Err(MarketNotFind);
            },
            Some(loader) => loader,
        };
        match loader.load_mut() {
            Ok(market) => Ok(market),
            Err(e) => {
                msg!("{:?}", e);
                Err(CouldNotLoadMarketData)
            },
        }
    }
    #[track_caller]
    #[inline(always)]
    pub fn get_ref(&self, symbol: &[u8; 32]) -> BumpResult<Ref<Market>> {
        let loader = match self.0.get(symbol) {
            None => {
                return Err(MarketNotFind);
            },
            Some(loader) => loader,
        };
        match loader.load() {
            Ok(market) => Ok(market),
            Err(e) => {
                msg!("{:?}", e);
                Err(CouldNotLoadMarketData)
            },
        }
    }
}

impl<'a> MarketMap<'a> {
    pub fn load(remaining_accounts: &'a [AccountInfo<'a>]) -> BumpResult<MarketMap<'a>> {
        let mut perp_market_map: MarketMap = MarketMap(BTreeMap::new());
        let market_discriminator: [u8; 8] = Market::discriminator();
        for account_info in remaining_accounts {
            if !account_info.owner.eq(&crate::id()) {
                continue;
            }
            if let Ok(data) = account_info.try_borrow_data() {
                let expected_data_len = Market::SIZE;
                if data.len() < expected_data_len {
                    continue;
                }
                let account_discriminator = array_ref![data, 0, 8];
                if account_discriminator != &market_discriminator {
                    continue;
                }
                let symbol = *array_ref![data, 8, 32];

                let account_loader: AccountLoader<'a, Market> =
                    AccountLoader::try_from(account_info)
                        .or(Err(BumpErrorCode::InvalidMarketAccount))?;
                perp_market_map.0.insert(symbol, account_loader);
            } else {
                continue;
            }
        }
        Ok(perp_market_map)
    }
}
