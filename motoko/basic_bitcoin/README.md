# Basic Bitcoin

This example demonstrates how a canister can send and receive Bitcoin on the Internet Computer using threshold ECDSA and Schnorr signatures. It covers three address types (P2PKH, P2TR key-path, P2TR script-path), querying balances and UTXOs, reading chain state, and sending transactions.

For a deeper understanding of the ICP ↔ Bitcoin integration, see the [Bitcoin integration concepts](https://docs.internetcomputer.org/concepts/chain-fusion/bitcoin).

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`
- Docker (required for local testing — bundles the IC network launcher + `bitcoind`)

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/basic_bitcoin
```

### Local deployment

The local environment uses a self-contained Docker image (`icp-cli-network-launcher-bitcoin`) that runs `bitcoind` in regtest mode alongside the IC network. Build it once:

```bash
make build-image
```

Then deploy and run tests:

```bash
icp network start -d
icp deploy --cycles 30t
make test
icp network stop
```

> If tests fail with an out-of-cycles error, run `make topup` and retry.

### Staging (IC mainnet, Bitcoin testnet4)

```bash
icp deploy -e staging
```

### Production (IC mainnet, Bitcoin mainnet)

```bash
icp deploy -e production
```

## Environments

| Environment | IC network | Bitcoin network | Key |
|-------------|-----------|----------------|-----|
| `local` | local (PocketIC) | regtest | `test_key_1` |
| `staging` | IC mainnet | testnet4 | `test_key_1` |
| `production` | IC mainnet | mainnet | `key_1` |

## Available functions

### Address generation

```bash
icp canister call backend get_p2pkh_address '()'
icp canister call backend get_p2tr_key_only_address '()'
icp canister call backend get_p2tr_address '()'
```

### Chain queries

```bash
icp canister call backend get_balance '("YOUR_ADDRESS")'
icp canister call backend get_utxos '("YOUR_ADDRESS")'
icp canister call backend get_current_fee_percentiles '()'
icp canister call backend get_block_headers '(0 : nat32, null)'
icp canister call backend get_blockchain_info '()'
```

### Sending Bitcoin

```bash
icp canister call backend send_from_p2pkh_address \
  '(record { destination_address = "DEST"; amount_in_satoshi = 4321 })'
icp canister call backend send_from_p2tr_key_only_address \
  '(record { destination_address = "DEST"; amount_in_satoshi = 4321 })'
icp canister call backend send_from_p2tr_address_key_path \
  '(record { destination_address = "DEST"; amount_in_satoshi = 4321 })'
icp canister call backend send_from_p2tr_address_script_path \
  '(record { destination_address = "DEST"; amount_in_satoshi = 4321 })'
```

### Local testing: mine blocks and fund an address

```bash
# Get a P2PKH address
ADDR=$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"')

# Mine 101 blocks to that address via the bundled bitcoind
CONTAINER=$(docker ps --filter "ancestor=icp-cli-network-launcher-bitcoin" --format "{{.ID}}" | head -1)
docker exec "$CONTAINER" bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 101 "$ADDR"

# Wait for the IC to sync, then check balance
sleep 5
icp canister call backend get_balance "(\"$ADDR\")"
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
