use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, MintTo, transfer, mint_to};
use anchor_spl::associated_token::AssociatedToken;
use constant_product_curve::ConstantProduct;
use solana_program::{address_lookup_table_account, pubkey};
use crate::{assert_non_zero, assert_not_locked, assert_not_expired};
use crate::state::pool::Pool;
use crate::state::config::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Box<Account<'info, Mint>>,
    pub mint_y: Box<Account<'info, Mint>>,
    /// CHECK: This is safe because we check it later
    pub mint_lp: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: Box<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        associated_token::mint = config.mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = config.mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is safe because we check it later
    pub user_lp: UncheckedAccount<'info>,

    /// CHECK: just a pda for signing
    #[account(seeds = [b"auth", config.key().as_ref()], bump = config.auth_bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [
            b"config",
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    /// CHECK 
    pub lut_address: UncheckedAccount<'info>,
    /// CHECK
    pub liqudity_pool: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(
        &self,
        max_x: u64, // Max amount of X we are willing to deposit
        max_y: u64, // Max amount of Y we are willing to deposit
        expiration: i64,
    ) -> Result<()> {
        assert_not_locked!(self.config.locked);
        assert_not_expired!(expiration);
        assert_non_zero!([max_x, max_y]);

        //Look for the best pool to deposit in

        let info = self.lut_address.to_account_info();
        let mut data = info.try_borrow_mut_data()?;
        let mut reader = &data[..];        
        let result = solana_address_lookup_table_program::state::AddressLookupTable::deserialize(reader);

        //How to check how many address are into the lut?
        //How do i get the address in it
        
        //let addresses be an array of UncheckedAccount from result.addresses
        let mut lowest_fee_pool: UncheckedAccount<'_>; 
        let mut lowest_fee: u16 = 100;

        //for cycle from 0 to Lut Lenght
            self.lut_address = lowest_fee_pool;
            let info = self.lut_address.to_account_info();
            let mut data = info.try_borrow_mut_data()?;

            let mut reader = &data[..];
            let pool = Pool::try_deserialize(&mut reader)?;

            if pool.fee < lowest_fee {
                lowest_fee = pool.fee;
                lowest_fee_pool = self.lut_address;
            }
        //}

        self.mint_lp = pool.mint_lp;
        self.vault_x = pool.vault_x;
        self.vault_y = pool.vault_y;
        self.mint_lp = pool.mint_lp;


        //If they aren't initailized?

        if lowest_fee < 50 {

            //ADD CHECK!!!!
            
            //Deposit Function and Logic

            // let (x,y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
            //     true => (max_x, max_y),
            //     false => {
            //         let amounts = ConstantProduct::xy_deposit_amounts_from_l(
            //             self.vault_x.amount,
            //             self.vault_y.amount,
            //             self.mint_lp.supply,
            //             amount,
            //             6
            //         ).map_err(AmmError::from)?;
            //         (amounts.x, amounts.y)
            //     }
            // };
    
            // // Check for slippage
            // require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);
            // self.deposit_tokens(true, x)?;
            // self.deposit_tokens(false, y)?;
            // self.mint_lp_tokens(amount)

            todo!()
        } else {

            //ADD CHECK!!!!

            //Intialize a new pool
            //Deposit Function and Logic
            
            todo!()
        }
    } 

    pub fn deposit_tokens(
        &self,
        is_x: bool,
        amount:u64
    ) -> Result<()> {  
        let (from, to) = match is_x {
            true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info())
        };      
        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(ctx, amount)
    }

    pub fn mint_lp_tokens(
        &self,
        amount:u64
    ) -> Result<()> {        
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
            signer_seeds
        );
        mint_to(ctx, amount)
    }
}