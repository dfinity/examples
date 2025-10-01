#!/usr/bin/env bash
dfx stop
set -e
trap 'dfx stop' EXIT

echo "Deploying ICP Ledger canister..."
dfx start --background --clean

export MINTER_ACCOUNT_ID=$(dfx --identity anonymous ledger account-id)
export DEFAULT_ACCOUNT_ID=$(dfx ledger account-id)

dfx deploy icp_ledger_canister --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 10_000_000_000 : nat64;
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
dfx canister call icp_ledger_canister account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")')'})'

echo "Deploying SNS Ledger canister..."
dfx deploy sns_ledger_canister --argument "
  (variant {
    Init = record {
      minting_account = \"$MINTER_ACCOUNT_ID\";
      initial_values = vec {
        record {
          \"$DEFAULT_ACCOUNT_ID\";
          record {
            e8s = 10_000_000_000 : nat64;
          };
        };
      };
      send_whitelist = vec {};
      transfer_fee = opt record {
        e8s = 10_000 : nat64;
      };
      token_symbol = opt \"LSNS\";
      token_name = opt \"Local SNS\";
    }
  })
"
dfx canister call sns_ledger_canister account_balance '(record { account = '$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$DEFAULT_ACCOUNT_ID'")]) + "}")')'})'

echo "Deploying Kong Backend canister..."

dfx deploy kong_backend

export ICP_LEDGER_CANISTER=$(dfx canister id icp_ledger_canister)
export SNS_LEDGER_CANISTER=$(dfx canister id sns_ledger_canister)
export MINTER_PRINCIPAL=$(dfx identity --identity anonymous get-principal)

# We calculate the expected subaccount for the treasury by
# using the function "utils::compute_treasury_subaccount_bytes"
# in kongswap_adaptor/tests/common/utils.rs.
# As principal "lvfsa-2aaaa-aaaaq-aaeyq-cai" is hardcoded in the function,
# we can just run the test to get the expected subaccount bytes.
export TREASURY_SUBACCOUNT="vec{77;160;253;221;253;251;87;10;228;114;213;228;7;28;245;16;77;166;192;190;113;202;102;233;183;225;219;111;110;173;29;195}";

echo "Deploying SNS Kongswap Adaptor canister..."
dfx deploy sns_kongswap_adaptor

TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister sns_kongswap_adaptor)"
TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')"
dfx canister call icp_ledger_canister transfer "(record { to=${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; amount=record { e8s=100_000 }; fee=record { e8s=10_000 }; memo=0:nat64; }, )"
dfx canister call sns_ledger_canister transfer "(record { to=${TOKENS_TRANSFER_ACCOUNT_ID_BYTES}; amount=record { e8s=100_000 }; fee=record { e8s=10_000 }; memo=0:nat64; }, )"

echo "Balances of the Kongswap Adaptor canister:"
dfx canister call icp_ledger_canister account_balance '(record { account = '$TOKENS_TRANSFER_ACCOUNT_ID_BYTES'})'
dfx canister call sns_ledger_canister account_balance '(record { account = '$TOKENS_TRANSFER_ACCOUNT_ID_BYTES'})'

dfx canister call sns_kongswap_adaptor deposit \
'(record {
  allowances = vec {
    record {
      asset = variant { Token = record {
        ledger_fee_decimals = 10000 : nat;
        ledger_canister_id = principal "'$SNS_LEDGER_CANISTER'";
        symbol = "LSNS";
      }}; 
      amount_decimals = 100000 : nat;
      owner_account = record {
        owner = principal "'$MINTER_PRINCIPAL'";
        subaccount = opt '"$TREASURY_SUBACCOUNT"';
      };
    };
    record {
      asset = variant { Token = record {
        ledger_fee_decimals = 10000 : nat;
        ledger_canister_id = principal "'$ICP_LEDGER_CANISTER'";
        symbol = "ICP";
      }}; 
      amount_decimals = 100000 : nat;
      owner_account = record {
        owner = principal "'$MINTER_PRINCIPAL'";
        subaccount = null;
      };
    };
  };
})'

echo "DONE"