#[program]
mod dynamic_pool {
    pub fn dynamic_deposit(
        ctx: Context<DynamicDeposit>,
        amount: u64,
        max_x: u64,
        max_y: u64,
        expiration: i64,
    ) -> Result<()> {
        // Set a mutable pointer
        let mut lookups = &mut self.accounts.config.accounts;

        // Create a default list of the "best pools" of the correct size
        let mut pools: Vec<u16> = lookups[0..self.accounts.config.pool as usize].to_vec();
        // Sort them by fee from lowest to highest
        pools.sort_by(|a, b| a.fee.partial_cmp(b.fee).unwrap());

        if pools[0].fee > 30 {
            //Create instruction to Init new Pool

            let mut ix = vec![ID]; //To add
            ix.extend_from_slice(POOL_INIT_DISCRIMINATOR_BYTES); //To find right?

            let info = self.accounts.config.lut_info.to_account_info();
            let mut data = info.try_borrow_mut_data()?;
            let mut reader = &data[..];
            let result =
                solana_address_lookup_table_program::state::AddressLookupTable::deserialize(reader)
                    .unwrap();

            let lut_addresses = result.addresses.into_iter();
            let lut_addresses_lenght = result.addresses.len();

            let mint_x = lut_addresses[0];
            let mint_y = lut_addresses[1];

            solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    mint_x.clone(),
                    mint_y.clone(),
                    pool[0].mint_lp.clone(),
                    self.accounts.config.clone(),
                    self.accounts.config.accounts[0], //HOWWWWW
                    ctx.accounts.config.authority.clone(),
                    //Do i need to put into token_program, associated_token_program, system_program?
                ],
            )?;

            //Create instruction to Deposit to existing pool
        } else {
            //Create instruction to Deposit to existing pool

            let mut ix = vec![ID]; //To add
            ix.extend_from_slice(DEPOSIT_DISCRIMINATOR_BYTES); //To find right?
            ix.extend_from_slice(amount.to_le_bytes());
            ix.extend_from_slice(max_x.to_le_bytes());
            ix.extend_from_slice(max_y.to_le_bytes());
            ix.extend_from_slice(expiration.to_le_bytes());

            let info = self.accounts.config.lut_info.to_account_info();
            let mut data = info.try_borrow_mut_data()?;
            let mut reader = &data[..];
            let result =
                solana_address_lookup_table_program::state::AddressLookupTable::deserialize(reader)
                    .unwrap();

            let lut_addresses = result.addresses.into_iter();
            let lut_addresses_lenght = result.addresses.len();

            let mint_x = lut_addresses[0];
            let mint_y = lut_addresses[1];
            let vault_x = lut_addresses[2];
            let vault_y = lut_addresses[3];

            let user_x = todo!();
            let user_y = todo!();
            let user_lp = todo!();

            solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    mint_x.clone(),
                    mint_y.clone(),
                    pool[0].mint_lp.clone(),
                    vault_x.clone(),
                    vault_y.clone(),
                    user_x.clone(),
                    user_y.clone(),
                    user_lp.clone(),
                    ctx.accounts.config.authority.clone(),
                    ctx.accounts.config.clone(),
                    //Do i need to put into token_program, associated_token_program, system_program?
                ],
            )?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct DynamicDeposit<'info> {
    signer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"config", mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump,
    )]
    pub config: Account<'info, PoolConfig>,
}
