# DIP721 NFT container

This example demonstrates implementing an NFT canister. NFTs (non-fungible tokens) are unique tokens with arbitrary
metadata, usually an image of some kind, to form the digital equivalent of trading cards. There are a few different
NFT standards for the Internet Computer (e.g [EXT](https://github.com/Toniq-Labs/extendable-token), [IC-NFT](https://github.com/rocklabs-io/ic-nft)), but for the purposes of this tutorial we use [DIP-721](https://github.com/Psychedelic/DIP721). You can see a quick introduction on [YouTube](https://youtu.be/1po3udDADp4).

The canister is a basic implementation of the standard, with support for the minting, burning, and notification interface extensions.

The sample code is available in the [samples repository](https://github.com/dfinity/examples) in [Rust](https://github.com/dfinity/examples/tree/master/rust/dip721-nft-container) and [Motoko](https://github.com/dfinity/examples/tree/master/motoko/dip721-nft-container).

Command-line length limitations would prevent you from minting an NFT with a large file, like an image or video, via `dfx`. To that end,
there is a [command-line minting tool](https://github.com/dfinity/experimental-minting-tool) provided for minting simple NFTs.

## Overview
The NFT canister is not very complicated since the [DIP-721](https://github.com/Psychedelic/DIP721) standard specifies most [CRUD](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) operations,
but we can still use it to explain three important concepts concerning dapp development for the Internet Computer:

 ### 1. Stable memory for canister upgrades.
The Internet Computer employs [orthogonal persistence](https://internetcomputer.org/docs/current/motoko/main/motoko.md#orthogonal-persistence), so developers generally do not need to think a lot about storing their data.
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
During canister code upgrades, memory is not persisted between different canister calls. Only memory in stable memory is carried over.
Because of that it is necessary to write all data to stable memory before the upgrade happens, which is usually done in the `pre_upgrade` function.
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
Once this minimal tree is constructed, certificate and minimal hash tree are sent as part of the `IC-Certificate` header.

For a much more detailed explanation how certification works, see [this explanation video](https://internetcomputer.org/how-it-works/response-certification).

### Managing control over assets
[DIP-721](https://github.com/Psychedelic/DIP721) specifies multiple levels of control over the NFTs:
- **Owner**: this person owns an NFT. They can transfer the NFT, add/remove operators, or burn the NFT.
- **Operator**: sort of a delegated owner. The operator does not own the NFT, but can do the same actions an owner can do.
- **Custodian**: creator of the NFT collection/canister. They can do anything (transfer, add/remove operators, burn, and even un-burn) to NFTs, but also mint new ones or change the symbol or description of the collection.

The NFT example canister keeps access control in these three levels very simple: 
- For every level of control, a separate list (or set) of principals is kept.
- Those three levels are then manually checked every single time someone attempts to do something for which they require authorization.
- If a user is not authorized to call a certain function an error is returned.

Burning an NFT is a special case. To burn an NFT means to either delete the NFT (not intended in DIP-721) or to set ownership to `null` (or a similar value).
On the Internet Computer, this non-existing principal is called the [management canister](https://internetcomputer.org/docs/current/references/ic-interface-spec.md#the-ic-management-canister).
> "The IC management canister is just a facade; it does not actually exist as a canister (with isolated state, Wasm code, etc.)," and its address is `aaaaa-aa`.
Using this management canister address, we can construct its principal and set the management canister as the owner of a burned NFT.

## NFT sample code tutorial

### Prerequisites 

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Download and install [git.](https://git-scm.com/downloads)

 ### Step 1: Clone the examples repo:

```
git clone git@github.com:dfinity/examples.git
```

 ### Step 2: Navigate to DIP721 project root:

```
cd examples/motoko/dip-721-nft-container
```

 ### Step 3: Run a local instance of the Internet Computer:

```
dfx start --background 
```

**If this is not a new installation, you may need to run `start` with the `--clean` flag.**

```
dfx start --clean --background
```

 ### Step 4: Deploy a DIP721 NFT canister to your local IC.
This command deploys the DIP721 NFT canister with the following initialization arguments:

```
dfx deploy --argument "(
  principal\"$(dfx identity get-principal)\", 
  record {
    logo = record {
      logo_type = \"image/png\";
      data = \"\";
    };
    name = \"My DIP721\";
    symbol = \"DFXB\";
    maxLimit = 10;
  }
)"
```

#### What this does
- `principal`: the initial custodian of the collection. A custodian is a user who can administrate the collection i.e. an "Admin" user. 

  :::info
  `"$(dfx identity get-principal)"` automatically interpolates the default identity used by dfx on your machine into the argument that gets passed to `deploy`.
  :::

- `logo`: The image that represents this NFT collection.
- `name`: The name of the NFT collection.
- `symbol`: A short, unique symbol to identify the token. 
- `maxLimit`: The maximum number of NFTs that are allowed in this collection.

You will receive output that resembles the following:

```
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    dip721_nft_container: http://127.0.0.1:4943/?canisterId=br5f7-7uaaa-aaaaa-qaaca-cai&id=be2us-64aaa-aaaaa-qaabq-cai
```

 ### Step 5: Mint an NFT.

Use the following command to mint an NFT:

```
dfx canister call dip721_nft_container mintDip721 \
"(
  principal\"$(dfx identity get-principal)\", 
  vec { 
    record {
      purpose = variant{Rendered};
      data = blob\"hello\";
      key_val_data = vec {
        record { key = \"description\"; val = variant{TextContent=\"The NFT metadata can hold arbitrary metadata\"}; };
        record { key = \"tag\"; val = variant{TextContent=\"anime\"}; };
        record { key = \"contentType\"; val = variant{TextContent=\"text/plain\"}; };
        record { key = \"locationType\"; val = variant{Nat8Content=4:nat8} };
      }
    }
  }
)"
```

If this succeeds, you should see the following message:

```
(variant { Ok = record { id = 1 : nat; token_id = 0 : nat64 } })
```

 ### Step 6: Transferring an NFT.
The DIP721 interface supports transferring an NFT to some other `principal` values via the `transferFromDip721` or `safeTransferFromDip721` methods.

First, create a different identity using DFX. This will become the principal that you receives the NFT

```
dfx identity new --disable-encryption alice
ALICE=$(dfx --identity alice identity get-principal)
```

Verify the identity for `ALICE` was created and set as an environment variable:
```
echo $ALICE
```

You should see a principal get printed
```
o4f3h-cbpnm-4hnl7-pejut-c4vii-a5u5u-bk2va-e72lb-edvgw-z4wuq-5qe
```

Transfer the NFT from the default user to `ALICE`. 

Here the arguments are:
`from`: principal that owns the NFT
`to`: principal to transfer the NFT to
`token_id`: the id of the token to transfer

```
dfx canister call dip721_nft_container transferFromDip721 "(principal\"$(dfx identity get-principal)\", principal\"$ALICE\", 0)"
```

Transfer the NFT from from `ALICE` back to the default user.

```
dfx canister call dip721_nft_container safeTransferFromDip721 "(principal\"$ALICE\", principal\"$(dfx identity get-principal)\", 0)"
```

Note the second transfer works because the caller is in the list of custodians, i.e. the default user has admin rights to modify the NFT collection.

### Other methods

 ### balanceOfDip721

```
dfx canister call dip721_nft_container balanceOfDip721 "(principal\"$(dfx identity get-principal)\")"
```

Output:

```
(1 : nat64)
```

 ### getMaxLimitDip721

```
dfx canister call dip721_nft_container getMaxLimitDip721
```

Output:

```
(10 : nat16)
```

 ### getMetadataDip721

Provide a token ID. 
The token ID was provided to you when you ran `mintDip721`, e.g. `(variant { Ok = record { id = 1 : nat; token_id = 0 : nat64 } })` So, the token ID is 0 in this case.

```
dfx canister call dip721_nft_container getMetadataDip721 "0"
```

Output:

```
(
  variant {
    Ok = vec {
      record {
        data = blob "hello";
        key_val_data = vec {
          record {
            key = "description";
            val = variant {
              TextContent = "The NFT metadata can hold arbitrary metadata"
            };
          };
          record { key = "tag"; val = variant { TextContent = "anime" } };
          record {
            key = "contentType";
            val = variant { TextContent = "text/plain" };
          };
          record {
            key = "locationType";
            val = variant { Nat8Content = 4 : nat8 };
          };
        };
        purpose = variant { Rendered };
      };
    }
  },
)
```


 ### getMetadataForUserDip721

```
dfx canister call dip721_nft_container getMetadataForUserDip721 "(principal\"$(dfx identity get-principal)\")"
```

Output:

```
(
  variant {
    Ok = record {
      token_id = 0 : nat64;
      metadata_desc = vec {
        record {
          data = blob "hello";
          key_val_data = vec {
            record {
              key = "description";
              val = variant {
                TextContent = "The NFT metadata can hold arbitrary metadata"
              };
            };
            record { key = "tag"; val = variant { TextContent = "anime" } };
            record {
              key = "contentType";
              val = variant { TextContent = "text/plain" };
            };
            record {
              key = "locationType";
              val = variant { Nat8Content = 4 : nat8 };
            };
          };
          purpose = variant { Rendered };
        };
      };
    }
  },
)
```

 ### getTokenIdsForUserDip721

```
dfx canister call dip721_nft_container getTokenIdsForUserDip721 "(principal\"$(dfx identity get-principal)\")"
```

Output:

```
(vec { 0 : nat64 })
```

 ### logoDip721

```
dfx canister call dip721_nft_container logoDip721
```

Output:

```
(record { data = ""; logo_type = "image/png" })
```

 ### nameDip721

```
dfx canister call dip721_nft_container nameDip721
```

Output:

```
("My DIP721")
```

 ### supportedInterfacesDip721

```
dfx canister call dip721_nft_container supportedInterfacesDip721
```

Output:

```
(vec { variant { TransferNotification }; variant { Burn }; variant { Mint } })
```

 ### symbolDip721

```
dfx canister call dip721_nft_container symbolDip721
```

Output:
```
("DFXB")
```

 ### totalSupplyDip721

```
dfx canister call dip721_nft_container totalSupplyDip721
```

Output:

```
(1 : nat64)
```

 ### ownerOfDip721
Provide a token ID. 
The token ID was provided to you when you ran `mintDip721`, e.g. `(variant { Ok = record { id = 1 : nat; token_id = 0 : nat64 } })` So, the token ID is 0 in this case.

```
dfx canister call dip721_nft_container ownerOfDip721 "0"
```

Output:

```
(
  variant {
    Ok = principal "5wuse-ejxao-gkqq6-4dhl5-hn5ps-2mgop-2se4s-w4zle-agr6j-svlhq-3qe"
  },
)
```

Verify that this is the same principal that you ran `mintDip721` with:

```
dfx identity get-principal
```

## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls can e.g. lead to time-of-check time-of-use or double spending security bugs.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important NFT data in the frontend that may be used by users to decide on future transactions.
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect when dealing with NFTs.

## Resources
[Rust](https://rustup.rs).
[DIP721](https://github.com/Psychedelic/DIP721).
[Minting tool](https://github.com/dfinity/experimental-minting-tool).