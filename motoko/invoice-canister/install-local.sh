#!/usr/bin/env bash

set -e

dfx stop
dfx start --background --clean

dfx identity new minter --disable-encryption
dfx identity use minter
export MINT_ACC=$(dfx ledger account-id)

dfx identity use default
export LEDGER_ACC=$(dfx ledger account-id)
export TEST_ACC="cd60093cef12e11d7b8e791448023348103855f682041e93f7d0be451f48118b"

# Use private api for install
rm src/ledger/ledger.did
cp src/ledger/ledger.private.did src/ledger/ledger.did

dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; record { "'${TEST_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'

# Replace with public api
rm src/ledger/ledger.did
cp src/ledger/ledger.public.did src/ledger/ledger.did

dfx deploy invoice
