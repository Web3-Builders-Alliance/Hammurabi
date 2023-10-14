use std::collections::BTreeMap;

use crate::errors::AmmError;
use crate::state::config::PoolConfig;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use solana_address_lookup_table_program::*;

#[derive(Accounts)]
pub struct InitializeTokenPair<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"config", mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump,
        space = PoolConfig::LEN
    )]
    pub config: Account<'info, PoolConfig>,

    /// CHECK: This is safe because it's just used to sign
    #[account(seeds = [b"auth", config.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    pub lookup_table: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeTokenPair<'info> {
    pub fn init(&mut self, bumps: &BTreeMap<String, u8>, authority: Option<Pubkey>) -> Result<()> {
        //require!(self.mint_x.key == new Pubkey("So11111111111111111111111111111111111111112") || mint_x == new Pubkey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"));
        //require!(self.initializer.key() == xxx) hardcode the initializer
        let (config_bump, auth_bump) = (
            *bumps.get("config").ok_or(AmmError::BumpError)?,
            *bumps.get("auth").ok_or(AmmError::BumpError)?,
        );

        //Is this right?

        // let mut new_addresses: Vec<Pubkey> = Vec::new();
        // new_addresses.push(self.mint_x.key());
        // new_addresses.push(self.mint_y.key());
        // new_addresses.push(self.vault_x.key());
        // new_addresses.push(self.vault_y.key());

        // let ix = solana_address_lookup_table_program::instruction::extend_lookup_table(
        //     self.intializer.key(),
        //     self.auth.key(),
        //     Some(self.creator.key()),
        //     new_addresses,
        // );
        // invoke(
        //     &ix,
        //     &[
        //         self.creator.to_account_info(),
        //         self.system_program.to_account_info(),
        //     ],
        // )?;

        self.config.init(authority, self.lookup_table.key());

        Ok(())
    }
}
