use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub expiry: u64, // expire time
    pub maker: Pubkey,
    pub maker_token: Pubkey, // mint
    pub taker_token: Pubkey, // mint
    pub offer_amount: u64,
    pub seed: u64, // ??
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub escrow_bump: u8,
}

impl Escrow {
    pub const LEN: usize = 8 + 3 * 32 + 3 * 8 + 3 * 1;
}
