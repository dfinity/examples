# DIP721 NFT Container

## Summary

Motoko example coming soon.

# Getting Started

## Install DFX
```
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

## Run Local Internet Computer
```
dfx start --background 
```

## Deploy DIP721 NFT canister to your local IC

```
dfx deploy --argument "(record {
  logo = record {
    logo_type = \"image/png\";
    data = \"\";
  };
  name = \"My DIP721\";
  symbol = \"DFXB\";
})"
```