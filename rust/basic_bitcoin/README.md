# Basic Bitcoin

This example demonstrates how to deploy a canister smart contract on the Internet Computer that can receive and send Bitcoin, including support for legacy (P2PKH), SegWit (P2WPKH), and Taproot (P2TR) address types.

## Architecture

This example integrates with the Internet Computer's built-in:

* [ECDSA API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key)
* [Schnorr API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr)
* [Bitcoin API](https://github.com/dfinity/bitcoin-canister/blob/master/INTERFACE_SPECIFICATION.md)

For background on the ICP <-> BTC integration, refer to the [Bitcoin integration documentation](https://wiki.internetcomputer.org/wiki/Bitcoin_Integration).

## Prerequisites

* Install the [IC SDK](https://internetcomputer.org/docs/building-apps/getting-started/install)
* Install `cargo` (Rust toolchain)

## Step 1: Building and deploying the canister

Clone the repo and build the Rust canister:

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/basic_bitcoin
cargo build --release --target wasm32-unknown-unknown
```

### Deploy to the Internet Computer

Make sure you have cycles, then deploy the canister:

```bash
dfx deploy --ic basic_bitcoin --argument '(variant { testnet })'
```

#### What this does

- `dfx deploy` tells the command line interface to `deploy` the smart contract
- `--ic` tells the command line to deploy the smart contract to the mainnet ICP blockchain
- `--argument '(variant { testnet })'` passes the argument `testnet` to initialize the smart contract, telling it to connect to the Bitcoin testnet

#### After deploy

Your canister is live and ready to use! You can interact with it using either the command line or using the Candid UI, which is the link you see in the terminal.

The `basic_bitcoin` example is deployed on mainnet for illustration purposes and is interacting with Bitcoin **testnet4**. It has the URL https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=vvha6-7qaaa-aaaap-ahodq-cai and serves up the Candid web UI for this particular canister deployed on mainnet.

## 2. Supported Bitcoin address types

This example demonstrates how to generate and use the following address types:

1. **P2PKH (Legacy)** — using ECDSA and `sign_with_ecdsa`
2. **P2WPKH (SegWit v0)** — using ECDSA and `sign_with_ecdsa`
3. **P2TR (Taproot, key-path-only)** — using Schnorr keys and `sign_with_schnorr`
4. **P2TR (Taproot, script-path-enabled)** — commits to a script allowing both key path and script path spending

Use the Candid UI or command line to generate these addresses with:

```bash
dfx canister call basic_bitcoin get_p2pkh_address
# or get_p2wpkh_address, get_p2tr_key_path_only_address, get_p2tr_script_path_enabled_address
```

## 3. Receiving Bitcoin

Send testnet Bitcoin to one of the generated addresses using a [testnet faucet](https://coinfaucet.eu/en/btc-testnet/). Make sure to select **testnet4**.

Once the transaction confirms (usually within \~10 minutes), the canister will be able to spend the received BTC.

## 4. Checking balance

Check the balance of any Bitcoin address:

```bash
dfx canister call basic_bitcoin get_balance '("<bitcoin_address>")'
```

This uses `bitcoin_get_balance` and works for any supported address type. Requires at least one confirmation to be reflected.

## 5. Sending Bitcoin

You can send BTC using the following endpoints:

* `send_from_p2pkh_address`
* `send_from_p2wpkh_address`
* `send_from_p2tr_key_path_only_address`
* `send_from_p2tr_script_path_enabled_address_key_spend`
* `send_from_p2tr_script_path_enabled_address_script_spend`

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

The function returns the transaction ID, which you can track on [mempool.space testnet4](https://mempool.space/testnet4/).

## 6. Retrieving block headers

You can query historical block headers:

```bash
dfx canister call basic_bitcoin get_block_headers '(10: nat32)'
# or a range:
dfx canister call basic_bitcoin get_block_headers '(0: nat32, 11: nat32)'
```

This calls `bitcoin_get_block_headers`, useful for validating blockchains or light client logic.

## Notes on Implementation

* Keys are derived using structured derivation paths, according to BIP-32.
* Key caching is used to avoid repeated calls to `get_ecdsa_public_key` and `get_schnorr_public_key`.
* Transactions are assembled and signed manually, ensuring maximum flexibility in construction and fee estimation.

## Conclusion

In this tutorial, you were able to:

- Deploy a canister smart contract on the ICP blockchain that can receive & send bitcoin.
- Acquire cycles to deploy the canister to the ICP mainnet.
- Connect the canister to the Bitcoin testnet.
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

*Last updated: May 2025*
