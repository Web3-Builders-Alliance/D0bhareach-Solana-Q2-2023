use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::structs::Escrow;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub new_taker_token: Account<'info, Mint>, // in case if mint is changing.
    #[account(
        mut,
        has_one = maker,
        seeds = [b"escrow", maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.escrow_bump
    )]
    pub escrow: Box<Account<'info, Escrow>>, // get our state from coordinates.
    pub system_program: Program<'info, System>, // this is the owner of coordinates, owner creator don't understand why its here.
}
