# ICP Transfer

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/icp_transfer)

## Overview

ICP Transfer is a canister that can transfer ICP tokens from its account to other accounts. It demonstrates how to interact with the ICP ledger canister from a Motoko smart contract. The same example is also available in [Rust](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

The example implements a single `transfer` function that accepts a recipient principal, optional subaccount, and an amount in e8s (1 ICP = 100_000_000 e8s). The canister calls the ICP ledger at its well-known principal (`ryjl3-tyaaa-aaaaa-aaaba-cai`) — the same address used on both mainnet and the local development network.

> The ICP ledger also supports the ICRC-1 standard, which is the recommended standard for new token integrations. See the [token_transfer](https://github.com/dfinity/examples/tree/master/motoko/token_transfer) example for an ICRC-1 transfer example.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/icp_transfer
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

The `make test` target funds the backend canister with 2 ICP from the local network's default identity (which is seeded with tokens automatically), then calls `transfer` to send 1 ICP back to the default identity.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.

For this example, the following aspects are particularly relevant:

- [Inter-canister calls and rollbacks](https://docs.internetcomputer.org/guides/security/overview), since issues around inter-canister calls (here the ledger) can lead to time-of-check time-of-use or double spending security bugs.
- [Certify query responses if they are relevant for security](https://docs.internetcomputer.org/guides/security/overview), since this is essential when displaying important financial data in the frontend that may be used by users to decide on future transactions.
