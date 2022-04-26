# DIP721 NFT Container

# Getting Started

## Install DFX
```
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

## Clone the Examples Project
```
git clone git@github.com:dfinity/examples.git
```

## Navigate to DIP721 Project root
```
cd examples/motoko/dip-721-nft-container
```

## Run Local Internet Computer
```
dfx start --background 
```

Note: if this is not a new installation, you may need to run `start` with the `--clean` flag

```
dfx start --clean --background
```

## Deploy DIP721 NFT canister to your local IC
This deploys the DIP721 NFT canister with the following initialization arguments:
- principal: the initial custodian of the collection. a custodian is a user who can administrate the collection i.e. an "Admin" user. 
  Note: `"$(dfx identity get-principal)"` automatically interpolates the default identity used by dfx on your machine into the argument that gets passed to `deploy`.
- logo: The image that represents this NFT collection
- name: The name of the NFT collection
- symbol: A short, unique symbol to identify the token. 
- maxLimit: The maximum number of NFTs that are allowed in this collection.

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

## Mint an NFT

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

## Transfering an NFT
The DIP721 interface supports transfering an NFT to some other Principal via the `transferFromDip721` or `safeTransferFromDip721` methods.

First, create a different identity using DFX. This will become the principal that you receives the NFT

```
dfx identity new alice
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

## Other Methods

### balanceOfDip721
```
dfx canister call dip721_nft_container balanceOfDip721 "(principal\"$(dfx identity get-principal)\")"
```

### getMaxLimitDip721
```
dfx canister call dip721_nft_container getMaxLimitDip721
```

### getMetadataDip721
Provide a token ID. 
The token ID was provided to you when you ran `mintDip721`, e.g. `(variant { Ok = record { id = 1 : nat; token_id = 0 : nat64 } })` So, the token ID is 0 in this case.

```
dfx canister call dip721_nft_container getMetadataDip721 "0"
```

### getMetadataForUserDip721
```
dfx canister call dip721_nft_container getMetadataForUserDip721 "(principal\"$(dfx identity get-principal)\")"
```

### getTokenIdsForUserDip721
```
dfx canister call dip721_nft_container getTokenIdsForUserDip721 "(principal\"$(dfx identity get-principal)\")"
```

### logoDip721
```
dfx canister call dip721_nft_container logoDip721
```

### nameDip721
```
dfx canister call dip721_nft_container nameDip721
```

### supportedInterfacesDip721
```
dfx canister call dip721_nft_container supportedInterfacesDip721
```

### symbolDip721
```
dfx canister call dip721_nft_container symbolDip721
```

### totalSupplyDip721
```
dfx canister call dip721_nft_container totalSupplyDip721
```

### ownerOfDip721
Provide a token ID. 
The token ID was provided to you when you ran `mintDip721`, e.g. `(variant { Ok = record { id = 1 : nat; token_id = 0 : nat64 } })` So, the token ID is 0 in this case.

```
dfx canister call dip721_nft_container ownerOfDip721 "0"
```

You should see something like this:

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