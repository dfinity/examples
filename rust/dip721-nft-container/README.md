---
keywords: [advanced, rust, nft, dip721]
---

# DIP721 NFT 

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/dip721-nft-container)

This example demonstrates implementing an NFT canister. NFTs (non-fungible tokens) are unique tokens with arbitrary
metadata, usually an image of some kind, to form the digital equivalent of trading cards. There are a few different
NFT standards for the Internet Computer (e.g [EXT](https://github.com/Toniq-Labs/extendable-token), [IC-NFT](https://github.com/rocklabs-io/ic-nft)), but this tutorial will use [DIP-721](https://github.com/Psychedelic/DIP721). You can see a quick introduction on [YouTube](https://youtu.be/1po3udDADp4).

The canister is a basic implementation of the standard, with support for the minting, burning, and notification interface extensions.

The sample code is available in the [samples repository](https://github.com/dfinity/examples) in [Rust](https://github.com/dfinity/examples/tree/master/rust/dip721-nft-container) and [Motoko](https://github.com/dfinity/examples/tree/master/motoko/dip721-nft-container).

Command-line length limitations would prevent you from minting an NFT with a large file, like an image or video, via `dfx`. To that end,
there is a [command-line minting tool](https://github.com/dfinity/experimental-minting-tool) provided for minting simple NFTs.

## Overview
The NFT canister is not very complicated since the [DIP-721](https://github.com/Psychedelic/DIP721) standard specifies most [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations,
but we can still use it to explain three important concepts concerning dapp development for the Internet Computer:

 ### 1. Stable memory for canister upgrades.
The Internet Computer employs [orthogonal persistence](https://internetcomputer.org/docs/current/motoko/getting-started/motoko-introduction), so developers generally do not need to think a lot about storing their data.
When upgrading canister code, however, it is necessary to explicitly handle canister data. The NFT canister example shows how stable memory can be handled using `pre_upgrade` and `post_upgrade`.

 ### 2. Certified data.
Generally, when a function only reads data, instead of modifying the state of the canister, it is
beneficial to use a [query call instead of an update call](https://internetcomputer.org/docs/current/concepts/canisters-code.md#query-and-update-methods).
But, since query calls do not go through consensus, [certified responses](https://internetcomputer.org/docs/current/developer-docs/security/general-security-best-practices)
should be used wherever possible. The HTTP interface of the Rust implementation shows how certified data can be handled.

 ### 3. Delegating control over assets.
For a multitude of reasons, users may want to give control over their assets to other identities, or even delete (burn) an item.
The NFT canister example contains all those cases and shows how it can be done.

## Architecture
Since the basic functions required in [DIP-721](https://github.com/Psychedelic/DIP721) are very straightforward to implement, this section only discusses how the above ideas are handled and implemented.

### Stable storage for canister upgrades
During canister code upgrades, memory does not persist between different canister calls. Only memory in stable memory is carried over.
Because of that, it is necessary to write all data to stable memory before the upgrade happens, which is usually done in the `pre_upgrade` function.
This function is called by the system before the upgrade happens. After the upgrade, it is normal to load data from stable memory into memory
during the `post_upgrade` function. The `post_upgrade` function is called by the system after the upgrade happened.
In case an error occurs during any part of the upgrade (including `post_upgdrade`), the entire upgrade is reverted.

The Rust CDK (Canister Development Kit) currently only supports one value in stable memory, so it is necessary to create an object that can hold everything you care about.
In addition, not every data type can be stored in stable memory; only ones that implement the [CandidType trait](https://docs.rs/candid/latest/candid/types/trait.CandidType.html)
(usually via the [CandidType derive macro](https://docs.rs/candid/latest/candid/derive.CandidType.html)) can be written to stable memory. 

Since the state of our canister includes an `RbTree` which does not implement the `CandidType`, it has to be converted into a data structure (in this case a `Vec`) that implements `CandidType`.
Luckily, both `RbTree` and `Vec` implement functions that allow converting to/from iterators, so the conversion can be done quite easily.
After conversion, a separate `StableState` object is used to store data during the upgrade.

### Certified data
To serve assets via HTTP over `<canister-id>.icp0.io` instead of `<canister-id>.raw.icp0.io`, responses have to
[contain a certificate](https://wiki.internetcomputer.org/wiki/HTTP_asset_certification) to validate their content.
Obtaining such a certificate can not happen during a query call since it has to go through consensus, so it has to be created during an update call.

A certificate is very limited in its content. At the time of writing, canisters can submit no more than 32 bytes of data to be certified.
To make the most out of that small amount of data, a `HashTree` (the `RbTree` from the previous section is also a `HashTree`) is used.
A `HashTree` is a tree-shaped data structure where the whole tree can be summarized (hashed) into one small hash of 32 bytes.
Whenever some content of the tree changes, the hash also changes. If the hash of such a tree is certified, it means that the content of the tree can be considered certified.
To see how data is certified in the NFT example canister, look at the function `add_hash` in `http.rs`.

For the response to be verified, it has to be checked that a) the served content is part of the tree, and b) the tree containing that content actually can be hashed to the certified hash.
The function `witness` is responsible for creating a tree with minimal content that still can be verified to fulfill a) and b).
Once this minimal tree is constructed, the certificate and minimal hash tree are sent as part of the `IC-Certificate` header.

For a much more detailed explanation of how certification works, see [this explanation video](https://internetcomputer.org/how-it-works/response-certification).

### Managing control over assets
[DIP-721](https://github.com/Psychedelic/DIP721) specifies multiple levels of control over the NFTs:
- **Owner**: this person owns an NFT. They can transfer the NFT, add/remove operators, or burn the NFT.
- **Operator**: sort of a delegated owner. The operator does not own the NFT but can do the same actions an owner can do.
- **Custodian**: creator of the NFT collection/canister. They can do anything (transfer, add/remove operators, burn, and even un-burn) to NFTs, but also mint new ones or change the symbol or description of the collection.

The NFT example canister keeps access control in these three levels very simple: 
- For every level of control, a separate list (or set) of principals is kept.
- Those three levels are then manually checked every single time someone attempts to do something for which they require authorization.
- If a user is not authorized to call a certain function an error is returned.

Burning an NFT is a special case. To burn an NFT means to either delete the NFT (not intended in DIP-721) or to set ownership to `null` (or a similar value).
On the Internet Computer, this non-existing principal is called the [management canister](https://internetcomputer.org/docs/current/references/ic-interface-spec.md#the-ic-management-canister).
> "The IC management canister is just a facade; it does not exist as a canister (with isolated state, Wasm code, etc.)," and its address is `aaaaa-aa`.
Using this management canister address, we can construct its principal and set the management canister as the owner of a burned NFT.

## NFT sample code tutorial

A running instance of the Rust canister for demonstration purposes is available as [t5l7c-7yaaa-aaaab-qaehq-cai](https://t5l7c-7yaaa-aaaab-qaehq-cai.icp0.io).
The interface is meant to be programmatic, but the Rust version additionally contains HTTP functionality so you can view a metadata file at `<canister URL>/<NFT ID>/<file ID>`.
It contains six NFTs, so you can look at items from `<canister URL>/0/0` to `<canister URL>/5/0`.

### Prerequisites

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Download and install [git.](https://git-scm.com/downloads)
- [x] `wasm32-unknown-unknown` targets; these can be installed with `rustup target add wasm32-unknown-unknown`.

 ### Step 1: Clone the Github repo for the project's files and navigate into the directory:

```sh
git clone https://github.com/dfinity/examples
cd examples/rust/dip721-nft-container
```

 ### Step 2: Start the local replica before installing the canister:

```sh
dfx start --background --clean
```

 ### Step 3: Install the canister. 

Deploy the canister with the command:
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

The canister expects a record parameter with the following fields:

- `custodians`: A list of users allowed to manage the canister. If unset, it will default to the caller. If you're using `dfx`, and haven't specified `--no-wallet`, that's your wallet principal, not your own, so be careful!
- `name`: The name of your NFT collection. Required.
- `symbol`: A short slug identifying your NFT collection. Required.
- `logo`: The logo of your NFT collection, represented as a record with fields `data` (the base-64 encoded logo) and `logo_type` (the MIME type of the logo file). If unset, it will default to the Internet Computer logo.

 ### Step 4: Interact with the canister.

Aside from the standard functions, it has five extra functions:

- `set_name`, `set_symbol`, `set_logo`, and `set_custodian`: these functions update the collection information of the corresponding field from when it was initialized.
- `is_custodian`: this function checks whether the specified user is a custodian.

The canister also supports a certified HTTP interface; going to `/<nft>/<id>` will return `nft`'s metadata file #`id`, with `/<nft>` returning the first non-preview file.

Remember that query functions are uncertified; the result of functions like `ownerOfDip721` can be modified arbitrarily by a single malicious node. If queried information is depended on, for example, if someone might send ICP to the owner of a particular NFT to buy it from them, those calls should be performed as update calls instead. You can force an update call by passing the `--update` flag to `dfx` or using the `Agent::update` function in `agent-rs`.

 ### Step 5: Mint an NFT. 

Due to size limitations on the length of a terminal command, an image- or video-based NFT would be impossible to send via `dfx`. To that end, there is an experimental [minting tool](https://github.com/dfinity/experimental-minting-tool) you can use to mint a single-file NFT. 

To use this tool, install the minting tool with the command:

`cargo install --git https://github.com/dfinity/experimental-minting-tool --locked`

As an example, to mint the default logo, you would run the following command:

```sh
minting-tool local "$(dfx canister id dip721_nft_container)" --owner "$(dfx identity get-principal)" --file ./logo.png --sha2-auto
```

The output of this command should look like this:

```
Successfully minted token 0 to x4d3z-ufpaj-lpxs4-v7gmt-v56ze-aub3k-bvifl-y4lsq-soafd-d3i4k-fqe (transaction id 0)
```

Minting is restricted to anyone authorized with the `custodians` parameter or the `set_custodians` function. Since the contents of `--file` are stored on-chain, it's important to prevent arbitrary users from minting tokens, or they will be able to store arbitrarily-sized data in the contract and exhaust the canister's cycles. Be careful not to upload too much data to the canister yourself, or the contract will no longer be able to be upgraded afterward.

#### Demo

This Rust example comes with a demo script, `demo.sh`, which runs through an example workflow with minting and trading an NFT between a few users. This is primarily designed to be read rather than run so that you can use it to see how basic NFT operations are done. For a more in-depth explanation, read the [standard][DIP721].

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls can e.g. lead to time-of-check time-of-use or double spending security bugs.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important NFT data in the frontend that may be used by users to decide on future transactions.
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect when dealing with NFTs.

## Resources
- [Rust](https://rustup.rs).
- [DIP721](https://github.com/Psychedelic/DIP721).
- [mint](https://github.com/dfinity/experimental-minting-tool).
