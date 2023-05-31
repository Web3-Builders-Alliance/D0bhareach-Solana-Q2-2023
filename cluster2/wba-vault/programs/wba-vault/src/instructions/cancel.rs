use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::token::{CloseAccount, Mint, Token, TokenAccount, TransferChecked};

#[derive(Accounts)]
pub struct Cancel<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub alice: Signer<'info>,
    pub apple_mint: Account<'info, Mint>,
    #[account(mut)]
    pub apple_vault: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        seeds = [b"authority".as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,
    #[account(mut)]
    pub alice_apple_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = escrow_state.alice_key == *alice.key,
        constraint = escrow_state.alice_apple_token_account == *alice_apple_token_account.to_account_info().key,
        close = alice
    )]
    pub escrow_state: Account<'info, EscrowState>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

impl<'info> Cancel<'info> {
    pub fn get_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.apple_vault.to_account_info(),
            mint: self.apple_mint.to_account_info(),
            to: self.alice_apple_token_account.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn get_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.apple_vault.to_account_info(),
            destination: self.alice.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
