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

# Two dedicated identities:
#  - ic-pos-minter: the ledger's minting account (mints the initial supply).
#  - ic-pos-dev:    a normal, pre-funded holder you pay merchants from.
# These must be DIFFERENT identities — and independent of your current default —
# so that transfers from ic-pos-dev are real "transfer"s (debited, with a fee)
# that the backend monitor picks up. If the minter and the holder were the same
# identity, its "transfers" would be mints: no debit, no fee, and the monitor
# (which only reacts to transfers) would ignore them.
icp identity new ic-pos-minter --storage plaintext 2>/dev/null || true
icp identity new ic-pos-dev --storage plaintext 2>/dev/null || true
MINTER=$(icp identity principal --identity ic-pos-minter)
DEV=$(icp identity principal --identity ic-pos-dev)

# 1. ICRC-1 ledger — minting account = ic-pos-minter; ic-pos-dev pre-funded.
# Named distinctly from the shared mainnet TICRC1 token to make clear this is a
# throwaway local ledger, not the real thing.
icp deploy icrc1_ledger --mode reinstall -y --args "(variant { Init = record { \
  token_name = \"Local ICRC-1\"; \
  token_symbol = \"LICRC1\"; \
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

# 3. Backend and frontend (init args come from icp.yaml). Internet Identity is
# provided by the local network (ii: true in icp.yaml), not deployed here.
icp deploy backend
icp deploy frontend

echo
echo "Deployed. The test tokens are held by the 'ic-pos-dev' identity"
echo "(your default identity has none). Pass --identity ic-pos-dev to spend"
echo "them — no need to change your selected identity:"
echo
echo "  # check the balance"
echo "  icp token \$(icp canister status icrc1_ledger -i) balance --identity ic-pos-dev"
echo
echo "  # pay a merchant 1 LICRC1 (amounts are in base units; 8 decimals),"
echo "  # a real transfer the backend monitor picks up"
echo "  icp canister call icrc1_ledger icrc1_transfer \\"
echo "    '(record { to = record { owner = principal \"<MERCHANT_PRINCIPAL>\" }; amount = 100_000_000 : nat })' \\"
echo "    --identity ic-pos-dev"
