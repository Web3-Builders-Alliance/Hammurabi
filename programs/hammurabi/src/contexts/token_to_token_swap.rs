use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};
use anchor_spl::associated_token::AssociatedToken;
use constant_product_curve::{ConstantProduct, LiquidityPair};
use crate::{ assert_not_locked, assert_not_expired, assert_non_zero};
use crate::state::config::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct TokenToTokenSwap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // Base currency: SOL/USDC
    pub mint_x: Box<Account<'info, Mint>>,
    // Trade currency: SPL Token
    pub mint_y: Box<Account<'info, Mint>>,
    // Counter-trade currency: SPL Token
    pub mint_z: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user
    )]
    pub user_x: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user
    )]
    pub user_y: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_z,
        associated_token::authority = user
    )]
    pub user_z: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth1
    )]
    pub vault_x1: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth2
    )]
    pub vault_x2: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = auth1
    )]
    pub vault_y: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_z,
        associated_token::authority = auth2
    )]
    pub vault_z: Box<Account<'info, TokenAccount>>,
    /// CHECK: just a pda for signing
    #[account(seeds = [b"auth", config1.key().as_ref()], bump = config1.auth_bump)]
    pub auth1: UncheckedAccount<'info>,
    /// CHECK: just a pda for signing
    #[account(seeds = [b"auth", config2.key().as_ref()], bump = config2.auth_bump)]
    pub auth2: UncheckedAccount<'info>,
    #[account(
        constraint = config1.mint_x == mint_x.key(),
        constraint = config1.mint_y == mint_y.key(),
        seeds = [
            b"config",
            config1.seed.to_le_bytes().as_ref()
        ],
        bump = config1.config_bump,
    )]
    pub config1: Account<'info, Config>,
    #[account(
        constraint = config2.mint_x == mint_x.key(),
        constraint = config2.mint_y == mint_z.key(),
        seeds = [
            b"config",
            config2.seed.to_le_bytes().as_ref()
        ],
        bump = config2.config_bump,
    )]
    pub config2: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> TokenToTokenSwap<'info> {
    pub fn token_to_token_swap(
        &mut self,
        is_x: bool,
        intermidiate_is_x: bool,
        amount: u64,
        min: u64,
        expiration: i64
    ) -> Result<()> {
        assert_not_locked!(self.config1.locked);
        assert_not_locked!(self.config2.locked);
        assert_not_expired!(expiration);
        assert_non_zero!([amount]);

        let mut curve_1 = ConstantProduct::init(
            self.vault_x1.amount,
            self.vault_y.amount,
            self.vault_x1.amount,
            self.config1.fee,
            None
        ).map_err(AmmError::from)?;

        let mut curve_2 = ConstantProduct::init(
            self.vault_x2.amount,
            self.vault_z.amount,
            self.vault_x2.amount,
            self.config2.fee,
            None
        ).map_err(AmmError::from)?;

        let p_1 = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y
        };

        let p_2 = match intermidiate_is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y
        };

        let res_1 = curve_1.swap(p_1, amount, min).map_err(AmmError::from)?;

        assert_non_zero!([res_1.deposit, res_1.withdraw]);
        self.deposit_token(is_x, res_1.deposit)?;

        let res_2 = curve_2.swap(p_2, res_1.withdraw, min).map_err(AmmError::from)?;

        assert_non_zero!([res_2.deposit, res_2.withdraw]);
        self.intermiadiate_exchange(is_x, intermidiate_is_x, res_2.deposit)?;
        self.withdraw_token(intermidiate_is_x, res_2.withdraw)?;

        Ok(())
    }

    pub fn deposit_token(
        &mut self,
        is_x: bool,
        amount: u64,
    ) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.user_x.to_account_info(), self.vault_x1.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info())
        };

        let accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info()
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(),
            accounts
        );

        transfer(ctx, amount)
    }

    pub fn intermiadiate_exchange(
        &mut self,
        is_x: bool,
        intermidiate_is_x: bool,
        amount: u64
    ) -> Result<()> {
        let (from, to) = match is_x {
            true => match intermidiate_is_x {
                true => (self.user_y.to_account_info(), self.vault_x2.to_account_info()),
                false => (self.user_y.to_account_info(), self.vault_z.to_account_info())
            }
            false => match intermidiate_is_x {
                true => (self.user_x.to_account_info(), self.vault_x2.to_account_info()),
                false => (self.user_x.to_account_info(), self.vault_z.to_account_info())
            }
        };

        let accounts = Transfer {
            from,
            to,
            authority: self.auth1.to_account_info()
        };

        let seeds = &[
            &b"auth"[..],
            &self.auth1.key.as_ref(),
            &[self.config1.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, amount)
    }

    pub fn withdraw_token(
        &mut self,
        intermidiate_is_x: bool,
        amount: u64,
    ) -> Result<()> {
        
        let (from, to) = match intermidiate_is_x {
            true => (self.vault_z.to_account_info(), self.user_y.to_account_info()),
            false => (self.vault_x2.to_account_info(), self.user_x.to_account_info())
        };

        let accounts = Transfer {
            from,
            to,
            authority: self.auth2.to_account_info()
        };

        let seeds = &[
            &b"auth"[..],
            &self.auth2.key.as_ref(),
            &[self.config2.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, amount)
    }
}



