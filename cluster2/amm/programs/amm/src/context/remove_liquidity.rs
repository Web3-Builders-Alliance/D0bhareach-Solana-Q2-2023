use crate::errors::AmmError;
use crate::state::config::Config;
use anchor_lang::prelude::*;
use anchor_spl::token::{burn, transfer, Burn, Mint, Token, TokenAccount, Transfer};

// TODO: refactor to base account common with AddLiquidity
#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = pda_auth,
    )]
    pub vault_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = pda_auth,
    )]
    pub vault_y: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = lp_mint,
        associated_token::authority = pda_auth,
    )]
    pub lp_token: Box<Account<'info, TokenAccount>>,
    #[account(
        associated_token::mint = lp_mint,
        associated_token::authority = user,
    )]
    pub user_lp_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = user,
    )]
    pub user_token_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = user,
    )]
    pub user_token_y: Box<Account<'info, TokenAccount>>,

    /// CHECK: This is for signing.
    #[account(
        mut,
        seeds = [ b"pda_auth" ],
        bump = config.auth_bump,
    )]
    pub pda_auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub lp_mint: Box<Account<'info, Mint>>,
    // looks like this config is not the same created in init.
    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Box<Account<'info, Config>>,
    pub token_program: Program<'info, Token>,
    // pub associated_token_program: Program<AssociatedToken>,
}
impl<'info> RemoveLiquidity<'info> {
    // What does it mean to remove liquidity?
    pub fn remove_liquidity(
        &self,
        amount_x_min: u64,
        amount_y_min: u64,
        expiration: i64,
    ) -> Result<()> {
        require!(!self.config.frozen, AmmError::PoolFrozen);
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time.lt(&expiration),
            AmmError::RemoveLiquidityExpired
        );
        let amount = self.lp_token.amount;
        self.burn_lp(amount)?;
        let amount_x = self.get_token_amount(amount, amount_x_min)?;
        self.transfer_tokens(true, amount_x)?;
        let amount_y = self.get_token_amount(amount, amount_y_min)?;
        self.transfer_tokens(true, amount_y)
    }

    pub fn get_token_amount(&self, lp_amount: u64, minimum: u64) -> Result<u64> {
        let token_amount = lp_amount
            .checked_mul(self.vault_x.amount)
            .unwrap()
            .checked_div(self.lp_mint.supply)
            .unwrap();
        if token_amount.lt(&minimum) {
            Err(AmmError::AmountLessThanMinimum.into())
        } else {
            Ok(token_amount)
        }
    }

    pub fn burn_lp(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: self.lp_mint.to_account_info(),
            from: self.user_lp_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        burn(ctx, amount)
    }
    // TODO: refactor to helper function
    pub fn transfer_tokens(&self, token_x: bool, amount: u64) -> Result<()> {
        let (user_token, recepient_account_info) = if token_x {
            (
                self.user_token_x.to_account_info(),
                self.vault_x.to_account_info(),
            )
        } else {
            (
                self.user_token_y.to_account_info(),
                self.vault_y.to_account_info(),
            )
        };

        let cpi_accounts = Transfer {
            from: user_token,
            to: recepient_account_info,
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }
}
