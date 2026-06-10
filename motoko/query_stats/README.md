# Query Stats

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/query_stats)

## Overview

This example demonstrates how a canister can read its own query statistics from the management canister. It uses `ic.canister_status` to retrieve metrics such as the total number of calls, instructions executed, and payload bytes for the canister's query methods.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/query_stats
```

### Deploy and test
```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP dapp.
