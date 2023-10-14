use crate::errors::AmmError;
use crate::state::config::PoolConfig;
use crate::{assert_non_zero, assert_not_expired, assert_not_locked};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer};
use constant_product_curve::ConstantProduct;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Box<Account<'info, Mint>>,
    pub mint_y: Box<Account<'info, Mint>>,
    #[account(
            mut,
            seeds = [b"lp", config.key().as_ref()],
            bump
        )]
    pub mint_lp: Box<Account<'info, Mint>>,
    #[account(
            mut,
            associated_token::mint = mint_x,
            associated_token::authority = auth,
        )]
    pub vault_x: Box<Account<'info, TokenAccount>>,
    #[account(
            mut,
            associated_token::mint = mint_y,
            associated_token::authority = auth,
        )]
    pub vault_y: Box<Account<'info, TokenAccount>>,
    #[account(
            mut,
            associated_token::mint = mint_x,
            associated_token::authority = user,
        )]
    pub user_x: Box<Account<'info, TokenAccount>>,
    #[account(
            mut,
            associated_token::mint = mint_y,
            associated_token::authority = user,
        )]
    pub user_y: Box<Account<'info, TokenAccount>>,
    #[account(
            init_if_needed,
            payer = user,
            associated_token::mint = mint_lp,
            associated_token::authority = user,
        )]
    pub user_lp: Box<Account<'info, TokenAccount>>,

    /// CHECK: just a pda for signing
    #[account(seeds = [b"auth", config.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
            seeds = [b"config", mint_x.key().as_ref(), mint_y.key().as_ref()],
            bump,
        )]
    pub config: Account<'info, PoolConfig>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(
        &self,
        amount: u64, // Amount of LP token to claim + WHY DO WE NEED THIS?
        max_x: u64,  // Max amount of X we are willing to deposit
        max_y: u64,  // Max amount of Y we are willing to deposit
        expiration: i64,
    ) -> Result<()> {
        assert_not_locked!(self.config.locked);
        assert_not_expired!(expiration);
        assert_non_zero!([max_x, max_y]);

        let (x, y) = match self.mint_lp.supply == 0
            && self.vault_x.amount == 0
            && self.vault_y.amount == 0
        {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6,
                )
                .map_err(AmmError::from)?;
                (amounts.x, amounts.y)
            }
        };

        // Check for slippage
        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);
        self.deposit_tokens(true, x)?;
        self.deposit_tokens(false, y)?;
        self.mint_lp_tokens(amount)
    }

    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false => (
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
            ),
        };
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }

    pub fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[
            &b"auth"[..],
            &self.config.key().clone().to_bytes(),
            &[self.config.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        mint_to(ctx, amount)
    }
}
