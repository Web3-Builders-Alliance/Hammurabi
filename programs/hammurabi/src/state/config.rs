use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct PoolConfig {
    pub authority: Option<Pubkey>,
    pub locked: bool,
    pub lut_info: Pubkey, //Mint_x, Mint_y, Vault_x, Vault_y
    pub accounts: Vec<PoolAccount>,
    pub pool: u8,
}

impl PoolConfig {
    pub const LEN: usize = OPTION_L + BOOL_L + PUBKEY_L + 4;

    pub fn init(&mut self, authority: Option<Pubkey>, lut_info: Pubkey) {
        self.authority = authority;
        self.locked = false;
        self.lut_info = lut_info;
        self.accounts = Vec::new();
        self.pool = 0;
    }
}

#[account]
pub struct PoolAccount {
    pub amount_x: u64,
    pub amount_y: u64,
    pub fee: u16,
    pub mint_lp: Pubkey,
}

impl PoolAccount {
    pub const LEN: usize = (U64_L * 2) + U16_L + PUBKEY_L;
}
