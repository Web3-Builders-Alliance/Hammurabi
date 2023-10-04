use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,           // Token X Mint
    pub x_locked: u64,
    pub mint_y: Pubkey,           // Token Y Mint
    pub y_locked: u64,
    pub locked: bool,
    pub config_bump: u8,
    pub lut_address: Pubkey,
    pub lut_bump: u8,
    pub auth_bump: u8
}

impl Config {
    pub const LEN: usize = 8 + (U64_L * 2) + OPTION_L + (PUBKEY_L * 4) + BOOL_L + (U8_L * 3);

    pub fn init(
        &mut self, 
        authority: Option<Pubkey>, 
        mint_x: Pubkey,
        mint_y: Pubkey,
        config_bump: u8,
        lut_address: Pubkey,
        lut_bump: u8,
        auth_bump: u8
    ) {
        self.authority = authority;
        self.mint_x = mint_x;
        self.x_locked = 0;
        self.mint_y = mint_y;
        self.y_locked = 0;
        self.locked = false;
        self.config_bump = config_bump;
        self.lut_address = lut_address;
        self.lut_bump = lut_bump;
        self.auth_bump = auth_bump;
    }
}