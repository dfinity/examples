#!/bin/bash

# parse arguments
ICP_SUPPLY=$1
AKITA_SUPPLY=$2

### === DEPLOY LOCAL LEDGER =====
dfx identity new minter
dfx identity use minter
export MINT_ACC=$(dfx ledger account-id)

dfx identity use default
export LEDGER_ACC=$(dfx ledger account-id)

# Use private api for install
rm src/ledger/ledger.did
cp src/ledger/ledger.private.did src/ledger/ledger.did

dfx deploy ledger --argument '(record  {
    minting_account = "'${MINT_ACC}'";
    initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s='${ICP_SUPPLY}' } }; };
    send_whitelist = vec {}
    })'

# Replace with public api
rm src/ledger/ledger.did
cp src/ledger/ledger.public.did src/ledger/ledger.did

### === DEPLOY DIP TOKENS =====

dfx canister --no-wallet create AkitaDIP20
 dfx build AkitaDIP20

export ROOT_PRINCIPAL="principal \"$(dfx identity get-principal)\""
dfx canister --no-wallet install AkitaDIP20 --argument="(\"https://dogbreedslist.com/wp-content/uploads/2019/08/Are-Golden-Retrievers-easy-to-train.png\", \"Golden Coin\", \"GLD\", 8, $AKITA_SUPPLY, $ROOT_PRINCIPAL, 10000)"
export LEDGER=$(dfx canister --no-wallet id ledger)

# set fees 
dfx canister call AkitaDIP20 setFeeTo '(principal "'${LEDGER}'")'
dfx canister call AkitaDIP20 setFee "(420)" 

dfx deploy defi_dapp