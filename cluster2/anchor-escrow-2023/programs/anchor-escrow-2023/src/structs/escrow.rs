use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub expiry: u64, // expire time
    pub maker: Pubkey,
    pub maker_token: Pubkey, // mint kind of token
    pub taker_token: Pubkey, // mint kind of token
    pub offer_amount: u64,
    pub seed: u64, // passed from the client random seed to
    // this are required to find accounts bumped from the curve, think about them as
    // a secret addition to pirate map (two steps to the East)
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub escrow_bump: u8,
}

impl Escrow {
    pub const LEN: usize = 8 + 3 * 32 + 3 * 8 + 3 * 1;
}
