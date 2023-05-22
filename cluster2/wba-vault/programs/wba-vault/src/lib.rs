use anchor_lang::prelude::*;

declare_id!("HAXGA1FwMfL1pgdAeexVTADUbJ6Fk3ziTWKTD9jyjfbt");

// need to initialaze vault to store 

#[program]
pub mod wba_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault_state;
        vault.score =  0;
        // keys in BTreeMap are the same as names in Accounts, watch out!!!!
        vault.auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        vault.vault_bump = *ctx.bumps.get("vault_holder").unwrap();
        vault.owner = *ctx.accounts.owner.key;
        Ok(())
    }
    
    pub fn update_score(ctx: Context<VaultHolder>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        msg!("vault:?");
        msg!("before add: {}", vault.score);
        vault.score += 1;
        msg!("after add: {}", vault.score);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
     #[account(mut)]
    pub owner : Signer <'info>,

    #[account(init, payer=owner, space=Vault::VAULT_DATA_SIZE)]
    pub vault_state : Account <'info, Vault>,

    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    /// CHECK don't need to validate.
    pub vault_auth: UncheckedAccount<'info>,

    #[account(seeds = [b"holder", vault_auth.key().as_ref()], bump)]
    pub vault_holder : SystemAccount <'info>,

    pub system_program: Program <'info, System>
}

// have no idea why we need all bumps. This account is actual data holder
#[account]
#[derive(Default, Debug)]
pub struct Vault {
    owner: Pubkey, // 32
    auth_bump: u8, // 1
    vault_bump: u8, // 1
    pub score: u8, // 1 
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