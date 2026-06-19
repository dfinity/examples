# Receiving ICP

A canister demonstrating how to receive ICP tokens by generating account identifiers and checking balances on the ICP ledger.

The canister exposes methods to compute account identifiers (including subaccounts based on arbitrary 128-bit upper/lower values) and to query balances from the ledger canister. This makes it easy to give each user or purpose a distinct deposit address while keeping all ICP under one canister's control.

## Environment configuration

The ICP ledger canister ID is configured via `icp.yaml` and read at runtime as a canister environment variable via `ic_cdk::api::env_var_value("ICP_LEDGER_CANISTER_ID")`:

| Environment | Ledger | Canister ID |
|---|---|---|
| `local` | ICP ledger (pre-deployed by icp-cli) | `ryjl3-tyaaa-aaaaa-aaaba-cai` |
| `ic` | TESTICP ledger | `xafvr-biaaa-aaaai-aql5q-cai` |

The local environment uses the same canister ID as the mainnet ICP ledger because icp-cli's local network pre-deploys it at that well-known address. The `ic` environment uses the TESTICP ledger so you can test token flows on ICP mainnet without spending real ICP.

For a production deployment using the real ICP ledger, update `ICP_LEDGER_CANISTER_ID` in `icp.yaml` to `ryjl3-tyaaa-aaaaa-aaaba-cai` — see the comment in `icp.yaml` for the exact change.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/receiving-icp
```

### Deploy and test locally

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

`bash test.sh` runs 7 tests: account identifier format, subaccount uniqueness, funding the main account and a specific subaccount via account ID hex, balance queries, and subaccount independence. Tests are delta-based and idempotent across re-runs.

### Deploy to ICP mainnet

```bash
icp deploy -e ic
```

This targets the TESTICP ledger by default. To use the real ICP ledger, update `ICP_LEDGER_CANISTER_ID` in `icp.yaml` first.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
