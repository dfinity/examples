# DIP721 NFT Container

## Summary

This is an example implementation of an NFT (non-fungible token) smart contract, using the [DIP721] v1 standard. A future version of this example may implement v2 instead.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Inter-Canister Calls and Rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls can e.g. lead to time-of-check time-of-use or double spending security bugs.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important nft data in the frontend that may be used by users to decide on future transactions.
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect when dealing with NFTs.


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

Remember that query functions are uncertified; the result of functions like `ownerOfDip721` can be modified arbitrarily by a single malicious node. If queried information is depended on, for example if someone might send ICP to the owner of a particular NFT to buy it from them, those calls should be performed as update calls instead. You can force an update call by passing the `--update` flag to `dfx` or using the `Agent::update` function in `agent-rs`.

## Minting

Due to size limitations on the length of a terminal command, an image- or video-based NFT would be impossible to send via `dfx`. To that end, there is an experimental [minting tool][mint] you can use to mint a single-file NFT. As an example, to mint the default logo, you would run the following command:

```sh
minting-tool local "$(dfx canister id dip721_nft_container)" --owner "$(dfx identity get-principal)" --file ./logo.png --sha2-auto
```

Minting is restricted to anyone authorized with the `custodians` parameter or the `set_custodians` function. Since the contents of `--file` are stored on-chain, it's important to prevent arbitrary users from minting tokens, or they will be able to store arbitrarily-sized data in the contract and exhaust the canister's cycles. Be careful not to upload too much data to the canister yourself, or the contract will no longer be able to be upgraded afterwards.

## Demo

This example comes with a demo script, `demo.sh`, which runs through an example workflow with minting and trading an NFT between a few users. Meant primarily to be read rather than run, you can use it to see how basic NFT operations are done. For a more in-depth explanation, read the [standard][DIP721].

[DFX]: https://smartcontracts.org/docs/developers-guide/install-upgrade-remove.html
[Rust]: https://rustup.rs
[DIP721]: https://github.com/Psychedelic/DIP721
[mint]: https://github.com/dfinity/experimental-minting-tool
