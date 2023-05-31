use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, TransferChecked},
};

#[derive(Accounts)]
#[instruction(escrow_seed: u64, apple_amount:u64, banana_amount: u64)] // passed seed from front-end
pub struct Initialize<'info> {
    #[account(mut)]
    pub alice: Signer<'info>,
    pub apple_mint: Account<'info, Mint>,
    pub banana_mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [b"state".as_ref(), &escrow_seed.to_le_bytes()],
        bump,
        payer = alice,
        space = EscrowState::space()
    )]
    pub escrow_state: Box<Account<'info, EscrowState>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        seeds = [b"authority".as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>, // PDA used to sign CPI

    #[account(
        init,
        payer = alice,
        associated_token::mint = apple_mint,
        associated_token::authority = vault_authority
    )]
    pub apple_vault: Account<'info, TokenAccount>, // wtf???

    #[account(
        mut,
        constraint = alice_apple_token_account.amount >= apple_amount
    )]
    pub alice_apple_token_account: Account<'info, TokenAccount>,
    pub alice_banana_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>, // reqired by init
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>, // required to work with tokens
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub associated_token_program: Program<'info, AssociatedToken>, // this is to work with associated tokens
}

impl<'info> Initialize<'info> {
    pub fn get_transfer_to_pda_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.alice_apple_token_account.to_account_info(),
            mint: self.apple_mint.to_account_info(),
            to: self.apple_vault.to_account_info(),
            authority: self.alice.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
