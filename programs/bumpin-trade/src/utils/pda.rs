use anchor_lang::prelude::Signer;
use solana_program::pubkey::Pubkey;
use crate::errors::BumpResult;

pub fn generate_position_key(
    symbol: [u8; 32],
    user: Pubkey,
    is_cross_margin: bool,
    program_id: &Pubkey,
) -> BumpResult<Pubkey> {
    // Convert is_cross_margin to a byte array
    let is_cross_margin_bytes: &[u8] = if is_cross_margin { &[1] } else { &[0] };
    let seeds: &[&[u8]] =
        &[user.as_ref(), &symbol, is_cross_margin_bytes];

    // Find the program address
    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}

pub fn generate_pool_vault_key(
    pool_index: u16,
    program_id: &Pubkey,
) -> BumpResult<Pubkey> {
    let seeds: &[&[u8]] =
        &["pool_mint_vault".as_ref(), pool_index.to_le_bytes().as_ref()];
    // Find the program address
    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}

pub fn generate_trade_token_vault_key(
    trade_token_index: u16,
    program_id: &Pubkey,
) -> BumpResult<Pubkey> {
    let seeds: &[&[u8]] =
        &["trade_token_vault".as_ref(), trade_token_index.to_le_bytes().as_ref()];

    let (address, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(address)
}