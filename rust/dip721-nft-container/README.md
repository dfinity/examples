# DIP721 NFT Container

## Summary

This is an example implementation of an NFT (non-fungible token) smart contract, using the [DIP721] standard.

## Setup

To build and install this code, you will need:

- Git
- [DFX] version 0.9.0
- [Rust] version 1.55.0 or later

## Running Locally

Run the following commands to download and set up the project:

```sh
git clone https://github.com/dfinity/examples
cd examples/rust/dip721-nft-container
```

To start the local replica before installing the canister:

```sh
dfx start --background --clean
```

The canister expects a record parameter with the following fields:

- `custodians`: A list of users allowed to manage the canister. If unset, it will default to the caller. If you're using `dfx`, and haven't specified `--no-wallet`, that's your wallet principal, not your own, so be careful!
- `name`: The name of your NFT collection. Required.
- `symbol`: A short slug identifying your NFT collection. Required.
- `logo`: The logo of your NFT collection, represented as a record with fields `data` (the base-64 encoded logo) and `logo_type` (the MIME type of the logo file). If unset, it will default to the Internet Computer logo.

Example initialization:
```sh
dfx deploy --no-wallet --argument \
"(record {
    name = \"Numbers One Through Fifty\";
    symbol = \"NOTF\";
    logo = opt record {
        data = \"$(base64 -i ./logo.png)\";
        logo_type = \"image/png\";
    };
    custodians = opt vec { principal \"$(dfx identity get-principal)\" };
})"
```

## Interface

Aside from the standard functions, it has five extra functions:

- `set_name`, `set_symbol`, `set_logo`, and `set_custodian`: Update the collection information of the corresponding field from when it was initialized.
- `is_custodian`: Checks whether the specified user is a custodian.

The canister also supports a certified HTTP interface; going to `/<nft>/<id>` will return `nft`'s metadata file #`id`, with `/<nft>` returning the first non-preview file.

## Minting

Due to size limitations on the length of a terminal command, an image- or video-based NFT would be impossible to send via `dfx`. To that end, there is an experimental [minting tool][mint] you can use to mint a single-file NFT. As an example, to mint the default logo, you would run the following command:

```sh
minting-tool local "$(dfx canister id dip721-nft-container)" --owner "$(dfx identity get-principal)" --file ./logo.png --sha2-auto
```

## Demo

This example comes with a demo script, `demo.sh`, which runs through an example workflow with minting and trading an NFT between a few users. Meant primarily to be read rather than run, you can use it to see how basic NFT operations are done. For a more in-depth explanation, read the [standard][DIP721].

[DFX]: https://smartcontracts.org/docs/developers-guide/install-upgrade-remove.html
[Rust]: https://rustup.rs
[DIP721]: https://github.com/Psychedelic/DIP721
[mint]: https://github.com/dfinity/experimental-minting-tool
