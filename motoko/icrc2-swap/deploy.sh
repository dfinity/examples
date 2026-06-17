#!/usr/bin/env bash
set -e

# Deploy all canisters in the correct order:
# 1. Create example-specific test identities (icrc2-alice, icrc2-bob) if not present.
# 2. Deploy token_a (pre-funded for alice) and token_b (pre-funded for bob).
# 3. Deploy backend — discovers token principals automatically via env vars.
#
# Note: use `bash deploy.sh` (not `icp deploy`) — the ledger canisters require
# init args with alice/bob principals that are only available after the
# identities are created.

MINTER=$(icp identity principal)
icp identity new icrc2-alice --storage plaintext 2>/dev/null || true
icp identity new icrc2-bob   --storage plaintext 2>/dev/null || true
ALICE=$(icp identity principal --identity icrc2-alice)
BOB=$(icp identity principal --identity icrc2-bob)

icp deploy token_a --mode reinstall -y --args "(variant { Init = record { \
  token_name = \"Token A\"; token_symbol = \"A\"; \
  minting_account = record { owner = principal \"$MINTER\" }; \
  initial_balances = vec { record { record { owner = principal \"$ALICE\" }; 100_000_000_000_000 : nat } }; \
  metadata = vec {}; transfer_fee = 10_000 : nat; \
  archive_options = record { trigger_threshold = 2000 : nat64; num_blocks_to_archive = 1000 : nat64; controller_id = principal \"$MINTER\" }; \
  feature_flags = opt record { icrc2 = true } } })"

icp deploy token_b --mode reinstall -y --args "(variant { Init = record { \
  token_name = \"Token B\"; token_symbol = \"B\"; \
  minting_account = record { owner = principal \"$MINTER\" }; \
  initial_balances = vec { record { record { owner = principal \"$BOB\" }; 100_000_000_000_000 : nat } }; \
  metadata = vec {}; transfer_fee = 10_000 : nat; \
  archive_options = record { trigger_threshold = 2000 : nat64; num_blocks_to_archive = 1000 : nat64; controller_id = principal \"$MINTER\" }; \
  feature_flags = opt record { icrc2 = true } } })"

icp deploy backend --mode reinstall -y
