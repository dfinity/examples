# Basic Bitcoin

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/basic_bitcoin)

## Overview

This example demonstrates how a canister smart contract can send and receive Bitcoin on the Internet Computer. It showcases the ECDSA API, Schnorr API (BIP340/BIP341), and Bitcoin API, supporting three address types: P2PKH, P2TR key-only spend, and P2TR with script path.

For a deeper understanding of the ICP <> BTC integration, see the [Bitcoin integration documentation](https://internetcomputer.org/docs/current/developer-docs/multi-chain/bitcoin/overview).

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/basic_bitcoin
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Generating Bitcoin addresses

Bitcoin has different types of addresses (e.g. P2PKH, P2TR). These addresses can be generated from an ECDSA public key or a Schnorr ([BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki), [BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki)) public key. The example showcases three address types:

1. A [P2PKH address](https://en.bitcoin.it/wiki/Transaction#Pay-to-PubkeyHash) using the ECDSA API.
2. A [P2TR address](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki) where funds can be spent using the internal key only (P2TR key path spend with unspendable script tree).
3. A [P2TR address](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki) where funds can be spent using either the internal key or a script path key.

```bash
icp canister call backend get_p2pkh_address '()'
icp canister call backend get_p2tr_key_only_address '()'
icp canister call backend get_p2tr_address '()'
```

## Checking balance and sending Bitcoin

```bash
icp canister call backend get_balance '("YOUR_BITCOIN_ADDRESS")'
icp canister call backend send_from_p2pkh_address '(record { destination_address = "DEST_ADDRESS"; amount_in_satoshi = 4321 })'
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP dapp.
