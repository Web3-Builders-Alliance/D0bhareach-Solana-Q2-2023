use crate::state::EscrowState;
use anchor_lang::prelude::*;
use anchor_spl::token::{CloseAccount, Mint, Token, TokenAccount, TransferChecked};

#[derive(Accounts)]
pub struct Exchange<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub alice: Signer<'info>,
    pub alice_apple_token_mint: Account<'info, Mint>,
    pub alice_banana_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub bob_banana_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub bob_apple_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub alice_apple_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub alice_banana_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = escrow_state.banana_amount <= bob_banana_token_account.amount,
        constraint = escrow_state.alice_apple_token_account == *alice_apple_token_account.to_account_info().key,
        constraint = escrow_state.alice_banana_token_account == *alice_banana_token_account.to_account_info().key,
        constraint = escrow_state.alice_key == *alice.key,
        close = initializer
    )]
    pub escrow_state: Account<'info, EscrowState>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(
        seeds = [b"authority".as_ref()],
        bump,
    )]
    pub vault_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

impl<'info> Exchange<'info> {
    pub fn get_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.bob_banana_token_account.to_account_info(),
            mint: self.bob_banana_token_mint.to_account_info(),
            to: self.alice_apple_token_account.to_account_info(),
            authority: self.vault_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn get_transfer_to_taker_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.initializer_deposit_token_mint.to_account_info(),
            to: self.taker_receive_token_account.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    pub fn get_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.alice.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
