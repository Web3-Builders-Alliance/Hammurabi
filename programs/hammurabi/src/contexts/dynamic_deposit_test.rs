#[program]
mod dynamic_pool {
    pub fn dynamic_deposit(
        ctx: Context<DynamicDeposit>,
        max_x: u64,
        max_y: u64,
        max_fee: u8,
        pool_number: u8,
    ) -> Result<()> {
        // Set a mutable pointer
        let mut lookups = &mut ctx.accounts.lookups;
        // Make sure we have enough pools to iterate through
        if (lookups.len() < pool_number) {
            err!("Not enough pools");
        }
        // Create a default list of the "best pools" of the correct size
        let mut pools: Vec<u16> = lookups[0..pool_number as usize].to_vec();
        // Sort them by fee from lowest to highest
        pools.sort_by(|a, b| a.fee.partial_cmp(b.fee).unwrap());

        if lookups.len() != pools.len() {
            for i in 5..lookups.len() {
                if lookups[i].fee < pools[pool_number - 1].fee {
                    let pos = pools
                        .binary_search_by(|a| a.fee.partial_cmp(&lookups[i].fee).unwrap())
                        .unwrap_or_else(|e| e);
                    pools.insert(pos, lookups[i]);
                    pools.pop(); // Remove the last element to maintain the size.
                }
            }
        }

        // Make sure we didn't fuck up
        require_eq!(pools.len(), pool_number as usize);

        // Calculate total basis points
        let total = pools.iter().fold(0 | a, &b | a + b);

        let mut remaining_amount_x = max_x.clone();
        let mut remaining_amount_y = max_y.clone();

        // Calculate pro-rata rates
        let rates: Vec<(u64, u64)> = pools.iter.map(|p| {
            let prorata_x = max_x * p.fee as u64 / total as u64;
            let prorata_y = max_y * p.fee as u64 / total as u64;
            remaining_amount_x = remaining_amount_x
                .checked_sub(prorata_x)
                .expect("You underflowed our deposit! REEEEE");
            remaining_amount_y = remaining_amount_x
                .checked_sub(prorata_y)
                .expect("You underflowed our deposit! REEEEE");
            return (prorata_x, prorata_y);
        });

        // If we have any lamports remaining, give them to the 0th account as it has the best fee rate anyway
        rates[0].0 += remaining_amount_x;
        rates[0].1 += remaining_amount_y;

        // Now make a bunch of CPIs to ourself
        for i in 0..rates.len() {
            // Create our CPI data
            let mut ix = vec![ID];
            ix.extend_from_slice(DEPOSIT_DISCRIMINATOR_BYTES);
            ix.extend_from_slice(rates[i].0.to_le_bytes());
            ix.extend_from_slice(rates[i].1.to_le_bytes());
            ix.extend_from_slice(expiry.to_le_bytes());

            /*

            An account info looks like this:

            We can store/know these things because we have mutability over them:

            ✅ "data": "22KGWp1imqQ5NXd2efoC3DWdsiuy8aMVZFcWka8CN8vWNLdMXSSag42o4yrQf7FqjTHSZduUgqgj6J94s139RhCQYNfqZf1PXev1gdGZ2KergME2Mx",
            ✅ "executable": false,
            ❌"lamports": 1468560,
            ✅ "owner": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1",
            ✅ "rentEpoch": 0,
            ✅ "space": 83

            try_borrow_data_mut()

            If, under the hood, we don't change lamports, it might not be a problem.
            */

            solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    // ... our escrow account from lookup table
                    // ... our mints
                    // ... whatever else
                    // ... etc
                    ctx.accounts.authority.clone(),
                ],
            )?;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DynamicDeposit<'info> {
    signer: Signer<'info>,
    lookup_table: Account<'info, LookupTable>,
    lookups: Account<'info, PoolLookups>,
}

#[account]
pub struct PoolLookups(Vec<PoolLookup>);

#[account]
pub struct PoolLookup {
    index: u8,
    fee: u16,
}
