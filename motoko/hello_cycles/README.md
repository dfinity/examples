# Hello, cycles!

This example demonstrates how to send, receive, and check cycle balances from a Motoko canister. It shows the three key cycle management patterns: checking the current balance, accepting incoming cycles, and forwarding cycles to another canister.

The example exposes three public functions:

- `wallet_balance`: returns the current cycle balance of the canister as a `Nat`.
- `wallet_receive`: accepts up to 10 million cycles sent by the caller and returns how many were accepted.
- `transfer`: forwards a specified number of cycles to any shared function with the signature `() -> ()`.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/hello_cycles
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
