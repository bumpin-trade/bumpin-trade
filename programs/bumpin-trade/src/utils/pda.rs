use anchor_lang::prelude::*;

use crate::errors::BumpResult;

pub fn generate_position_key(
    user: &Pubkey,
    symbol: [u8; 32],
    is_portfolio_margin: bool,
    program_id: &Pubkey,
) -> BumpResult<Pubkey> {
    // Convert is_portfolio_margin to a byte array
    let is_portfolio_margin_bytes: &[u8] = if is_portfolio_margin { &[1] } else { &[0] };
    let seeds: &[&[u8]] = &[user.as_ref(), &symbol, is_portfolio_margin_bytes];

    // Find the program address
    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}

pub fn generate_pool_vault_key(pool_index: u16, program_id: &Pubkey) -> BumpResult<Pubkey> {
    let binding = pool_index.to_le_bytes();
    let seeds: &[&[u8]] = &["pool_mint_vault".as_ref(), binding.as_ref()];
    // Find the program address
    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}

pub fn generate_trade_token_vault_key(
    trade_token_index: u16,
    program_id: &Pubkey,
) -> BumpResult<Pubkey> {
    let binding = trade_token_index.to_le_bytes();
    let seeds: &[&[u8]] = &["trade_token_vault".as_ref(), binding.as_ref()];

    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}

pub fn generate_pool_pda(pool_index: u16, program_id: &Pubkey) -> BumpResult<Pubkey> {
    let binding = pool_index.to_le_bytes();
    let seeds: &[&[u8]] = &["pool".as_ref(), binding.as_ref()];

    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}

pub fn generate_market_pda(market_index: u16, program_id: &Pubkey) -> BumpResult<Pubkey> {
    let binding = market_index.to_le_bytes();
    let seeds: &[&[u8]] = &["market".as_ref(), binding.as_ref()];

    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}

pub fn generate_user_pda(user_authority: &Pubkey, program_id: &Pubkey) -> BumpResult<Pubkey> {
    let binding = user_authority.to_bytes();
    let seeds: &[&[u8]] = &["user".as_ref(), binding.as_ref()];

    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}
