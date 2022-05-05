# Bitcoin Wallet Example

## Summary

This example dapp shows how to build a Bitcoin wallet making use of the [Bitcoin intergration](https://smartcontracts.org/docs/developers-guide/concepts/bitcoin-integration.html).

This dapp is under active development and not ready to be used yet.
<!--Once some parts of its functionality are implemented, this README will be updated accordingly with instructions on how to build and run the dapp.-->

## Usage

The following commands will start a replica, install the Bitcoin canister and the development Internet Identity canister, and deploy the Bitcoin wallet webapp:

```bash
$ dfx start --background --clean
$ dfx deploy btc
$ cargo run --features="tokio candid ic-agent garcon tonic tonic-build" --bin adapter-shim $(dfx canister id btc)

$ npm install
$ dfx deploy internet_identity --argument '(null)'
$ dfx deploy bitcoin_wallet_assets --argument "(record { bitcoin_canister_id = principal \"$(dfx canister id btc)\" })"
```
