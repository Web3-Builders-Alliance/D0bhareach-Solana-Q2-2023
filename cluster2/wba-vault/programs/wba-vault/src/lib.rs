use anchor_lang::prelude::*;

declare_id!("HAXGA1FwMfL1pgdAeexVTADUbJ6Fk3ziTWKTD9jyjfbt");

// need to initialaze vault to store 

#[program]
pub mod wba_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.score =  0;
        vault.auth_bump = *ctx.bumps.get("auth").unwrap();
        vault.vault_bump = *ctx.bumps.get("vault_holder").unwrap();
        vault.owner = *ctx.accounts.owner.key;
        Ok(())
    }
    
    pub fn update_score(ctx: Context<VaultHolder>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.score += 1;
        Ok(())
    }
}
// vault must be owned by who? I think the program itself. It's done automatically according to dosc.
// this struct is for shape of context passed to initialized function.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:  this account is only to authorize vault_holder owned by my wallet.
    #[account(seeds=[b"auth", owner.key().as_ref()], bump)]
    pub vault_auth: UncheckedAccount<'info>,

    /// CHECK: this account is more down the line
    #[account(seeds=[b"vault_holder", vault_auth.key().as_ref()], bump)]
    pub vault_holder: UncheckedAccount<'info>,

    // account to hold vault data, seeded from vault_holder
    #[account(init, payer = owner,
        space = 8 + crate::Vault::VAULT_DATA_SIZE)]
    pub vault: Account<'info, Vault>,
    // account to pay, In my case it's my wallet. It used to create PDA which 
    // is used to hold status of program in the vault.
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// have no idea why we need all bumps. This account is actual data holder
#[account]
#[derive(Default)]
pub struct Vault {
    owner: Pubkey, // 32
    auth_bump: u8, // 1
    vault_bump: u8, // 1
    score: u8, // 1 
}

impl Vault {
    const VAULT_DATA_SIZE: usize = 8 + 32 + 1 + 1 + 1;

}
// need account derived with auth account which will hold account of vault in it.
#[derive(Accounts)]
pub struct VaultHolder<'info> {
    #[account(mut, seeds = [b"vault"], bump = vault.vault_bump)]
    pub vault: Account<'info, Vault>,
}