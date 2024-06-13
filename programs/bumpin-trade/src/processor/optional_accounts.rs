use std::iter::Peekable;
use std::slice::Iter;

use solana_program::account_info::AccountInfo;

use crate::errors::BumpResult;
use crate::state::market_map::MarketMap;
use crate::state::oracle_map::OracleMap;
use crate::state::pool_map::PoolMap;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user_map::UserMap;

pub struct AccountMaps<'a> {
    pub market_map: MarketMap<'a>,
    pub trade_token_map: TradeTokenMap<'a>,
    pub oracle_map: OracleMap<'a>,
    pub pool_map: PoolMap<'a>,
    pub user_map: UserMap<'a>,
}

pub fn load_maps<'a: 'info, 'info>(
    account_info_iter: &mut Peekable<Iter<'a, AccountInfo<'info>>>,
) -> BumpResult<AccountMaps<'info>> {
    let market_map = MarketMap::load(account_info_iter)?;
    let trade_token_map = TradeTokenMap::load(account_info_iter)?;
    let oracle_map = OracleMap::load(account_info_iter)?;
    let pool_map = PoolMap::load(account_info_iter)?;
    let user_map = UserMap::load(account_info_iter)?;

    Ok(AccountMaps { market_map, trade_token_map, oracle_map, pool_map, user_map })
}
