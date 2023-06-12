#!/usr/bin/env bash
dfx stop
set -e
trap 'dfx stop' EXIT

dfx start --background --clean
dfx identity new alice --disable-encryption || true
ALICE=$(dfx --identity alice identity get-principal)
dfx identity new bob --disable-encryption || true
BOB=$(dfx --identity bob identity get-principal)

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

dfx canister call dip721_nft_container transferFromDip721 "(principal\"$(dfx identity get-principal)\", principal\"$ALICE\", 0)"
dfx canister call dip721_nft_container safeTransferFromDip721 "(principal\"$ALICE\", principal\"$(dfx identity get-principal)\", 0)"
dfx canister call dip721_nft_container balanceOfDip721 "(principal\"$(dfx identity get-principal)\")"

echo "DONE"
