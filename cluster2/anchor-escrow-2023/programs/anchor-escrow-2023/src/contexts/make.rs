use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::structs::Escrow;
// Creates a state for escrow and transfers maker tokes to the vault.
#[derive(Accounts)]
#[instruction(seed: u64)] // instruction / args to access during context creation. Passed in fn.
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = maker_token, //  == represents tokens of this type.
        associated_token::authority = maker // what exactly authority maker will sign account changes
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    // found here: https://www.alchemy.com/overviews/associated-token-account
    // An Associated Token Account is created via the Solana Associated Token Account Program,
    // holds information about a specific token (balance, owner). It is a variant of
    // a Program Derived Address. Bumped of the curve.
    // used to mint tokens to it in anchor-escrow-2023.ts:258. Refund to refund.rs:60

    // mints are kinds of tokens(apple, banana).
    // this is required to create
    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,

    #[account(
        seeds = [b"auth"],
        bump
    )]
    /// CHECK: This is not dangerous because this account doesn't exist
    pub auth: UncheckedAccount<'info>, // has 'power of attorney' to sign instead
    // of program. I think SystemProgram creates / find address for this one.
    #[account(
        init,
        payer = maker,
        seeds = [b"vault", escrow.key().as_ref()],
        bump,
        token::mint = maker_token,
        token::authority = auth
        // auth has power of attorney. It will sign on behalf of the program later
        // in take.rs empty_vault_to_taker method.
    )]
    pub vault: Account<'info, TokenAccount>, // but we already have nice and shiny ATA
    // maker_ata, what for is this one???
    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key.as_ref(), seed.to_le_bytes().as_ref()], // GPS coordinates
        bump,
        space = Escrow::LEN
    )]
    pub escrow: Box<Account<'info, Escrow>>, // program state holds bytes []

    // has functions to work with tokens of the same type. Must read src.
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>, // master of ATA keys, creates keys for ATA???
    pub system_program: Program<'info, System>, // master of keys, creates new accounts  of standard type
}

impl<'info> Make<'info> {
    pub fn transfer_to_vault(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }
}
