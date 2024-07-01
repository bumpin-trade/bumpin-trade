pub fn get_signer_seeds(nonce: &u8) -> [&[u8]; 2] {
    [b"bump_state".as_ref(), bytemuck::bytes_of(nonce)]
}
