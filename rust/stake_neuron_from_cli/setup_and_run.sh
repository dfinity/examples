#!/usr/bin/env bash
set -e

echo "===========STARTING LOCAL IC========="
# Stop any existing dfx instance
dfx stop 2>/dev/null || true

# Start clean local IC
dfx start --background --clean

# Wait a moment for the replica to be ready
sleep 3

echo "===========CREATING TEST IDENTITY========="
# Create a test identity for our CLI
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
# Deploy NNS governance with all required fields
dfx deploy nns_governance --argument "
  (record {
    default_followees = vec {};
    making_sns_proposal = null;
    most_recent_monthly_node_provider_rewards = null;
    maturity_modulation_last_updated_at_timestamp_seconds = null;
    wait_for_quiet_threshold_seconds = 86400 : nat64;
    metrics = null;
    neuron_management_voting_period_seconds = null;
    node_providers = vec {};
    cached_daily_maturity_modulation_basis_points = null;
    economics = opt record {
      neuron_minimum_stake_e8s = 100_000_000 : nat64;
      max_proposals_to_keep_per_topic = 100 : nat32;
      neuron_management_fee_per_proposal_e8s = 1_000_000 : nat64;
      reject_cost_e8s = 1_000_000 : nat64;
      transaction_fee_e8s = 10_000 : nat64;
      neuron_spawn_dissolve_delay_seconds = 604_800 : nat64;
      minimum_icp_xdr_rate = 2_590 : nat64;
      maximum_node_provider_rewards_e8s = 1_000_000_000 : nat64;
      neurons_fund_economics = null;
      voting_power_economics = null;
    };
    restore_aging_summary = null;
    spawning_neurons = null;
    latest_reward_event = null;
    to_claim_transfers = vec {};
    short_voting_period_seconds = 345600 : nat64;
    proposals = vec {};
    xdr_conversion_rate = null;
    in_flight_commands = vec {};
    neurons = vec {};
    genesis_timestamp_seconds = $(date +%s) : nat64;
  })
"

echo "===========SETUP COMPLETE========="
echo "Local IC is running with:"
echo "- ICP Ledger: $(dfx canister id icp_ledger)"  
echo "- NNS Governance: $(dfx canister id nns_governance)" 

echo ""
echo "===========RUNNING CLI TOOL========="
# Find the identity file for the test_user
IDENTITY_FILE="$HOME/.config/dfx/identity/test_user/identity.pem"

if [ ! -f "$IDENTITY_FILE" ]; then
    echo "ERROR: Identity file not found at $IDENTITY_FILE"
    echo "Available identities:"
    dfx identity list
    exit 1
fi

echo "Using identity file: $IDENTITY_FILE"
cargo run -- --identity "$IDENTITY_FILE" --url "http://127.0.0.1:4943"

echo ""
echo "===========SETUP REMAINS ACTIVE========="
echo "The local IC is still running. You can now:"
echo "- Query canisters: dfx canister call <canister> <method> '<args>'"
echo "- Check status: dfx canister status <canister>"
echo "- View canister IDs: dfx canister id <canister>"
echo ""
echo "To stop the local IC later, run: dfx stop"
echo "Current replica URL: http://127.0.0.1:4943"