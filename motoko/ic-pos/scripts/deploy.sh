#!/bin/bash

# Exit on errors
set -e

# Change to the script's directory
cd "$(dirname "$0")"

# Kills all dfx-related processes on the system.
dfx killall

# Start the local Internet Computer replica
dfx start --clean --background

# Deploy the internet identity canister
dfx deploy internet_identity

# Use the currently active identity as the owner for the token
export OWNER=$(dfx identity get-principal)

# Deploy token ledger
dfx deploy icrc1_ledger --argument '
  (variant {
    Init = record {
      token_name = "Local ckBTC";
      token_symbol = "LCKBTC";
      minting_account = record {
        owner = principal "'${OWNER}'";
      };
      initial_balances = vec {
        record {
          record {
            owner = principal "'${OWNER}'";
          };
          100_000_000_000;
        };
      };
      metadata = vec {};
      transfer_fee = 10;
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal "'${OWNER}'";
      }
    }
  })
'

# Deploy token index canister
dfx deploy icrc1_index --argument '
  record {
   ledger_id = (principal "mxzaz-hqaaa-aaaar-qaada-cai");
  }
'

# Deploy the icpos backend canister
dfx deploy icpos --argument '(0)'

# Install the frontend dependencies:w
pnpm install

# Deploy the icpos frontend canister
dfx deploy icpos_frontend
