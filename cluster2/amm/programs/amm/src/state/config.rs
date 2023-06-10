#![allow(clippy::too_many_arguments)]
use anchor_lang::prelude::*;

use anchor_lang::prelude::Pubkey;

// in amm we have to have two mints x and y
// we need Config because we can not construct bumps, or can we?
// this way we use Initialization to construct what we need and use Config to keep the data.
#[account]
pub struct Config {
    pub admin: Option<Pubkey>, // 1 + 32
    pub mint_x: Pubkey,        // 32
    pub mint_y: Pubkey,        // 32
    pub seed: u64,             // seed to create some of PDA // 8
    pub frozen: bool,          // 1
    pub config_bump: u8,       // 1
    pub auth_bump: u8,         // 1
    pub lp_bump: u8,           // 1
}

impl Config {
    pub const CONFIG_SIZE: usize = (3 * 32) + (2 * 8) + 5;
    pub fn init(
        &mut self,
        admin: Option<Pubkey>,
        mint_x: Pubkey,
        mint_y: Pubkey,
        seed: u64,
        frozen: bool,
        config_bump: u8,
        auth_bump: u8,
        lp_bump: u8,
    ) -> Result<()> {
        self.admin = admin;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.seed = seed;
        self.frozen = frozen;
        self.config_bump = config_bump;
        self.auth_bump = auth_bump;
        self.lp_bump = lp_bump;
        Ok(())
    }
}
