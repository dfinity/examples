#!/usr/bin/env bash
dfx stop
set -e
trap 'dfx stop' EXIT

echo "===========SETUP========="
dfx start --background --clean
dfx identity new alice_stake_neuron --storage-mode plaintext --force
export MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

# Deploy local ICP ledger for testing
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

# Check initial balance
echo "Initial ICP balance:"
dfx canister call icp_ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")')'})'

echo "===========SETUP DONE========="

# Deploy the stake neuron backend
dfx deploy stake_neuron_backend

# Get canister account and fund it with ICP
CANISTER_ACCOUNT_ID="$(dfx canister call stake_neuron_backend get_canister_account | grep -o '"[^"]*"' | tr -d '"')"
CANISTER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$CANISTER_ACCOUNT_ID'")]) + "}")')"

echo "Funding canister with 10 ICP..."
dfx canister call icp_ledger transfer "(record { to=${CANISTER_ACCOUNT_ID_BYTES}; amount=record { e8s=1_000_000_000 }; fee=record { e8s=10_000 }; memo=0:nat64; }, )"

echo "===========STAKING DEMONSTRATION========="

echo ""
echo "Checking canister ICP balance:"
dfx canister call stake_neuron_backend get_canister_balance

echo ""
echo "Staking 2 ICP with 1 year dissolve delay..."

# Stake 2 ICP (200,000,000 e8s) with 1 year dissolve delay
dfx canister call stake_neuron_backend stake_neuron "(record {
  amount_e8s = 200_000_000 : nat64;
  dissolve_delay_seconds = 31_557_600 : nat32;
  memo = 12345 : nat64;
})"

echo ""
echo "Staking another 1 ICP with 6 month dissolve delay..."

# Stake another 1 ICP with minimum voting dissolve delay (6 months)
dfx canister call stake_neuron_backend stake_neuron "(record {
  amount_e8s = 100_000_000 : nat64;
  dissolve_delay_seconds = 15_778_800 : nat32;
  memo = 67890 : nat64;
})"

echo ""
echo "Checking final canister ICP balance:"
dfx canister call stake_neuron_backend get_canister_balance

echo ""
echo "Checking final ICP balance:"
dfx canister call icp_ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")')'})'

echo ""
echo "DEMO COMPLETE!"
echo "Successfully created 2 neurons:"
echo "1. 2 ICP neuron with 1 year dissolve delay"
echo "2. 1 ICP neuron with 6 month dissolve delay"
