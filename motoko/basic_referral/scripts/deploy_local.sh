#!/bin/bash

vessel install

### === DEPLOY LOCAL LEDGER =====
dfx identity new owner --disable-encryption
dfx identity use owner
MINT_ACC=$(dfx ledger account-id)

dfx identity use default
LEDGER_ACC=$(dfx ledger account-id)

# Use private api for install
rm src/ledger/ledger.did
cp src/ledger/ledger.private.did src/ledger/ledger.did

dfx deploy ledger --argument '(record {
  minting_account = "'${MINT_ACC}'";
  initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000_000 } }; };
  send_whitelist = vec {}
  })'

# Replace with public api
rm src/ledger/ledger.did
cp src/ledger/ledger.public.did src/ledger/ledger.did
dfx canister call ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$LEDGER_ACC'")]) + "}")')' })'

# update ./src/backend/.env.mo
LEDGER_ID=$(dfx canister id ledger)

FILE="./src/backend/.env.mo"

/bin/cat <<EOM >$FILE
module Env {
  public let LEDGER_ID = "${LEDGER_ID}";
}
EOM

## === DEPLOY BACKEND ====
dfx deploy basic_referral
## === Transfer ICP to DAO's default subaccount ===
SYSTEM_ADDR=$(dfx canister call basic_referral getSystemAddress | tr -d '\n' | sed 's/,)/)/')
echo $SYSTEM_ADDR
dfx canister call ledger transfer "(record { amount = record { e8s = 10_000_000_000 }; to = $SYSTEM_ADDR; fee = record { e8s = 10_000}; memo = 1;})"
dfx canister call basic_referral getSystemBalance
