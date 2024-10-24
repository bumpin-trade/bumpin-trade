use std::collections::BTreeMap;

use crate::errors::BumpErrorCode::OracleNotFound;
use crate::errors::{BumpErrorCode, BumpResult};
use crate::ids::pyth_program;
use crate::math::safe_unwrap::SafeUnwrap;
use crate::state::oracle::{get_oracle_price, OraclePriceData};
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{FeedId, PriceUpdateV2};

pub struct OracleMap {
    oracles: BTreeMap<FeedId, PriceUpdateV2>,
    price_data: BTreeMap<FeedId, OraclePriceData>,
}

impl OracleMap {
    pub fn contains(&self, pubkey: &FeedId) -> bool {
        self.oracles.contains_key(pubkey) || pubkey == &FeedId::default()
    }

    pub fn get_price_data(&mut self, key: &Pubkey) -> BumpResult<&OraclePriceData> {
        let feed_id: &FeedId = key.as_ref().try_into().or(Err(BumpErrorCode::InvalidOracle))?;
        if self.price_data.contains_key(feed_id) {
            return self.price_data.get(feed_id).safe_unwrap().clone();
        }
        let price_update_v2 = match self.oracles.get(feed_id) {
            Some(price_update_v2) => price_update_v2,
            None => {
                msg!("oracle pubkey:{} not found in oracle_map", key);
                return Err(OracleNotFound);
            },
        };
        let price_result = get_oracle_price(feed_id, price_update_v2)?;
        self.price_data.insert(*feed_id, price_result);

        self.price_data.get(feed_id).safe_unwrap()
    }

    pub fn load<'a>(remaining_accounts: &'a [AccountInfo<'a>]) -> BumpResult<OracleMap> {
        let mut oracles: BTreeMap<[u8; 32], PriceUpdateV2> = BTreeMap::new();
        // let price_update_v2_discriminator: [u8; 8] = PriceUpdateV2::discriminator();
        for account_info in remaining_accounts.iter() {
            if account_info.owner == &pyth_program::id() {
                if let Ok(data) = account_info.try_borrow_mut_data() {
                    // let expected_data_len = PriceUpdateV2::LEN;
                    // if data.len() < expected_data_len {
                    //     continue;
                    // }
                    // let account_discriminator = array_ref![data, 0, 8];
                    // if account_discriminator != &price_update_v2_discriminator {
                    //     continue;
                    // }

                    // let data = &account_info.data.borrow_mut();
                    match PriceUpdateV2::try_deserialize(&mut &**data) {
                        Ok(price_update_v2) => {
                            let feed_id = price_update_v2.price_message.feed_id.clone();
                            oracles.insert(feed_id, price_update_v2);
                        },
                        Err(_) => {
                            msg!("error in deserialize priceUpdateV2");
                            continue;
                        },
                    }
                    // let price_update_v2 = PriceUpdateV2::try_deserialize(&mut &***data).or(Err(BumpErrorCode::InvalidPriceUpdateV2Account))?;

                    // oracles.insert(price_update_v2.price_message.feed_id.clone(), price_update_v2);
                } else {
                    msg!("price_update_v2 borrowing failed");
                    continue;
                }
            }
        }

        Ok(OracleMap { oracles, price_data: BTreeMap::new() })
    }
}
