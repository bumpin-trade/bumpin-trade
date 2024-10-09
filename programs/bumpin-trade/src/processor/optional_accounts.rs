use anchor_lang::prelude::*;

use crate::errors::BumpResult;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool_map::PoolMap;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::vault_map::VaultMap;

pub struct AccountMaps<'a> {
    pub market_map: MarketMap<'a>,
    pub trade_token_map: TradeTokenMap<'a>,
    pub oracle_map: OracleMap,
    pub pool_map: PoolMap<'a>,
    pub vault_map: VaultMap<'a>,
}

#[track_caller]
pub fn load_maps<'a: 'info, 'info>(
    remaining_accounts: &'info [AccountInfo<'info>],
) -> BumpResult<AccountMaps<'info>> {
    let market_map = MarketMap::load(remaining_accounts)?;
    let trade_token_map = TradeTokenMap::load(remaining_accounts)?;
    let oracle_map = OracleMap::load(remaining_accounts)?;
    let pool_map = PoolMap::load(remaining_accounts)?;
    let vault_map = VaultMap::load(remaining_accounts)?;

    Ok(AccountMaps { market_map, trade_token_map, oracle_map, pool_map, vault_map })
}
