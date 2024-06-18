use crate::errors::BumpResult;
use crate::processor::optional_accounts::AccountMaps;

pub fn re_balance_pool_unsettle(account_maps: AccountMaps) -> BumpResult {
    let trade_token_map = account_maps.trade_token_map;
    let trade_token_vec = trade_token_map.get_all_trade_token()?;


    Ok(())
}

pub fn re_balance_stable_pool_unsettle(account_maps: AccountMaps) -> BumpResult {
    let trade_token_map = account_maps.trade_token_map;
    let trade_token_vec = trade_token_map.get_all_trade_token()?;
    Ok(())
}