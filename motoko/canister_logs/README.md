# Canister logs

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/canister_logs)

## Overview

This example demonstrates canister logging and error handling on the Internet Computer. It shows how `Debug.print` messages and trap messages are recorded in the canister log, covering update calls, replicated query calls, timer-triggered traps, memory out-of-bounds errors, and management canister calls like `raw_rand`.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/canister_logs
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
