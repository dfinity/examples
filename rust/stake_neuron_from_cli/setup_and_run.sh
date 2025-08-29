#!/usr/bin/env bash
set -e

echo "===========STARTING LOCAL IC========="
# Stop any existing dfx instance
dfx stop 2>/dev/null || true

# Start clean local IC
dfx start --background --clean

# Create a test identity for easier testing
dfx identity new test_user --storage-mode plaintext --force || true
dfx identity use test_user

echo "===========SETTING UP ACCOUNTS========="
# Get account identifiers for initialization
export MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

echo "Minter account: $MINTER_ACCOUNT_ID"
echo "Default account: $DEFAULT_ACCOUNT_ID" 

echo "===========DEPLOYING LEDGER========="
# Deploy ICP ledger with initial balance
dfx deploy icp_ledger --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 1_000_000_000_000 : nat64;
          };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record {
        e8s = 10_000 : nat64;
      };
      token_symbol = opt \"LICP\";
      token_name = opt \"Local ICP\";
    }
  })
"

echo "===========CHECKING LEDGER BALANCE========="
# Verify ledger is working
dfx canister call icp_ledger account_balance "(record { account = $(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")') })"

echo "===========DEPLOYING GOVERNANCE========="
# Deploy NNS governance (this might fail on first try due to complexity, but let's see)
dfx deploy nns_governance --argument "
  (record {
    economics = opt record {
      neuron_minimum_stake_e8s = 100_000_000 : nat64;
      max_proposals_to_keep_per_topic = 100 : nat32;
      neuron_management_fee_per_proposal_e8s = 1_000_000 : nat64;
      reject_cost_e8s = 1_000_000 : nat64;
      transaction_fee_e8s = 10_000 : nat64;
      neuron_spawn_dissolve_delay_seconds = 604_800 : nat64;
      minimum_icp_xdr_rate = 2_590 : nat64;
      maximum_node_provider_rewards_e8s = 1_000_000_000 : nat64;
    };
    genesis_timestamp_seconds = $(date +%s) : nat64;
    proposals = vec {};
    neurons = vec {};
    in_flight_commands = vec {};
  })
" || echo "Note: Governance deployment might need manual initialization"

echo "===========SETUP COMPLETE========="
echo "Local IC is running with:"
echo "- ICP Ledger: $(dfx canister id icp_ledger)"  
echo "- NNS Governance: $(dfx canister id nns_governance)" 

echo ""
echo "===========RUNNING CLI TOOL========="
cargo run

echo ""
echo "===========SETUP REMAINS ACTIVE========="
echo "The local IC is still running. You can now:"
echo "- Query canisters: dfx canister call <canister> <method> '<args>'"
echo "- Check status: dfx canister status <canister>"
echo "- View canister IDs: dfx canister id <canister>"
echo ""
echo "To stop the local IC later, run: dfx stop"
echo "Current replica URL: http://127.0.0.1:8080"
