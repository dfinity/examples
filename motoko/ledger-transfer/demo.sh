#!/usr/bin/env bash
dfx stop
set -e
trap 'dfx stop' EXIT

export IC_VERSION=2cb0afe1f49b8bbd4e60db234ca1f4a6f68ea115
test -f ledger.wasm.gz || curl -o ledger.wasm.gz https://download.dfinity.systems/ic/${IC_VERSION}/canisters/ledger-canister_notify-method.wasm.gz
test -f ledger.wasm || gunzip ledger.wasm.gz
test -f ledger.private.did || curl -o ledger.private.did https://raw.githubusercontent.com/dfinity/ic/${IC_VERSION}/rs/rosetta-api/ledger.did
test -f ledger.public.did || curl -o ledger.public.did https://raw.githubusercontent.com/dfinity/ic/${IC_VERSION}/rs/rosetta-api/ledger_canister/ledger.did

dfx start --background --clean
dfx identity new alice --disable-encryption || true
cat <<<"$(jq '.canisters.ledger.candid="ledger.private.did"' dfx.json)" >dfx.json
export MINT_ACC=$(dfx --identity anonymous ledger account-id)
export LEDGER_ACC=$(dfx ledger account-id)
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)
dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'
cat <<<"$(jq '.canisters.ledger.candid="ledger.public.did"' dfx.json)" >dfx.json
dfx canister call ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$LEDGER_ACC'")]) + "}")')' })'

dfx deploy ledger_transfer
RESPONSE="$(dfx canister call ledger_transfer canisterAccount '()')"
BLOB=$(echo "$RESPONSE" | grep blob | sed 's/,//g')
dfx canister call ledger transfer "(record { to = $BLOB; memo = 1; amount = record { e8s = 2_00_000_000 }; fee = record { e8s = 10_000 }; })"
dfx --identity alice canister call ledger_transfer post "(\"Nom Nom Love Donuts\")"
dfx canister call ledger_transfer distributeRewards '()'

echo "DONE"
