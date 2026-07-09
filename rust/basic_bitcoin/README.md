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

> **Fresh network:** When you restart with `icp network start`, the ord server may show stale data from a previous session. Ord stores its index in `ord-db/regtest/` (inside the `basic_bitcoin` directory) and does not clear it automatically when the Bitcoin node resets. Ord detects the chain change and reindexes as soon as the first block is mined in the new instance — the mining step in the walkthrough below takes care of this. To start completely clean, delete the `ord-db/` directory before starting the ord server.

> **Important — Bitcoin Configuration:** To work with Bitcoin assets, make sure bitcoind is configured to accept non-standard transactions by including this setting in your `bitcoin.conf`:
>
> ```
> acceptnonstdtxn=1
> ```

## Ordinals

[Ordinals](https://ordinals.com) is a protocol that assigns a unique serial number to every satoshi, then allows arbitrary data (text, images, etc.) to be inscribed onto individual satoshis. Inscriptions are stored permanently in the Bitcoin blockchain using a two-transaction commit/reveal process. Two operations make up the full lifecycle: **inscribe** → **transfer**.

### Inscribe an Ordinal

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

#### Step 3 — Create the inscription

```bash
TXID=$(icp canister call backend inscribe_ordinal '("Hello Bitcoin")' | grep -o '"[^"]*"' | tr -d '"')
echo "Reveal txid: $TXID"
```

#### Step 4 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 5 — View in the ord explorer

```bash
echo "http://127.0.0.1/inscription/${TXID}i0"
```

- All inscriptions: `http://127.0.0.1/inscriptions`

> **Note:** The homepage at `http://127.0.0.1/` shows a "Latest Inscriptions" section that may appear empty — use `/inscriptions` instead.

### Transfer an Ordinal

An inscription is permanently bound to the first satoshi of the reveal output. To move the inscription to a new owner, that satoshi must flow to the first output (vout 0) of the spending transaction. `transfer_ordinal` achieves this by making the inscription UTXO the sole input — the inscription satoshi then flows directly to the recipient.

> **Prerequisites:** Complete the inscribe steps above. `$TXID`, `$ADDR`, and `$CONTAINER` must be set.

#### Step 1 — Transfer the inscription

```bash
DEST="bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt"
ORD_TRANSFER_TXID=$(icp canister call backend transfer_ordinal "(\"$TXID\", \"$DEST\")" | grep -o '"[^"]*"' | tr -d '"')
echo "Ordinal transfer txid: $ORD_TRANSFER_TXID"
```

#### Step 2 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 3 — Verify the new owner

Query the inscription's owner via the ord JSON API. Before the transfer it shows the canister's Taproot address; after mining the block it should show the destination:

```bash
curl -s -H "Accept: application/json" "http://127.0.0.1/inscription/${TXID}i0" | python3 -m json.tool | grep address
# Expected: "address": "bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt"
```

You can also view the inscription in the ord explorer and confirm ownership visually:

```bash
echo "http://127.0.0.1/inscription/${TXID}i0"
```

> **How satoshi tracking works:** The Ordinals protocol assigns ordinal numbers to satoshis in the order they are mined. The inscription is associated with the first satoshi of the reveal output. When that UTXO is spent as the first (and only) input, Bitcoin's satoshi-ordering rules guarantee the first satoshi lands in vout 0 — so the recipient at vout 0 receives the inscription. This is why `transfer_ordinal` uses the reveal UTXO as the sole input and puts the recipient at output index 0.

## Runes

[Runes](https://docs.ordinals.com/runes.html) is a fungible token protocol that embeds token metadata directly into Bitcoin transactions using `OP_RETURN` outputs. Two operations make up the full lifecycle: **etch** → **transfer**.

### Etch a Rune

Etching a rune requires two transactions separated by at least 6 block confirmations:

1. **Commit** — creates a P2TR output whose tapscript contains the rune name commitment bytes
2. **Etch** — spends that output (via script-path, placing the commitment in the witness) together with an `OP_RETURN` Runestone

The ord indexer validates that the etching transaction's input spends a P2TR output containing the rune commitment bytes, and that the committed output has at least 6 block confirmations. Without this, the etching is silently ignored and no Rune ID is assigned.

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

#### Step 3 — Commit to the rune name

The name must be uppercase and between 1–28 characters. The Runes protocol uses a block-height-based unlock schedule — **names with 13 or more characters are immediately active** in regtest at any block height.

> **Why 13+ characters?** The ord indexer uses linear interpolation to derive a minimum rune value per block. Every 13-char name has a numeric value above `STEPS[12] = 99,246,114,928,149,462` — the highest minimum the formula ever produces — so 13-char names are immediately active at any block height. Shorter names have unlock heights that must be mined past (12-char names unlock across blocks 0–17,499; 7-char names across 87,500–104,999).

> **Turbo mode**: The `etch_rune` implementation sets `turbo: true`, which opts the rune into future ord protocol upgrades. It does **not** bypass the name unlock schedule.

```bash
COMMIT_TXID=$(icp canister call backend commit_rune '("BASICBITCOINS")' | grep -o '"[^"]*"' | tr -d '"')
echo "Commit txid: $COMMIT_TXID"
```

#### Step 4 — Mine 6 blocks to mature the commitment

The ord protocol requires the commit output to have at least 6 block confirmations before the etching is accepted.

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 6 "$ADDR"
```

#### Step 5 — Etch the Rune

```bash
ETCH_TXID=$(icp canister call backend etch_rune '("BASICBITCOINS")' | grep -o '"[^"]*"' | tr -d '"')
echo "Etch txid: $ETCH_TXID"
```

#### Step 6 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 7 — Decode the Runestone to verify

```bash
ord --config-dir . decode --txid "$ETCH_TXID"
# Expected output:
# {
#   "inscriptions": [],
#   "runestone": {
#     "Runestone": {
#       "edicts": [],
#       "etching": {
#         "divisibility": null,
#         "premine": 1000000,
#         "rune": "BASICBITCOINS",
#         "spacers": null,
#         "symbol": "🪙",
#         "terms": null,
#         "turbo": true
#       },
#       "mint": null,
#       "pointer": null
#     }
#   }
# }
```

#### Step 8 — View in the ord explorer

Open the rune in the ord explorer: `http://127.0.0.1/rune/BASICBITCOINS`

- All runes: `http://127.0.0.1/runes`

The Rune is now etched with 1,000,000 tokens minted to your address.

### Transfer a Rune

#### Step 1 — Look up the rune ID and verify your balance

The rune ID (block height : transaction index) is required to identify which rune to transfer. Fetch it from the ord JSON API:

```bash
RUNE_ID=$(curl -s -H "Accept: application/json" http://127.0.0.1/rune/BASICBITCOINS | \
  tr -d '[:space:]' | grep -o '"id":"[0-9]*:[0-9]*"' | cut -d'"' -f4)
RUNE_BLOCK=$(echo "$RUNE_ID" | cut -d: -f1)
RUNE_TX=$(echo "$RUNE_ID" | cut -d: -f2)
echo "Rune ID: $RUNE_BLOCK:$RUNE_TX"
```

The 1,000,000 premined tokens are held at the canister's dedicated rune address (derivation index 1, separate from the main funding address). Check the balance:

```bash
RUNE_ADDR=$(icp canister call backend get_rune_address '()' | grep -o '"[^"]*"' | tr -d '"')
curl -s -H "Accept: application/json" "http://127.0.0.1/address/$RUNE_ADDR" | python3 -m json.tool | grep -A6 '"runes"'
# Expected: "BASICBITCOINS": { "amount": 1000000, ... }
```

> **Note:** If the balance shows empty, ord may still be indexing the etching block. Wait a moment and retry.

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

The output should contain a Runestone with an edict assigning 100,000 tokens to output 0 (the recipient) and a `pointer: 2` directing the unallocated remainder to output 2 (the change). Check the resulting output balances:

```bash
# Recipient — should show 100,000 BASICBITCOINS
curl -s -H "Accept: application/json" "http://127.0.0.1/output/${TRANSFER_TXID}:0" | python3 -m json.tool

# Change (rune address) — should show 900,000 BASICBITCOINS
curl -s -H "Accept: application/json" "http://127.0.0.1/output/${TRANSFER_TXID}:2" | python3 -m json.tool
```

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

A BRC-20 transfer chains three Bitcoin transactions internally:
1. **Commit** — funds a Taproot address committed to the transfer JSON script
2. **Reveal** — spends the commit output, creating the transfer inscription at the canister's own address (locking `amount` tokens from the sender's balance)
3. **Send** — moves the inscription UTXO from the canister's address to the recipient

BRC-20 indexers see the inscription travel from sender → recipient and credit the recipient's balance accordingly. The returned txid is the send transaction (step 3).

#### Step 1 — Transfer tokens

```bash
DEST="bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt"
TRANSFER_TXID=$(icp canister call backend transfer_brc20 '(record {
  tick = "DEMO";
  amount = 500: nat64;
  destination_address = "'"$DEST"'";
})' | grep -o '"[^"]*"' | tr -d '"')
echo "BRC-20 send txid: $TRANSFER_TXID"
```

#### Step 2 — Mine a confirmation block

```bash
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

#### Step 3 — Verify the inscription arrived at the recipient

```bash
curl -s -H "Accept: application/json" "http://127.0.0.1/output/${TRANSFER_TXID}:0" | python3 -m json.tool | grep -E '"address"|"inscriptions"'
# Expected:
#   "address": "bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt"
#   "inscriptions": ["<reveal_txid>i0"]
```

You can view the inscription content (the transfer JSON) in the ord explorer:

```bash
# Get the inscription ID from the output and open it in the ord explorer
INSCRIPTION_ID=$(curl -s -H "Accept: application/json" "http://127.0.0.1/output/${TRANSFER_TXID}:0" | python3 -c "import sys,json; print(json.load(sys.stdin)['inscriptions'][0])")
echo "http://127.0.0.1/inscription/$INSCRIPTION_ID"
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
