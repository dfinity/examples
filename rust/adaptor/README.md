# Neuron Staking from CLI

A Rust CLI application demonstrating to use [SNS Kongswap Adaptor](https://github.com/ShahriarJavidi/sns-kongswap-adaptor/tree/ICP-Ninja). Please note that this forked code is slightly modified to ease the interactions. Most drastically:
1. when initializing the adaptor, it doesn't expect any transfers/approvals
2. transfer flow for deposits has changed from `ICRC-2` to `ICRC-1`

## What You Can Learn

This is example teaches you
1. how to build a wrapper around the kongswap adaptor
2. how to interact with [SNS Kongswap Adaptor](https://github.com/ShahriarJavidi/sns-kongswap-adaptor/tree/ICP-Ninja).

## Local Testing with setup_and_run.sh

The `demo.sh` script inside the adaptor's repository provides a complete local testing environment:

```bash
#!/usr/bin/env bash
dfx stop
set -e
trap 'dfx stop' EXIT

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

dfx deploy sns_kongswap_adaptor --argument "
  (variant {
    Init = record {
      allowances = vec {
        record {
          asset = variant {
            Token = record {
              ledger_fee_decimals = 10_000 : nat;
              ledger_canister_id = principal \"$SNS_LEDGER_CANISTER\";
              symbol = \"LSNS\";
            }
          };
          amount_decimals = 0 : nat;
          owner_account = record {
            owner = principal \"$MINTER_PRINCIPAL\";
            subaccount = opt $TREASURY_SUBACCOUNT;
          };
        };
        record {
          asset = variant {
            Token = record {
              ledger_fee_decimals = 10_000 : nat;
              ledger_canister_id = principal \"$ICP_LEDGER_CANISTER\";
              symbol = \"ICP\";
            }
          };
          amount_decimals = 0 : nat;
          owner_account = record {
            owner = principal \"$MINTER_PRINCIPAL\";
            subaccount = null;
          };
        }
      };
    }
  })
"
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
```


## Running the Example

```bash
cd sns-kongswap-adaptor
./demo.sh
```

This will:
1. Start a local IC replica
2. Deploy an SNS and an ICP Ledger
3. Deploy the in-production wasm of Kongswap (as of 9th Septemeber 2025)
4. Deploy the adaptor
5. Deposit to the adaptor

### Prerequisites

1. **Install dfx**: Follow [DFINITY SDK installation](https://internetcomputer.org/docs/current/developer-docs/setup/install/)
2. **Rust toolchain**: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs/ | sh`



### Inspecting Results

After running locally, you can verify the balances:

```bash
(
  variant {
    Ok = record {
      timestamp_ns = 1_757_497_646_192_427_578 : nat64;
      asset_to_balances = opt vec {
        record {
          variant {
            Token = record {
              ledger_fee_decimals = 10_000 : nat;
              ledger_canister_id = principal "ryjl3-tyaaa-aaaaa-aaaba-cai";
              symbol = "ICP";
            }
          };
          record {
            treasury_owner = opt record {
              name = opt "DAO Treasury";
              amount_decimals = 0 : nat;
              account = opt record {
                owner = principal "2vxsx-fae";
                subaccount = null;
              };
            };
            suspense = opt record {
              name = null;
              amount_decimals = 0 : nat;
              account = null;
            };
            fee_collector = opt record {
              name = null;
              amount_decimals = 20_000 : nat;
              account = null;
            };
            treasury_manager = opt record {
              name = opt "KongSwapAdaptor(u6s2n-gx777-77774-qaaba-cai)";
              amount_decimals = 0 : nat;
              account = opt record {
                owner = principal "u6s2n-gx777-77774-qaaba-cai";
                subaccount = null;
              };
            };
            external_custodian = opt record {
              name = null;
              amount_decimals = 80_000 : nat;
              account = null;
            };
            payees = opt record {
              name = null;
              amount_decimals = 0 : nat;
              account = null;
            };
            payers = opt record {
              name = null;
              amount_decimals = 0 : nat;
              account = null;
            };
          };
        };
        record {
          variant {
            Token = record {
              ledger_fee_decimals = 10_000 : nat;
              ledger_canister_id = principal "lvfsa-2aaaa-aaaaq-aaeyq-cai";
              symbol = "LSNS";
            }
          };
          record {
            treasury_owner = opt record {
              name = opt "DAO Treasury";
              amount_decimals = 0 : nat;
              account = opt record {
                owner = principal "2vxsx-fae";
                subaccount = opt blob "\4d\a0\fd\dd\fd\fb\57\0a\e4\72\d5\e4\07\1c\f5\10\4d\a6\c0\be\71\ca\66\e9\b7\e1\db\6f\6e\ad\1d\c3";
              };
            };
            suspense = opt record {
              name = null;
              amount_decimals = 0 : nat;
              account = null;
            };
            fee_collector = opt record {
              name = null;
              amount_decimals = 20_000 : nat;
              account = null;
            };
            treasury_manager = opt record {
              name = opt "KongSwapAdaptor(u6s2n-gx777-77774-qaaba-cai)";
              amount_decimals = 0 : nat;
              account = opt record {
                owner = principal "u6s2n-gx777-77774-qaaba-cai";
                subaccount = null;
              };
            };
            external_custodian = opt record {
              name = null;
              amount_decimals = 80_000 : nat;
              account = null;
            };
            payees = opt record {
              name = null;
              amount_decimals = 0 : nat;
              account = null;
            };
            payers = opt record {
              name = null;
              amount_decimals = 0 : nat;
              account = null;
            };
          };
        };
      };
    }
  },
)
```

