# Receiving ICP

A canister demonstrating how to receive ICP tokens by generating account identifiers and checking balances on the ICP ledger.

The canister exposes methods to compute account identifiers (including subaccounts based on arbitrary 128-bit upper/lower values) and to query balances from the ledger canister. This makes it easy to give each user or purpose a distinct deposit address while keeping all ICP under one canister's control.

## Environment configuration

The ICP ledger canister ID is configured via `icp.yaml` and read at runtime via `ic_cdk::api::env_var_value("ICP_LEDGER_CANISTER_ID")`:

| Environment | Ledger | Canister ID |
|---|---|---|
| `local` | ICP ledger (pre-deployed by icp-cli) | `ryjl3-tyaaa-aaaaa-aaaba-cai` |
| `staging` | TESTICP ledger | `xafvr-biaaa-aaaai-aql5q-cai` |
| `production` | ICP ledger (mainnet) | `ryjl3-tyaaa-aaaaa-aaaba-cai` |

The local environment uses the same principal as production because icp-cli's local network pre-deploys the ICP ledger at that well-known address. Staging uses the TESTICP ledger so you can test token flows without spending real ICP.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/receiving-icp
```

### Deploy and test

```bash
# Local (default)
icp network start -d
icp deploy
bash test.sh
icp network stop

# Staging (targets TESTICP ledger on mainnet)
icp deploy --environment staging

# Production
icp deploy --environment production
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
