# NFT WALLET

This is the NFT Wallet example dapp that runs utilizing minted NFTs from the Rust dip721-nft-container. Among some of its essential features, the wallet can register NFTs, transfer out NFTs and check how many NFTs it owns.

NFT Wallet dapp has a frontend UI available!

## Get started

Simple way to start is running:

```bash
./start.sh
```

Start script installs dependencies as well as a local internet-identity. Then it goes on to deploy the NFT Wallet locally.

If you prefer to manually deploy

```bash
dfx start --background
cd internet-identity
npm install
II_ENV=development dfx deploy
cd ..
./deploy.sh
```

If you'd like to deploy on the IC network run

```bash
./deploy.sh --network ic
```

## Make calls against NFT wallet canister

Example to Transfer an NFT

```bash
dfx canister call nftwallet transfer '(record {canister = principal "<NFT canister id>"; index = 1:nat64}, principal "<recipient canister id>", opt true)'
```
