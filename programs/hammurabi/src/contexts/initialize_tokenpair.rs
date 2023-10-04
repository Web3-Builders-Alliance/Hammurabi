use std::collections::BTreeMap;

use anchor_lang::{prelude::*};
use anchor_spl::token::Mint;
use crate::errors::AmmError;
use crate::state::config::Config;
use solana_address_lookup_table_program::{self, ID};

#[derive(Accounts)]
pub struct InitializeTokenPair<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init, 
        payer = initializer, 
        seeds = [b"config", mint_x.key().as_ref(), mint_y.key().as_ref()], 
        bump,
        space = Config::LEN
    )]
    pub config: Account<'info, Config>,
    /// CHECK: This is safe because it's just used to sign
    #[account(seeds = [b"auth", config.key().as_ref()], bump)]
    pub auth: UncheckedAccount<'info>,
    pub lookup_table: AccountInfo<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> InitializeTokenPair<'info> {
    pub fn init(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        authority: Option<Pubkey>        
    ) -> Result<()> {
        //require!(mint_x == new Pubkey("So11111111111111111111111111111111111111112") || mint_x == new Pubkey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"));
        
        let (lut_bump, config_bump, auth_bump) = (
            *bumps.get("lut").ok_or(AmmError::BumpError)?,
            *bumps.get("config").ok_or(AmmError::BumpError)?,
            *bumps.get("auth").ok_or(AmmError::BumpError)?,
        );

        let seeds = &[
            &b"auth"[..],
            &self.config.key().clone().to_bytes(),
            &[self.config.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        //Create the Look out Table for all the pool
        let (ix, lut_address) = solana_address_lookup_table_program::instruction::create_lookup_table(
            self.auth.key(),
            self.initializer.key(),
            Clock::get()?.slot,
        );
        solana_program::program::invoke_signed(
            &ix,
            &[
                self.lookup_table.clone(),
                self.auth.clone().to_account_info(),
                self.initializer.clone().to_account_info(),
                self.system_program.clone().to_account_info(),
            ],
            signer_seeds,
        )?;
        
        self.config.init(
            authority,
            self.mint_x.key(),
            self.mint_y.key(),
            config_bump,
            lut_address,
            lut_bump,
            auth_bump
        );

        Ok(())
    }
}