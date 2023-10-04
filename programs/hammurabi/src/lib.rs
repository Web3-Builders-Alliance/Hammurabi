use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod constants;
mod state;
mod errors;
mod helpers;

declare_id!("ammmDN4bVS1pFRNc9SoH1bFbdJJcmnsNgmeoU6KhLag");

#[program]
pub mod hammurabi {
    use super::*;

    pub fn initialize_pair(
        ctx: Context<InitializeTokenPair>, 
        authority: Option<Pubkey> // Update authority (if required)
    ) -> Result<()> {
        // Initialise our TokenPair config
        ctx.accounts.init(&ctx.bumps, authority)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64, // Amount of LP token to claim
        max_x: u64, // Max amount of X we are willing to deposit
        max_y: u64, // Max amount of Y we are willing to deposit
        expiration: i64,
    ) -> Result<()> {
        // Deposit liquidity to swap
        ctx.accounts.deposit(amount, max_x, max_y, expiration)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64, // Amount of liquidity tokens to burn
        min_x: u64, // Minimum amount of liquidity we are willing to receive
        min_y: u64, // Minimum amount of liquidity we are willing to receive
        expiration: i64,
    ) -> Result<()> {
        // Withdraw liquidity from swap
        ctx.accounts.withdraw(amount, min_x, min_y, expiration)
    }

    pub fn swap(
        ctx: Context<Swap>,
        is_x: bool,
        amount: u64, // Amount of tokens we deposit
        min: u64, // Minimum amount of tokens I'd be willing to withdraw
        expiration: i64
    ) -> Result<()> {
        // Swap Token X for Token Y or vice versa
        ctx.accounts.swap(is_x, amount, min, expiration)
    }

    pub fn token_to_token_swap(
        ctx: Context<TokenToTokenSwap>,
        amount: u64, // Amount of tokens we deposit
        min: u64, // Minimum amount of tokens I'd be willing to withdraw
        expiration: i64
    ) -> Result<()> {
        // Swap Token X for Token Y passing from another Token
        ctx.accounts.token_to_token_swap(amount, min, expiration)
    }

    pub fn lock(
        ctx: Context<Update>,
    ) -> Result<()> {
        ctx.accounts.lock()
    }

    pub fn unlock(
        ctx: Context<Update>,
    ) -> Result<()> {
        ctx.accounts.unlock()
    }
}
