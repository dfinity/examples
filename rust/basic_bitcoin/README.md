# Basic Bitcoin

This example demonstrates how a canister can receive and send bitcoin on the Internet Computer, including support for legacy (P2PKH), SegWit (P2WPKH), and Taproot (P2TR) address types.

This example also includes how to work with Bitcoin assets such as Ordinals, Runes, and BRC-20 tokens.

See also the [Motoko version](../../motoko/basic_bitcoin).

## Architecture

This example integrates with the Internet Computer's built-in:

* [ECDSA API](https://docs.internetcomputer.org/references/management-canister/#ic-ecdsa_public_key)
* [Schnorr API](https://docs.internetcomputer.org/references/management-canister/#ic-sign_with_schnorr)
* [Bitcoin API](https://docs.internetcomputer.org/references/protocol-canisters/#bitcoin-canisters)

For background on the ICP<>BTC integration, refer to the [Learn Hub](https://learn.internetcomputer.org/hc/en-us/articles/34211154520084-Bitcoin-Integration).

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) v1.85+ with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Docker](https://docs.docker.com/get-docker/) (required to run the custom network launcher image that bundles bitcoind)
- On macOS, an `llvm` version that supports the `wasm32-unknown-unknown` target is required. The Rust `bitcoin` library relies on the `secp256k1-sys` crate, which requires `llvm` to build. The default `llvm` version provided by XCode does not meet this requirement. Install the [Homebrew version](https://formulae.brew.sh/formula/llvm) using `brew install llvm`.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_bitcoin
```

### Build the network launcher image

The local network bundles bitcoind inside a custom Docker image. Build it once before starting the network:

```bash
./build-image.sh
# or: make build-image
```

### Deploy locally and test

```bash
icp network start -d
icp deploy --cycles 30t
bash test.sh
icp network stop
```

> If tests fail with an out-of-cycles error, run `make topup` to add 30 trillion cycles to the backend canister and retry.

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

## Receiving bitcoin

Use `bitcoin-cli` inside the running network container to mine blocks and send the block reward to a canister address:

```bash
# Get the container ID of the running network launcher
CONTAINER=$(docker ps --filter "ancestor=icp-cli-network-launcher-bitcoin" --format "{{.ID}}" | head -1)

# Get an address from the canister
ADDR=$(icp canister call backend get_p2pkh_address '()' | grep -o '"[^"]*"' | tr -d '"')

# Mine 1 block to that address
docker exec $CONTAINER bitcoin-cli -regtest \
  -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
  generatetoaddress 1 "$ADDR"
```

## Checking balance

Check the balance of any Bitcoin address:

```bash
icp canister call backend get_balance '("<bitcoin_address>")'
```

This uses `bitcoin_get_balance` and works for any supported address type. The balance requires at least one confirmation to be reflected.

## Sending bitcoin

You can send bitcoin using the following endpoints:

- `send_from_p2pkh_address`
- `send_from_p2wpkh_address`
- `send_from_p2tr_key_path_only_address`
- `send_from_p2tr_script_path_enabled_address_key_spend`
- `send_from_p2tr_script_path_enabled_address_script_spend`

Each endpoint internally:

1. Estimates fees
2. Looks up spendable UTXOs
3. Builds a transaction to the target address
4. Signs using ECDSA or Schnorr, depending on address type
5. Broadcasts the transaction using `bitcoin_send_transaction`

Example:

```bash
icp canister call backend send_from_p2pkh_address '(record {
  destination_address = "bcrt1qg8qknn6f3txqg97gt8ca0ctya0vw7ep6d02qmt";
  amount_in_satoshi = 4321;
})'
```

> **Important:** Newly mined bitcoin cannot be spent until 100 additional blocks have been added to the chain. To make your bitcoin spendable, create 100 additional blocks with any valid address as the recipient.

The function returns the transaction ID. When the canister is deployed on IC mainnet, you can track testnet transactions on [mempool.space](https://mempool.space/testnet4/).

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

> **Note:** This repository includes a [default ord config file](./ord.yaml) that matches the also provided [bitcoin config file](./bitcoin.conf).

> **Important — Bitcoin Configuration:** To work with Bitcoin assets, make sure bitcoind is configured to accept non-standard transactions by including this setting in your `bitcoin.conf`:
>
> ```
> acceptnonstdtxn=1
> ```

## Inscribe an Ordinal

[Ordinals](https://ordinals.com) is a protocol that allows inscribing arbitrary data (text, images, etc.) onto individual satoshis, creating unique digital artifacts on Bitcoin. Each inscription is permanently stored in the Bitcoin blockchain using a two-transaction commit/reveal process.

### Step-by-step process:

1. **Start the ord server** to index transactions:
   ```bash
   ord --config-dir . server
   ```

2. **Get a Taproot address** for funding the inscription:
   ```bash
   icp canister call backend get_p2tr_key_path_only_address '()'
   ```

3. **Fund the address** with sufficient bitcoin (101 blocks ensures spendability):
   ```bash
   docker exec $CONTAINER bitcoin-cli -regtest \
     -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
     generatetoaddress 101 <p2tr_key_path_only_address>
   ```

4. **Create the inscription** with your desired text:
   ```bash
   icp canister call backend inscribe_ordinal '("Hello Bitcoin")'
   ```

5. **Mine a block** to confirm the transactions:
   ```bash
   docker exec $CONTAINER bitcoin-cli -regtest \
     -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
     generatetoaddress 1 <p2tr_key_path_only_address>
   ```

The function returns the reveal transaction ID. Your inscription is now permanently stored on Bitcoin and can be viewed using ord or other Ordinals explorers. The default address of the local `ord` server is `http://127.0.0.1:80/`.

## Etch a Rune

[Runes](https://docs.ordinals.com/runes.html) is a fungible token protocol that embeds token metadata directly into Bitcoin transactions using `OP_RETURN` outputs. Unlike Ordinals, Runes are created in a single transaction and support standard fungible token operations.

### Step-by-step process:

1. **Start the ord server** to track Rune balances:
   ```bash
   ord --config-dir . server
   ```

2. **Get a Taproot address** for the Rune etching:
   ```bash
   icp canister call backend get_p2tr_key_path_only_address '()'
   ```

3. **Fund the address** with bitcoin to pay for the etching:
   ```bash
   docker exec $CONTAINER bitcoin-cli -regtest \
     -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
     generatetoaddress 101 <p2tr_key_path_only_address>
   ```

4. **Etch the Rune** with an uppercase name (maximum 28 characters):
   ```bash
   icp canister call backend etch_rune '("ICPRUNE")'
   ```

5. **Mine a block** to confirm the etching:
   ```bash
   docker exec $CONTAINER bitcoin-cli -regtest \
     -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
     generatetoaddress 1 <p2tr_key_path_only_address>
   ```

6. **Decode the Runestone** to verify the etching:
   ```bash
   ord --config-dir . decode --txid <transaction_id>
   ```

The Rune is now etched with 1,000,000 tokens minted to your address. The tokens can be transferred using standard Bitcoin transactions with Runestone data.

## Deploy a BRC-20 token

[BRC-20](https://domo-2.gitbook.io/brc-20-experiment/) is a token standard built on top of Ordinals that uses structured JSON payloads to create fungible tokens. BRC-20 tokens follow the same inscription process as Ordinals but with standardized JSON formats.

### Step-by-step process:

1. **Start the ord server** to index BRC-20 inscriptions:
   ```bash
   ord --config-dir . server
   ```

2. **Get a Taproot address** and fund it with bitcoin.

3. **Deploy the BRC-20 token** with a 4-character ticker:
   ```bash
   icp canister call backend inscribe_brc20 '("DEMO")'
   ```

4. **Mine a block** to confirm the deployment:
   ```bash
   docker exec $CONTAINER bitcoin-cli -regtest \
     -rpcuser=ic-btc-integration -rpcpassword=ic-btc-integration \
     generatetoaddress 1 <p2tr_key_path_only_address>
   ```

This creates a BRC-20 token with:
- Ticker: "DEMO"
- Max supply: 21,000,000 tokens
- Mint limit: 1,000 tokens per mint

The deployment inscription contains JSON metadata that BRC-20 indexers use to track token balances and transfers. Additional mint and transfer operations require separate inscriptions following the BRC-20 protocol.

To view the deployed BRC-20 token, use the local `ord` explorer at `http://127.0.0.1:80/`.

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
