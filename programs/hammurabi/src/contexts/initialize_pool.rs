use crate::errors::AmmError;
use crate::state::config::PoolAccount;
use crate::state::config::PoolConfig;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"lp", config.key().as_ref()],
        payer = initializer,
        bump,
        mint::decimals = 6,
        mint::authority = auth
    )]
    pub mint_lp: Account<'info, Mint>,

    #[account(
        mut,
        realloc = PoolConfig::LEN + (PoolAccount::LEN * config.pool as usize),
        realloc::payer = initializer,
        realloc::zero = false,
        seeds = [b"config", mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump,
    )]
    pub config: Account<'info, PoolConfig>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"pool", config.key().as_ref(), (config.pool + 1).to_le_bytes().as_ref()],
        bump,
        space = PoolAccount::LEN,
    )]
    pub pool: Account<'info, PoolAccount>,

    #[account(seeds = [b"auth", config.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializePool<'info> {
    pub fn init(&mut self) -> Result<()> {
        let new_pool = PoolAccount {
            amount_x: 0,
            amount_y: 0,
            fee: 0,
            mint_lp: self.mint_lp.key(),
        }; //Todo the Fee

        self.config.pool += 1;
        self.config.accounts.push(new_pool);

        Ok(())
    }
}
