use crate::errors::AmmError;
use crate::state::config::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    /// CHECK: This is authority for many accounts.
    #[account(
        seeds = [b"auth"],
        bump
    )]
    pub pda_auth: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"config"],
        bump,
        space =  Config::CONFIG_SIZE,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_x,
        associated_token::authority = pda_auth,
    )]
    pub vault_token_x: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint_y,
        associated_token::authority = pda_auth,
    )]
    pub vault_token_y: Box<Account<'info, TokenAccount>>,

    //  It can be anything in seeds, I guess config chosen for it's kinda const it will stay to the
    // rest of the program and it's unknown to outside world since it's also the address that exists
    // outside of the curve.
    #[account(init,
    seeds = [b"lp", config.key().as_ref()],
    bump,
    payer = payer,
    mint::decimals = 6,
    mint::authority = pda_auth,
    )]
    pub lp_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        seed: u64,
        admin: Option<Pubkey>,
    ) -> Result<()> {
        // Initialize went through initialization and bumps now calculated
        let (auth_bump, config_bump, lp_bump) = (
            *bumps.get("auth").ok_or(AmmError::BumpError)?,
            *bumps.get("config").ok_or(AmmError::BumpError)?,
            *bumps.get("lp").ok_or(AmmError::BumpError)?,
        );

        self.config.init(
            admin,
            self.mint_x.key(),
            self.mint_y.key(),
            seed,
            false,
            config_bump,
            auth_bump,
            lp_bump,
        )
    }
}
