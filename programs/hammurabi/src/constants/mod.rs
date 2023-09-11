use solana_program::{pubkey, pubkey::Pubkey};

pub const PUBKEY_L: usize = 32;
pub const U64_L: usize = 8;
pub const U16_L: usize = 2;
pub const BOOL_L: usize = 1;
pub const OPTION_L: usize = 1;
pub const U8_L: usize = 1;

pub const USDC_TOKEN: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
pub const SOL_WRAPPED: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const BASE_TOKENS: [Pubkey; 2] = [USDC_TOKEN, SOL_WRAPPED];
