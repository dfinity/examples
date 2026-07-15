#!/usr/bin/env bash
set -e

# Local deploy for ic-pos. Use `bash deploy.sh` (not `icp deploy`): the ICRC-1
# ledger and index require init args (minting account, initial balances, ledger
# id) that are only known after identities exist.
#
# For mainnet, deploy with `icp deploy -e ic` instead — the ledger, index, and
# Internet Identity are not deployed there (the app uses the shared TICRC1 test
# token and the production II; see icp.yaml and the README).
#
# Prerequisite: a running local network (`icp network start -d`).

# The default identity is the ledger's minting account. A separate, pre-funded
# "ic-pos-dev" identity lets you pay merchants locally with a real transfer via
# `icp canister call icrc1_ledger icrc1_transfer ...` — minting (a transfer from
# the minting account) would create a "mint", not the "transfer" the backend
# monitors.
MINTER=$(icp identity principal)
icp identity new ic-pos-dev --storage plaintext 2>/dev/null || true
DEV=$(icp identity principal --identity ic-pos-dev)

# 1. ICRC-1 ledger — minting account = default identity; ic-pos-dev pre-funded.
icp deploy icrc1_ledger --mode reinstall -y --args "(variant { Init = record { \
  token_name = \"Test ICRC1\"; \
  token_symbol = \"TICRC1\"; \
  minting_account = record { owner = principal \"$MINTER\" }; \
  initial_balances = vec { record { record { owner = principal \"$DEV\" }; 1_000_000_000_000 : nat } }; \
  metadata = vec {}; \
  transfer_fee = 10_000 : nat; \
  archive_options = record { \
    trigger_threshold = 2000 : nat64; \
    num_blocks_to_archive = 1000 : nat64; \
    controller_id = principal \"$MINTER\" }; \
  feature_flags = opt record { icrc2 = true } } })"

# 2. ICRC-1 index — points at the ledger we just deployed.
LEDGER_ID=$(icp canister status icrc1_ledger -i)
icp deploy icrc1_index --mode reinstall -y --args "(opt variant { Init = record { \
  ledger_id = principal \"$LEDGER_ID\"; \
  retrieve_blocks_from_ledger_interval_seconds = opt (1 : nat64) } })"

# 3. Internet Identity, backend, and frontend (init args come from icp.yaml).
icp deploy internet_identity -y
icp deploy icpos -y
icp deploy icpos_frontend -y

echo
echo "Deployed. Pay a merchant from the pre-funded ic-pos-dev identity, e.g.:"
echo "  icp canister call icrc1_ledger icrc1_transfer \\"
echo "    '(record { to = record { owner = principal \"<MERCHANT_PRINCIPAL>\" }; amount = 100_000 : nat })' \\"
echo "    --identity ic-pos-dev"
