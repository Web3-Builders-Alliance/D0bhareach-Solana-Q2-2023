use anchor_lang::prelude::*;
// use anchor_lang::program;
use anchor_spl::token::{self, Token, TokenAccount};
use cancel::*;
use exchange::*;
use instructions::*;
pub mod instructions;
pub mod state;

declare_id!("HAXGA1FwMfL1pgdAeexVTADUbJ6Fk3ziTWKTD9jyjfbt");

// Description:
// Two parties buyer and seller are exchanging apples for bananas using Anchor framework.
// since I'm still learning would make everything easyer  by naming my seller Alice and
// my buyer Bob. So Alice wants to trade some apples for some bananas.
const AUTHORITY_SEED: &[u8] = b"authority";

#[program]
pub mod wba_vault {
    use anchor_spl::token;

    use super::*;
    // need three methods init - where initializator makes a proposition.
    // cancel - where initializator is canceling the offer
    // exchange - apples and oranges are exchainged

    pub fn initialize(
        ctx: Context<Initialize>,
        escrow_seed: u64,
        apple_amount: u64,
        banana_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_state.alice_key = *ctx.accounts.alice.key;
        ctx.accounts.escrow_state.alice_apple_token_account =
            *ctx.accounts.alice_apple_token_account.to_account_info().key;
        ctx.accounts.escrow_state.alice_banana_token_account = *ctx
            .accounts
            .alice_banana_token_account
            .to_account_info()
            .key;
        ctx.accounts.escrow_state.apple_amount = apple_amount;
        ctx.accounts.escrow_state.banana_amount = banana_amount;
        ctx.accounts.escrow_state.escrow_seed = escrow_seed;

        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[AUTHORITY_SEED], ctx.program_id);
        ctx.accounts.escrow_state.vault_authority_bump = vault_authority_bump;

        // setup offer
        token::transfer_checked(
            ctx.accounts.get_transfer_to_pda_context(),
            ctx.accounts.escrow_state.apple_amount,
            ctx.accounts.apple_mint.decimals,
        )?;

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let authority_seeds = &[
            AUTHORITY_SEED,
            &[ctx.accounts.escrow_state.vault_authority_bump],
        ];

        token::transfer_checked(
            ctx.accounts
                .get_transfer_to_initializer_context()
                .with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_state.apple_amount,
            ctx.accounts.apple_mint.decimals,
        )?;

        token::close_account(
            ctx.accounts
                .get_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }
    pub fn exchange(ctx: Context<Exchange>) -> Result<()> {
        let authority_seeds = &[
            AUTHORITY_SEED,
            &[ctx.accounts.escrow_state.vault_authority_bump],
        ];

        token::transfer_checked(
            ctx.accounts.get_transfer_to_initializer_context(),
            ctx.accounts.escrow_state.banana_amount,
            ctx.accounts.alice_banana_token_mint.decimals,
        )?;

        token::transfer_checked(
            ctx.accounts
                .get_transfer_to_taker_context()
                .with_signer(&[&authority_seeds[..]]),
            ctx.accounts.escrow_state.apple_amount,
            ctx.accounts.alice_apple_token_mint.decimals,
        )?;

        token::close_account(
            ctx.accounts
                .get_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;

        Ok(())
    }
}
