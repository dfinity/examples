# NFT wallet

This is an NFT wallet example dapp that utilizes minted NFTs from the Rust dip721-nft-container. Among some of its essential features, the wallet can register NFTs, transfer out NFTs, and check how many NFTs it contains. This dapp includes a frontend UI for interaction. 

## Prerequisites

- [x] Install the [IC
  SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install). For local testing, `dfx >= 0.22.0` is required.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Step 1: Setup project environment

You can deploy the dapp using the `start.sh` script:

```bash
cd examples/rust/nft-wallet
./start.sh
```

This script installs dependencies as well as a local Internet Identity, then deploys the NFT wallet locally.

Alternatively, the dapp can be deployed manually with the commands:

```bash
dfx start --background
cd internet-identity
npm install
II_FETCH_ROOT_KEY=1 dfx deploy
cd ..
./deploy.sh
```

If you'd like to deploy on the mainnet, run the command:

```bash
./deploy.sh --network ic
```

## Step 2: Make calls against NFT wallet canister

For example, to to transfer an NFT use the command:

```bash
dfx canister call nftwallet transfer '(record {canister = principal "<NFT canister id>"; index = 1:nat64}, principal "<recipient canister id>", opt true)'
```
