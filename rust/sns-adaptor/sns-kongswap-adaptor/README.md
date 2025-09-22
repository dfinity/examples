# SNS Kongswap Adaptor

[SNS Kongswap Adaptor](https://github.com/ShahriarJavidi/sns-kongswap-adaptor/tree/ICP-Ninja) is a Rust-based canister designed to act as an adaptor between the Service Nervous System (SNS) treasury and the KongSwap decentralized exchange on the Internet Computer. Its primary function is to facilitate and automate the management of token assets (such as SNS and ICP tokens) held by a DAO treasury, enabling operations like deposits, withdrawals, balance tracking, and token swaps through KongSwap. The adaptor interacts with multiple canisters, including ledger canisters for different tokens and the KongSwap backend, to execute and audit these operations securely and transparently.

The codebase is structured to ensure robust state management, transaction auditing, and error handling. It provides mechanisms to refresh ledger metadata, manage asset balances, and emit transactions with detailed logging and access control. 

Please note that this forked code is slightly modified to ease the interactions. Most drastically:

1. when initializing the adaptor, it doesn't expect any transfers/approvals
2. transfer flow for deposits has changed from `ICRC-2` to `ICRC-1`

## What You Can Learn

This is example teaches you
1. how to build a wrapper around the kongswap adaptor
2. how to interact with [SNS Kongswap Adaptor](https://github.com/ShahriarJavidi/sns-kongswap-adaptor/tree/ICP-Ninja).

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Run" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/ShahriarJavidi/sns-kongswap-adaptor/tree/ICP-Ninja)

## Local Testing with demo.sh

The `demo.sh` script inside the adaptor's repository provides a complete local testing environment:

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

This output shows the result of a balance query for two tokens, "ICP" and "LSNS", displaying how each token is distributed among various roles in the treasury system. For both tokens, the balances are split into categories such as `treasury_owner`, `suspense`, `fee_collector`, `treasury_manager`, `external_custodian`, payees, and payers. Most balances are 0, except for `fee_collector` (20_000) and `external_custodian` (80_000).

The values reflect that, when transferring funds to the DEX, two actions each incur a fee of 10_000: 
first, giving approval to the DEX, and second, the transfer initiated by the DEX. These fees are accouned in the `fee_collector` (totaling 20_000 per token). The `external_custodian` balance (80_000) is the amount that actually reaches the DEX after fees are deducted. Thus, the output shows the net result of a typical DEX transfer flow, with fees accounted for and the final amount available to the DEX shown under `external_custodian`.