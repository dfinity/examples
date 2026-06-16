# Receiving ICP

A canister demonstrating how to receive ICP tokens by generating account identifiers and checking balances on the ICP ledger.

The canister exposes methods to compute account identifiers (including subaccounts based on arbitrary 128-bit upper/lower values) and to query balances from the ledger canister. This makes it easy to give each user or purpose a distinct deposit address while keeping all ICP under one canister's control.

> **Note:** By default, the canister connects to a test ICP ledger (`xafvr-biaaa-aaaai-aql5q-cai`). To receive real ICP, update `LEDGER_PRINCIPAL` in `backend/lib.rs` to the mainnet ledger principal `ryjl3-tyaaa-aaaaa-aaaba-cai`.

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
icp network start -d
icp deploy
make test
icp network stop
```

## Security considerations and best practices

For information about security best practices for ICP canisters, see the [security overview](https://docs.internetcomputer.org/guides/security/overview).
