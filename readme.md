# Hammurabi
A modern, Anchor-based AMM

# Setup
Some basic ground rules in order to keep our tests, `Anchor.toml` and `lib.rs` in sync:

### 1. Create a symlink to your wallet
Make sure to create a symlink to your Solana keypair in the root directory, like so:
`ln -s /solana-id.json /Users/dean/.config/solana/id.json ./wallet.json`

While this wallet won't be committed to the repo as it is currently included in the `.gitignore`, so your funds should be `#SAFU`, it is recommended to spin up a new, specific wallet that you don't care about just for testing just in case.

### 2. Deploy and use the existing keypair on Localnet
We are syncing the locally used keypair for `ammmDN4bVS1pFRNc9SoH1bFbdJJcmnsNgmeoU6KhLag`. All keypairs in the `/target/deploy` directory should be considered doxed and unsafe for public use.

We will deploy to devnet and mainnet with a different set of keys which will be determined at a later date and held by the POC responsible for deployments.
