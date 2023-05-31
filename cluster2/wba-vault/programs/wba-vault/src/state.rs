use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct EscrowState {
    pub escrow_seed: u64, // random seed to have new state for every offer
    pub alice_key: Pubkey,
    pub alice_apple_token_account: Pubkey,
    pub alice_banana_token_account: Pubkey, // to save bananas after exchange.
    pub apple_amount: u64,
    pub banana_amount: u64,
    pub vault_authority_bump: u8, // pda derived from seed and programId. It's used to mint more tokens.
                                  // will use one authority to mint both apples and bananas in ts test file.
}

impl EscrowState {
    pub fn space() -> usize {
        8 + (64 * 3) + (32 * 3) + 1
    }
}
