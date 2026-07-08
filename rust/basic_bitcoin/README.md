# Basic Bitcoin

This example demonstrates how a canister can receive and send bitcoin on the Internet Computer, including support for legacy (P2PKH), SegWit (P2WPKH), and Taproot (P2TR) address types.

This example also includes how to work with Bitcoin assets such as Ordinals, Runes, and BRC-20 tokens.

See also the [Motoko version](../../motoko/basic_bitcoin).

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

## Bitcoin assets

Bitcoin's scripting capabilities enable various digital assets beyond simple transfers. This example demonstrates how to create and interact with three major Bitcoin asset protocols from an ICP canister:

- **Ordinals**: Inscribe arbitrary data onto individual satoshis
- **Runes**: Create fungible tokens using `OP_RETURN` outputs
- **BRC-20**: Build fungible tokens on top of Ordinals using JSON

### Prerequisites for Bitcoin assets

All Bitcoin assets rely on off-chain indexing since the Bitcoin protocol doesn't natively support querying these assets. The `ord` CLI tool is the standard indexer for Bitcoin assets like Ordinals and Runes.

Install `ord` using a package manager. For example, on macOS:

```bash
brew install ord
```

For other platforms, see the [ord repository](https://github.com/ordinals/ord) for installation instructions.

Make sure the `$CONTAINER` variable is set before running any of the steps below. If you haven't done so yet from the Bitcoin walkthrough above:

```bash
CONTAINER=$(docker ps --filter "ancestor=icp-cli-network-launcher-bitcoin" --format "{{.ID}}" | head -1)
```

> **Note:** This repository includes a [default ord config file](./ord.yaml) that matches the also provided [bitcoin config file](./bitcoin.conf).

> **Important — Bitcoin Configuration:** To work with Bitcoin assets, make sure bitcoind is configured to accept non-standard transactions by including this setting in your `bitcoin.conf`:
>
> ```
> acceptnonstdtxn=1
> ```

## Inscribe an Ordinal

[Ordinals](https://ordinals.com) is a protocol that allows inscribing arbitrary data (text, images, etc.) onto individual satoshis, creating unique digital artifacts on Bitcoin. Each inscription is permanently stored in the Bitcoin blockchain using a two-transaction commit/reveal process.

### Step 1 — Start the ord server (separate terminal)

```bash
ord --config-dir . server
```

### Step 2 — Fund a Taproot address

```bash
ADDR=$(icp canister call backend get_p2tr_key_path_only_address '()' | grep -o '"[^"]*"' | tr -d '"')
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 101 "$ADDR"
```

### Step 3 — Create the inscription

```bash
TXID=$(icp canister call backend inscribe_ordinal '("Hello Bitcoin")' | grep -o '"[^"]*"' | tr -d '"')
echo "Reveal txid: $TXID"
```

### Step 4 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

### Step 5 — View in the ord explorer

```bash
echo "http://127.0.0.1/inscription/${TXID}i0"
```

- All inscriptions: `http://127.0.0.1/inscriptions`

> **Note:** The homepage at `http://127.0.0.1/` shows a "Latest Inscriptions" section that may appear empty — use `/inscriptions` instead.

## Runes

[Runes](https://docs.ordinals.com/runes.html) is a fungible token protocol that embeds token metadata directly into Bitcoin transactions using `OP_RETURN` outputs. Unlike Ordinals, Runes are created in a single transaction and support standard fungible token operations. Two operations make up the full lifecycle: **etch** → **transfer**.

### Etch a Rune

#### Step 1 — Start the ord server (separate terminal)

```bash
ord --config-dir . server
```

#### Step 2 — Fund a Taproot address

```bash
ADDR=$(icp canister call backend get_p2tr_key_path_only_address '()' | grep -o '"[^"]*"' | tr -d '"')
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 101 "$ADDR"
```

#### Step 3 — Etch the Rune

The name must be uppercase and between 1–28 characters. The Runes protocol reserves short names until a future Bitcoin block height — names with 12 or more characters are available immediately in regtest. Using a shorter name (e.g. 7 characters) means the rune won't receive an active ID until block ~87,500+ is mined, making it unusable for transfers.

> **Turbo mode**: The `etch_rune` implementation sets `turbo: true`, which opts the rune into future ord protocol upgrades. It does **not** bypass the name unlock schedule.

```bash
TXID=$(icp canister call backend etch_rune '("BASICBITCOIN")' | grep -o '"[^"]*"' | tr -d '"')
echo "Rune txid: $TXID"
```

#### Step 4 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 5 — Decode the Runestone to verify

```bash
ord --config-dir . decode --txid "$TXID"
```

#### Step 6 — View in the ord explorer

```bash
echo "http://127.0.0.1/rune/BASICBITCOIN"
```

- All runes: `http://127.0.0.1/runes`

> **Note:** The `/runes` listing page may appear empty for the same reason as `/` for inscriptions — use the direct URL above instead.

The Rune is now etched with 1,000,000 tokens minted to your address.

### Transfer a Rune

#### Step 1 — Look up the rune ID

The rune ID (block height : transaction index) is required to identify which rune to transfer. Fetch it from the ord JSON API:

```bash
RUNE_ID=$(curl -s -H "Accept: application/json" http://127.0.0.1/rune/BASICBITCOIN | \
  tr -d '[:space:]' | grep -o '"id":"[0-9]*:[0-9]*"' | cut -d'"' -f4)
RUNE_BLOCK=$(echo "$RUNE_ID" | cut -d: -f1)
RUNE_TX=$(echo "$RUNE_ID" | cut -d: -f2)
echo "Rune ID: $RUNE_BLOCK:$RUNE_TX"
```

> **Why 12+ characters?** The ord indexer only assigns a rune ID once the rune's name reaches its unlock height. Names shorter than 12 characters have unlock heights of 17,500–105,000+ blocks in regtest, which is impractical. With a 12-character name like `BASICBITCOIN`, the rune is immediately active and the JSON response includes its `"id"` field.

#### Step 2 — Transfer rune tokens

```bash
DEST="bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt"
TRANSFER_TXID=$(icp canister call backend transfer_rune "(record {
  rune_id_block = ${RUNE_BLOCK}: nat64;
  rune_id_tx = ${RUNE_TX}: nat32;
  amount = 100000: nat64;
  destination_address = \"$DEST\";
})" | grep -o '"[^"]*"' | tr -d '"')
echo "Transfer txid: $TRANSFER_TXID"
```

#### Step 3 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 4 — Verify the transfer

Decode the transfer transaction to confirm the Runestone edict is correct:

```bash
ord --config-dir . decode --txid "$TRANSFER_TXID"
```

The output should contain a Runestone with an edict assigning 100,000 tokens to output 0 (the recipient). The remaining 900,000 tokens stay in the change output at the canister's address.

## BRC-20 tokens

[BRC-20](https://domo-2.gitbook.io/brc-20-experiment/) is a token standard built on top of Ordinals that uses structured JSON payloads to create fungible tokens. Three operations make up the full lifecycle: **deploy** → **mint** → **transfer**.

### Deploy a BRC-20 token

#### Step 1 — Start the ord server (separate terminal)

```bash
ord --config-dir . server
```

#### Step 2 — Fund a Taproot address

```bash
ADDR=$(icp canister call backend get_p2tr_key_path_only_address '()' | grep -o '"[^"]*"' | tr -d '"')
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 101 "$ADDR"
```

#### Step 3 — Deploy the BRC-20 token

The ticker must be exactly 4 characters:

```bash
DEPLOY_TXID=$(icp canister call backend inscribe_brc20 '("DEMO")' | grep -o '"[^"]*"' | tr -d '"')
echo "BRC-20 deploy txid: $DEPLOY_TXID"
```

#### Step 4 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

This creates a BRC-20 token with:
- Ticker: "DEMO"
- Max supply: 21,000,000 tokens
- Mint limit: 1,000 tokens per mint

View the deploy inscription in the ord explorer:

```bash
echo "http://127.0.0.1/inscription/${DEPLOY_TXID}i0"
```

### Mint BRC-20 tokens

Minting claims tokens from the deployed supply and credits them to the canister's address. The amount must not exceed the per-mint limit (1,000 in this example).

#### Step 1 — Mint tokens

```bash
MINT_TXID=$(icp canister call backend mint_brc20 '("DEMO", 1000: nat64)' | grep -o '"[^"]*"' | tr -d '"')
echo "BRC-20 mint txid: $MINT_TXID"
```

#### Step 2 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 3 — View the mint inscription

```bash
echo "http://127.0.0.1/inscription/${MINT_TXID}i0"
```

### Transfer BRC-20 tokens

A BRC-20 transfer uses three transactions: create a transfer inscription at the sender's (canister's) address, then move that inscription UTXO to the recipient. This two-step handoff is how BRC-20 indexers track balance changes.

#### Step 1 — Transfer tokens

```bash
DEST="bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt"
TRANSFER_TXID=$(icp canister call backend transfer_brc20 '(record {
  tick = "DEMO";
  amount = 500: nat64;
  destination_address = "'"$DEST"'";
})' | grep -o '"[^"]*"' | tr -d '"')
echo "BRC-20 transfer txid: $TRANSFER_TXID"
```

#### Step 2 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 3 — Verify the transfer inscription

```bash
ord --config-dir . decode --txid "$TRANSFER_TXID"
# The inscription content is the transfer JSON:
# {"p":"brc-20","op":"transfer","tick":"DEMO","amt":"500"}
```

The recipient now holds the BRC-20 transfer inscription. BRC-20 indexers credit 500 DEMO to the recipient's balance and debit it from the canister's balance.

## Notes on implementation

This example implements several important patterns for Bitcoin integration:

- **Derivation paths**: Keys are derived using structured derivation paths according to BIP-32, ensuring reproducible key generation.
- **Key caching**: Optimization is used to avoid repeated calls to `get_ecdsa_public_key` and `get_schnorr_public_key`.
- **Manual transaction construction**: Transactions are assembled and signed manually, ensuring maximum flexibility in construction and fee estimation.
- **Cost optimization**: When testing on mainnet, the [chain-key testing canister](https://github.com/dfinity/chainkey-testing-canister) can be used to save on costs for calling the threshold signing APIs.
- **Asset protocols**: Bitcoin assets (Ordinals, Runes, BRC-20) demonstrate advanced scripting capabilities and witness data usage.

## Security considerations and best practices

This example is provided for educational purposes and is not production-ready. It is important to consider security implications when developing applications that interact with Bitcoin or other cryptocurrencies. The code has **not been audited** and may contain vulnerabilities or security issues.

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

- Certify query responses if they are relevant for security, since the app offers a method to read balances.
- Use a decentralized governance system like SNS to give a canister a decentralized controller, since decentralized control may be essential for canisters holding bitcoins on behalf of users.
