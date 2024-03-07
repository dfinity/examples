#!/usr/bin/env bash
dfx stop
set -e
trap 'dfx stop' EXIT

echo "===========SETUP========="
# Version corresponding to the latest upgrade proposal for the ICP Ledger canister
# https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai
export IC_VERSION=98eb213581b239c3829eee7076bea74acad9937b
test -f ledger.wasm.gz || curl -o ledger.wasm.gz https://download.dfinity.systems/ic/${IC_VERSION}/canisters/ledger-canister_notify-method.wasm.gz
test -f ledger.wasm || gunzip ledger.wasm.gz
test -f ledger.did || curl -o ledger.did https://raw.githubusercontent.com/dfinity/ic/${IC_VERSION}/rs/rosetta-api/icp_ledger/ledger.did
dfx start --background --clean
dfx identity new alice --disable-encryption || true
cat <<<"$(jq '.canisters.ledger.candid="ledger.did"' dfx.json)" >dfx.json
export MINT_ACC=$(dfx --identity anonymous ledger account-id)
export LEDGER_ACC=$(dfx ledger account-id)
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)
dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'
dfx canister call ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$LEDGER_ACC'")]) + "}")')' })'
echo "===========SETUP DONE========="

LEDGER_ID="$(dfx canister id ledger)"
dfx deploy --argument "(record { ledger_canister_id=principal \"${LEDGER_ID}\"; transaction_fee=record { e8s=10_000 }; subaccount=null }, )" tokens_transfer

TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister tokens_transfer)"
TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')" 
dfx canister call ledger transfer "(record { to=${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; amount=record { e8s=100_000 }; fee=record { e8s=10_000 }; memo=0:nat64; }, )"

dfx canister call tokens_transfer transfer "(record { amount=record { e8s=5 }; to_principal=principal \"$(dfx identity get-principal)\" },)"

echo "DONE"
