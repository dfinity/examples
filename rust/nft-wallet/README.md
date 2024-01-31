# NFT wallet

## Overview

This is an NFT wallet example dapp that utilizes minted NFTs from the Rust dip721-nft-container. Among some of its essential features, the wallet can register NFTs, transfer out NFTs and check how many NFTs it contains. This dapp includes a frontend UI for interaction. 

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.


### Step 1: You can deploy the dapp using the `start.sh` script:

```bash
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

### Step 2: If you'd like to deploy on the IC network run the command:

```bash
./deploy.sh --network ic
```

### Step 3: Make calls against NFT wallet canister:

For example, to to transfer an NFT use the command:

```bash
dfx canister call nftwallet transfer '(record {canister = principal "<NFT canister id>"; index = 1:nat64}, principal "<recipient canister id>", opt true)'
```
