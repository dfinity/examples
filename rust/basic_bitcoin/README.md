# Basic Bitcoin

This example demonstrates how to deploy a smart contract on the Internet Computer that can receive and send Bitcoin, including support for legacy (P2PKH), SegWit (P2WPKH), and Taproot (P2TR) address types.

The repository also includes examples of how to work with Bitcoin assets such as Ordinals, Runes, and BRC-20 tokens.

## Table of Contents

- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Building and deploying the smart contract](#building-and-deploying-the-smart-contract)
- [Generating Bitcoin addresses](#generating-bitcoin-addresses)
- [Receiving Bitcoin](#receiving-bitcoin)
- [Checking balance](#checking-balance)
- [Sending Bitcoin](#sending-bitcoin)
- [Retrieving block headers](#retrieving-block-headers)
- [Bitcoin Assets](#bitcoin-assets)
- [Inscribe an Ordinal](#inscribe-an-ordinal)
- [Etch a Rune](#etch-a-rune)
- [Deploy a BRC-20 Token](#deploy-a-brc-20-token)
- [Notes on implementation](#notes-on-implementation)
- [Security considerations and best practices](#security-considerations-and-best-practices)

## Architecture

This example integrates with the Internet Computer's built-in:

- [ECDSA API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key)
- [Schnorr API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr)
- [Bitcoin API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md)

For background on the ICP<>BTC integration, refer to the [Learn Hub](https://learn.internetcomputer.org/hc/en-us/articles/34211154520084-Bitcoin-Integration).

## Prerequisites

- [x] [Rust toolchain](https://www.rust-lang.org/tools/install)
- [x] [Internet Computer SDK](https://internetcomputer.org/docs/building-apps/getting-started/install)
- [x] [Local Bitcoin testnet (regtest)](https://internetcomputer.org/docs/build-on-btc/btc-dev-env#create-a-local-bitcoin-testnet-regtest-with-bitcoind)
- [x] On macOS, an `llvm` version that supports the `wasm32-unknown-unknown` target is required. This is because the Rust `bitcoin` library relies on the `secp256k1-sys` crate, which requires `llvm` to build. The default `llvm` version provided by XCode does not meet this requirement. Instead, install the [Homebrew version](https://formulae.brew.sh/formula/llvm), using `brew install llvm`.

## Building and deploying the smart contract

### 1. Clone the examples repo

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_bitcoin
```

### 2. Start the ICP execution environment

In terminal 1, run the following:
```bash
dfx start --enable-bitcoin --bitcoin-node 127.0.0.1:18444
```
This starts a local canister execution environment with Bitcoin support enabled.

### 3. Start the Bitcoin testnet (regtest)

In terminal 2, run the following to start the local Bitcoin testnet:

```bash
bitcoind -conf=$(pwd)/bitcoin.conf -datadir=$(pwd)/bitcoin_data --port=18444
```

### 4. Deploy the smart contract

Finally, in terminal 3, run the following to deploy the smart contract:

```bash
dfx deploy basic_bitcoin --argument '(variant { regtest })'
```

What this does:

- `dfx deploy` tells the command line interface to `deploy` the smart contract.
- `--argument '(variant { regtest })'` passes the argument `regtest` to initialize the smart contract, telling it to connect to the local Bitcoin regtest network.

Your smart contract is live and ready to use! You can interact with it using either the command line or the Candid UI, which is the link you see in the terminal.

> [!NOTE]
> You can also interact with a pre-deployed version of the `basic_bitcoin` example running on the IC mainnet and configured to interact with Bitcoin **testnet4**.
>
> Access the Candid UI of the example: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=vvha6-7qaaa-aaaap-ahodq-cai

## Generating Bitcoin addresses

The example demonstrates how to generate and use the following address types:

1. **P2PKH (Legacy)** using ECDSA and `sign_with_ecdsa`
2. **P2WPKH (SegWit v0)** using ECDSA and `sign_with_ecdsa`
3. **P2TR (Taproot, key-path-only)** using Schnorr keys and `sign_with_schnorr`
4. **P2TR (Taproot, script-path-enabled)** commits to a script allowing both key path and script path spending

Use the Candid UI or command line to generate these addresses with:

```bash
dfx canister call basic_bitcoin get_p2pkh_address
# or get_p2wpkh_address, get_p2tr_key_path_only_address, get_p2tr_script_path_enabled_address
```

## Receiving Bitcoin

Use the `bitcoin-cli` to mine a Bitcoin block and send the block reward in the form of local testnet BTC to one of the smart contract addresses.

```bash
bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 1 <bitcoin_address>
```

## Checking balance

Check the balance of any Bitcoin address:

```bash
dfx canister call basic_bitcoin get_balance '("<bitcoin_address>")'
```

This uses `bitcoin_get_balance` and works for any supported address type. The balance requires at least one confirmation to be reflected.

## Sending Bitcoin

You can send BTC using the following endpoints:

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
dfx canister call basic_bitcoin send_from_p2pkh_address '(record {
  destination_address = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt";
  amount_in_satoshi = 4321;
})'
```

> [!IMPORTANT]
> Newly mined bitcoin, like those you created with the above `bitcoin-cli` command, cannot be spent until 100 additional blocks have been added to the chain. To make your bitcoin spendable, create 100 additional blocks. Choose one of the smart contract addresses as receiver of the block reward or use any valid Bitcoin dummy address.
>
> ```bash
> bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 100 <bitcoin_address>
> ```

The function returns the transaction ID. When interacting with the contract deployed on IC mainnet, you can track testnet transactions on [mempool.space](https://mempool.space/testnet4/).

## Retrieving block headers

You can query historical block headers:

```bash
dfx canister call basic_bitcoin get_block_headers '(10: nat32)'
# or a range:
dfx canister call basic_bitcoin get_block_headers '(0: nat32, 11: nat32)'
```

This calls `bitcoin_get_block_headers`, which is useful for blockchain validation or light client logic.

## Bitcoin Assets

Bitcoin's scripting capabilities enable various digital assets beyond simple transfers. This example demonstrates how to create and interact with three major Bitcoin asset protocols from an ICP smart contract:

- **Ordinals**: Inscribe arbitrary data onto individual satoshis
- **Runes**: Create fungible tokens using OP_RETURN outputs
- **BRC-20**: Build fungible tokens on top of Ordinals using JSON

### Prerequisites for Bitcoin Assets

All Bitcoin assets rely on off-chain indexing since the Bitcoin protocol doesn't natively support querying these assets. The `ord` CLI tool is the standard indexer for Bitcoin assets like Ordinals and Runes.

Install `ord` using a package manager. For example, on macOS:

```bash
brew install ord
```

For other platforms, see the [ord repository](https://github.com/ordinals/ord) for installation instructions.

> [!NOTE]
> This repository includes a [default ord config file](./ord.yaml) that matches the also provided [bitcoin config file](./bitcoin.conf).

> [!IMPORTANT]
> **Bitcoin Configuration**: To work with Bitcoin assets, make sure bitcoind is configured to accept non-standard transactions by including this setting in your `bitcoin.conf`:
>
> ```
> acceptnonstdtxn=1
> ```

## Inscribe an Ordinal

[Ordinals](https://ordinals.com) is a protocol that allows inscribing arbitrary data (text, images, etc.) onto individual satoshis, creating unique digital artifacts on Bitcoin. Each inscription is permanently stored in the Bitcoin blockchain using a two-transaction commit/reveal process.

### Step-by-step Process:

1. **Start the ord server** to index transactions:
   ```bash
   ord --config-dir . server
   ```

2. **Get a Taproot address** for funding the inscription:
   ```bash
   dfx canister call basic_bitcoin get_p2tr_key_path_only_address '()'
   ```

3. **Fund the address** with sufficient bitcoin (100 blocks ensures spendability):
   ```bash
   bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 100 <p2tr_key_path_only_address>
   ```

4. **Create the inscription** with your desired text:
   ```bash
   dfx canister call basic_bitcoin inscribe_ordinal '("Hello Bitcoin")'
   ```

5. **Mine a block** to confirm the transactions:
   ```bash
   bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 1 <p2tr_key_path_only_address>
   ```

The function returns the reveal transaction ID. Your inscription is now permanently stored on Bitcoin and can be viewed using ord or other Ordinals explorers. The default address of the local `ord` server is `http://127.0.0.1:80/`.

## Etch a Rune

[Runes](https://docs.ordinals.com/runes.html) is a fungible token protocol that embeds token metadata directly into Bitcoin transactions using OP_RETURN outputs. Unlike Ordinals, Runes are created in a single transaction and support standard fungible token operations.

### Step-by-step Process:

1. **Start the ord server** to track rune balances:
   ```bash
   ord --config-dir . server
   ```

2. **Get a Taproot address** for the rune etching:
   ```bash
   dfx canister call basic_bitcoin get_p2tr_key_path_only_address '()'
   ```

3. **Fund the address** with bitcoin to pay for the etching:
   ```bash
   bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 100 <p2tr_key_path_only_address>
   ```

4. **Etch the rune** with an uppercase name (maximum 28 characters):
   ```bash
   dfx canister call basic_bitcoin etch_rune '("ICPRUNE")'
   ```

5. **Mine a block** to confirm the etching:
   ```bash
   bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 1 <p2tr_key_path_only_address>
   ```

6. **Decode the runestone** to verify the etching:
   ```bash
   ord --config-dir . decode --txid <transaction_id>
   ```

The rune is now etched with 1,000,000 tokens minted to your address. The tokens can be transferred using standard Bitcoin transactions with runestone data.

## Deploy a BRC-20 Token

[BRC-20](https://domo-2.gitbook.io/brc-20-experiment/) is a token standard built on top of Ordinals that uses structured JSON payloads to create fungible tokens. BRC-20 tokens follow the same inscription process as Ordinals but with standardized JSON formats.

### Step-by-step Process:

1. **Start the ord server** to index BRC-20 inscriptions:
   ```bash
   ord --config-dir . server
   ```

2. **Get a Taproot address** for the token deployment:
   ```bash
   dfx canister call basic_bitcoin get_p2tr_key_path_only_address '()'
   ```

3. **Fund the address** with bitcoin:
   ```bash
   bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 100 <p2tr_key_path_only_address>
   ```

4. **Deploy the BRC-20 token** with a 4-character ticker:
   ```bash
   dfx canister call basic_bitcoin inscribe_brc20 '("DEMO")'
   ```

5. **Mine a block** to confirm the deployment:
   ```bash
   bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 1 <p2tr_key_path_only_address>
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

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

- [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/building-apps/security/data-integrity-and-authenticity#using-certified-variables-for-secure-queries), since the app e.g. offers a method to read balances.
- [Use a decentralized governance system like SNS to make a smart contract have a decentralized controller](https://internetcomputer.org/docs/building-apps/security/decentralization), since decentralized control may be essential for smart contracts holding bitcoins on behalf of users.

---

_Last updated: June 2025_
