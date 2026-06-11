# Basic Bitcoin

This example demonstrates how a canister smart contract can send and receive Bitcoin on the Internet Computer. It showcases the ECDSA API, Schnorr API (BIP340/BIP341), and Bitcoin API, supporting three address types: P2PKH, P2TR key-only spend, and P2TR with script path.

For a deeper understanding of the ICP <> BTC integration, see the [Bitcoin integration documentation](https://internetcomputer.org/docs/current/developer-docs/multi-chain/bitcoin/overview).

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- Docker (for local testing with bitcoind)

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/basic_bitcoin
```

### Local deployment (with bitcoind)

The local environment uses a self-contained Docker image that bundles `bitcoind` in regtest mode alongside the IC network launcher. Build the image first:

```bash
make build-image
```

Then deploy and test:

```bash
icp network start -d
icp deploy --cycles 30t
make test
icp network stop
```

> If tests fail with an out-of-cycles error, run `make topup` to add 30 trillion cycles to the backend canister and retry.

### Staging deployment (IC testnet)

```bash
icp deploy -e staging
```

### Production deployment (IC mainnet)

```bash
icp deploy -e production
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
icp canister call backend get_utxos '("YOUR_BITCOIN_ADDRESS")'
icp canister call backend get_current_fee_percentiles '()'
```

### Sending Bitcoin

```bash
icp canister call backend send_from_p2pkh_address '(record { destination_address = "DEST_ADDRESS"; amount_in_satoshi = 4321 })'
icp canister call backend send_from_p2tr_key_only_address '(record { destination_address = "DEST_ADDRESS"; amount_in_satoshi = 4321 })'
icp canister call backend send_from_p2tr_address_key_path '(record { destination_address = "DEST_ADDRESS"; amount_in_satoshi = 4321 })'
icp canister call backend send_from_p2tr_address_script_path '(record { destination_address = "DEST_ADDRESS"; amount_in_satoshi = 4321 })'
```

### Local testing with bitcoind JSON-RPC

For local testing, the Docker-based network launcher exposes the bitcoind JSON-RPC on port 18443. Mine blocks to a canister address using `curl`:

```bash
# Get the P2PKH address
ADDR=$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"')

# Mine 101 blocks to that address (provides spendable funds)
curl -s -X POST http://ic-btc-integration:ic-btc-integration@localhost:18443 \
  -H 'Content-Type: application/json' \
  -d "{\"jsonrpc\":\"1.0\",\"method\":\"generatetoaddress\",\"params\":[101,\"$ADDR\"]}"

# Wait for the IC to sync the blocks, then check balance
sleep 5
icp canister call backend get_balance "(\"$ADDR\")"
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
