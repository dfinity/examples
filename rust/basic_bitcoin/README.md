# Basic Bitcoin

This example demonstrates how a canister can receive and send bitcoin on the Internet Computer, including support for legacy (P2PKH), SegWit (P2WPKH), and Taproot (P2TR) address types.

## Architecture

For a deeper understanding of the ICP ↔ Bitcoin integration, see the [Bitcoin integration concepts](https://docs.internetcomputer.org/concepts/chain-fusion/bitcoin).

This example integrates with the Internet Computer's built-in:

- Threshold ECDSA ([`ecdsa_public_key`](https://docs.internetcomputer.org/references/ic-interface-spec/management-canister/#ic-ecdsa_public_key), [`sign_with_ecdsa`](https://docs.internetcomputer.org/references/ic-interface-spec/management-canister/#ic-sign_with_ecdsa)) — derives P2PKH addresses and signs transactions spending from them
- Threshold Schnorr ([`schnorr_public_key`](https://docs.internetcomputer.org/references/ic-interface-spec/management-canister/#ic-schnorr_public_key), [`sign_with_schnorr`](https://docs.internetcomputer.org/references/ic-interface-spec/management-canister/#ic-sign_with_schnorr)) — derives P2TR addresses (BIP340/341) and signs Taproot transactions
- [Bitcoin canister](https://docs.internetcomputer.org/references/protocol-canisters/#bitcoin-canisters) — queries balances, UTXOs, fee percentiles, and block data; submits signed transactions to the Bitcoin network

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) v1.85+ with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Docker](https://docs.docker.com/get-docker/) (required to run the custom network launcher image that bundles bitcoind)
- On macOS, a `clang` with WASM support is required to compile the `secp256k1-sys` C library for the `wasm32-unknown-unknown` target. Xcode's bundled clang does not include the WASM backend. Install the [Homebrew LLVM](https://formulae.brew.sh/formula/llvm) and add it to your PATH:
  ```bash
  brew install llvm
  export PATH="$(brew --prefix llvm)/bin:$PATH"
  ```
  Add the `export` line to your shell profile (`~/.zshrc` or `~/.bashrc`) to make it permanent.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_bitcoin
```

### Build the network launcher image

The local network bundles bitcoind inside a custom Docker image. Build it once before starting the network:

```bash
./build-image.sh
```

### Deploy locally and test

```bash
icp network start -d
icp deploy --cycles 30t
bash test.sh
icp network stop
```

> If tests fail with an out-of-cycles error, top up the canister and retry:
> ```bash
> icp canister top-up --amount 30t backend
> ```

### Deploy to the IC network

The `ic` environment deploys to IC mainnet connected to Bitcoin testnet4, using `test_key_1`:

```bash
icp deploy -e ic --cycles 30t
```

> To deploy to Bitcoin mainnet, change the `init_args` for the `ic` environment in `icp.yaml` from `testnet` to `mainnet`. The canister automatically selects `key_1` (the production threshold signing key) when initialized with the `mainnet` variant.

## Generating Bitcoin addresses

The example demonstrates how to generate and use the following address types:

1. **P2PKH (Legacy)** using ECDSA and `sign_with_ecdsa`
2. **P2WPKH (SegWit v0)** using ECDSA and `sign_with_ecdsa`
3. **P2TR (Taproot, key-path-only)** using Schnorr keys and `sign_with_schnorr`
4. **P2TR (Taproot, script-path-enabled)** commits to a script allowing both key path and script path spending

```bash
icp canister call backend get_p2pkh_address '()'
# or: get_p2wpkh_address, get_p2tr_key_path_only_address, get_p2tr_script_path_enabled_address
```

## Funding and sending bitcoin: a complete walkthrough

This walkthrough shows how to fund an address, check its balance, send bitcoin to another address, and confirm the transfer — using the bundled `bitcoind` in regtest mode.

> **Coinbase maturity:** In Bitcoin, newly mined block rewards (coinbase UTXOs) cannot be spent until 100 more blocks have been mined on top. Mine at least 101 blocks upfront so the first reward is immediately spendable.

### Step 1 — Get the canister's address and the container ID

```bash
CONTAINER=$(docker ps --filter "ancestor=icp-cli-network-launcher-bitcoin" --format "{{.ID}}" | head -1)
ADDR=$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"')
echo "Address: $ADDR"
```

### Step 2 — Mine 101 blocks to fund the address

Mining 101 blocks ensures the first block reward (50 BTC) is past the coinbase maturity threshold and immediately spendable.

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 101 "$ADDR"
```

### Step 3 — Check the balance

The IC Bitcoin integration syncs new blocks continuously. If the balance shows 0, wait a few seconds and retry.

```bash
icp canister call backend get_balance "(\"$ADDR\")"
# Expected: (505_000_000_000 : nat64)  — 101 blocks × 50 BTC each
```

### Step 4 — Send bitcoin

```bash
DEST="bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt"
icp canister call backend send_from_p2pkh_address "(record {
  destination_address = \"$DEST\";
  amount_in_satoshi = 4321;
})"
# Returns the transaction ID
```

The transaction is now broadcast to `bitcoind`'s mempool. The destination balance will remain 0 until it is confirmed in a block.

### Step 5 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

### Step 6 — Verify the destination received the funds

```bash
icp canister call backend get_balance "(\"$DEST\")"
# Expected: (4_321 : nat64)
```

## Other send endpoints

The same pattern (fund → send → mine confirmation block → verify) applies to the other address types:

- `send_from_p2wpkh_address`
- `send_from_p2tr_key_path_only_address`
- `send_from_p2tr_script_path_enabled_address_key_spend`
- `send_from_p2tr_script_path_enabled_address_script_spend`

Each endpoint internally estimates fees, selects UTXOs, builds a transaction, signs it using ECDSA or Schnorr, and broadcasts it via `bitcoin_send_transaction`.

When the canister is deployed on IC mainnet, you can track testnet transactions on [mempool.space](https://mempool.space/testnet4/).

## Querying UTXOs

You can inspect the UTXOs held at any Bitcoin address:

```bash
icp canister call backend get_utxos "(\"$ADDR\")"
```

This returns all unspent outputs at the address — useful for verifying that funds arrived or for debugging balance issues. The response includes each outpoint (txid + vout index), value in satoshis, and confirmation height.

## Retrieving blockchain info

You can query the current state of the Bitcoin blockchain:

```bash
icp canister call backend get_blockchain_info '()'
```

This calls `get_blockchain_info` on the Bitcoin canister and returns the tip height, block hash, timestamp, difficulty, and total UTXO count. It is useful for monitoring the state of the Bitcoin network from your canister.

## Retrieving block headers

You can query historical block headers:

```bash
icp canister call backend get_block_headers '(10: nat32, null)'
# or a range:
icp canister call backend get_block_headers '(10: nat32, opt (11: nat32))'
```

This calls `bitcoin_get_block_headers`, which is useful for blockchain validation or light client logic.

## Notes on implementation

This example implements several important patterns for Bitcoin integration:

- **Derivation paths**: Keys are derived using structured derivation paths according to BIP-32, ensuring reproducible key generation.
- **Key caching**: Optimization is used to avoid repeated calls to `get_ecdsa_public_key` and `get_schnorr_public_key`.
- **Manual transaction construction**: Transactions are assembled and signed manually, ensuring maximum flexibility in construction and fee estimation.
- **Cost optimization**: When testing on mainnet, the [chain-key testing canister](https://github.com/dfinity/chainkey-testing-canister) can be used to save on costs for calling the threshold signing APIs.

## Security considerations and best practices

This example is provided for educational purposes and is not production-ready. It is important to consider security implications when developing applications that interact with Bitcoin or other cryptocurrencies. The code has **not been audited** and may contain vulnerabilities or security issues.

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

- Certify query responses if they are relevant for security, since the app offers a method to read balances.
- Use a decentralized governance system like SNS to give a canister a decentralized controller, since decentralized control may be essential for canisters holding bitcoins on behalf of users.
