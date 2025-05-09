# Basic Bitcoin

This example demonstrates how to deploy a canister smart contract on the Internet Computer that can receive and send Bitcoin, including support for legacy (P2PKH), SegWit (P2WPKH), and Taproot (P2TR) address types.

## Architecture

This example integrates with the Internet Computer's built-in:

- [ECDSA API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key)
- [Schnorr API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr)
- [Bitcoin API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md)

For background on the ICP <-> BTC integration, refer to the [Bitcoin integration documentation](https://wiki.internetcomputer.org/wiki/Bitcoin_Integration).

## Prerequisites

- [x] [Rust toolchain](https://www.rust-lang.org/tools/install)
- [x] [Internet Computer SDK](https://internetcomputer.org/docs/building-apps/getting-started/install)
- [x] [Local installation of Bitcoin](https://internetcomputer.org/docs/bitcoin) 
- [x] On macOS, an `llvm` version that supports the `wasm32-unknown-unknown` target is required. This is because the `bitcoin` library relies on `secp256k1-sys`, which requires `llvm` to build. The default `llvm` version provided by XCode does not meet this requirement. Instead, install the [Homebrew version](https://formulae.brew.sh/formula/llvm), using `brew install llvm`. 

## Step 1: Building and deploying the canister

### Clone the examples repo

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_bitcoin
```

### Start the ICP local development environment

```bash
dfx start --clean --background
```

### Start the local Bitcoin testnet (regtest)

In a separate terminal window, run the following: 

```bash
bitcoind -conf=$(pwd)/bitcoin.conf -datadir=$(pwd)/bitcoin_data --port=18444
```

### Deploy the canister

```bash
dfx deploy basic_bitcoin --argument '(variant { regtest })'
```

What this does:

- `dfx deploy` tells the command line interface to `deploy` the smart contract.
- `--argument '(variant { regtest })'` passes the argument `regtest` to initialize the smart contract, telling it to connect to the local Bitcoin regtest network.


Your canister is live and ready to use! You can interact with it using either the command line or using the Candid UI, which is the link you see in the terminal.

> [!NOTE]
> You can also interact with a pre-deployed version of the `basic_bitcoin` example running on the IC mainnet and configured to interact with Bitcoin **testnet4**.
> 
> Access the Candid UI of the example: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=vvha6-7qaaa-aaaap-ahodq-cai

## 2. Supported Bitcoin address types

This example demonstrates how to generate and use the following address types:

1. **P2PKH (Legacy)** using ECDSA and `sign_with_ecdsa`
2. **P2WPKH (SegWit v0)** using ECDSA and `sign_with_ecdsa`
3. **P2TR (Taproot, key-path-only)** using Schnorr keys and `sign_with_schnorr`
4. **P2TR (Taproot, script-path-enabled)** commits to a script allowing both key path and script path spending

Use the Candid UI or command line to generate these addresses with:

```bash
dfx canister call basic_bitcoin get_p2pkh_address
# or get_p2wpkh_address, get_p2tr_key_path_only_address, get_p2tr_script_path_enabled_address
```

## 3. Receiving bitcoin

Use the `bitcoin-cli` to mine a Bitcoin block and send the block reward in the form of local testnet BTC to one of the canister addresses.

```bash
bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 1 <bitcoin_address>
```

## 4. Checking balance

Check the balance of any Bitcoin address:

```bash
dfx canister call basic_bitcoin get_balance '("<bitcoin_address>")'
```

This uses `bitcoin_get_balance` and works for any supported address type. Requires at least one confirmation to be reflected.

## 5. Sending bitcoin

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
> Newly created bitcoin, like those you created with the above `bitcoin-cli` command cannot be spent until 10 additional blocks have been added to the chain. To make your bitcoin spendable, create 10 additional blocks. Choose one of the canister addresses as receiver of the block reward or use any valid bitcoin dummy addres.
> 
> ```bash
> bitcoin-cli -conf=$(pwd)/bitcoin.conf generatetoaddress 10 <bitcoin_address>
> ```

The function returns the transaction ID, which you can track on [mempool.space testnet4](https://mempool.space/testnet4/).

## 6. Retrieving block headers

You can query historical block headers:

```bash
dfx canister call basic_bitcoin get_block_headers '(10: nat32)'
# or a range:
dfx canister call basic_bitcoin get_block_headers '(0: nat32, 11: nat32)'
```

This calls `bitcoin_get_block_headers`, useful for validating blockchains or light client logic.

## Notes on implementation

- Keys are derived using structured derivation paths according to BIP-32.
- Key caching is used to avoid repeated calls to `get_ecdsa_public_key` and `get_schnorr_public_key`.
- Transactions are assembled and signed manually, ensuring maximum flexibility in construction and fee estimation.

## Conclusion

In this tutorial, you were able to:

- Deploy a canister smart contract on the ICP blockchain that can receive & send bitcoin.
- Acquire cycles to deploy the canister to the ICP mainnet.
- Connect the canister to the local Bitcoin testnet.
- Send the canister some testnet BTC.
- Check the testnet BTC balance of the canister.
- Use the canister to send testnet BTC to another testnet BTC address.

The steps to develop Bitcoin dapps locally are extensively documented in [this tutorial](https://internetcomputer.org/docs/current/developer-docs/integrations/bitcoin/local-development).

Note that for _testing_ on mainnet, the [chain-key testing canister](https://github.com/dfinity/chainkey-testing-canister) can be used to save on costs for calling the threshold signing APIs for signing the BTC transactions.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

- [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since the app e.g. offers a method to read balances.
- [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview), since decentralized control may be essential for canisters holding bitcoins on behalf of users.

---

_Last updated: May 2025_
