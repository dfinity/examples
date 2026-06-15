# Parallel inter-canister calls

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/parallel_calls)

## Overview

This example demonstrates how to implement inter-canister calls that run in parallel in Motoko, and highlights some differences between parallel and sequential calls. Running independent calls in parallel can lower latency, especially when messages are sent across subnets. For example, a canister that swaps two tokens might want to launch both token transfer operations in parallel.

The example consists of two canisters, `backend` (caller) and `callee`. The `backend` canister has three endpoints:
1. `setup_callee` — sets the ID of the callee canister.
2. `sequential_calls` — takes a number `n` and issues `n` calls to the callee sequentially, returning the number of successful calls.
3. `parallel_calls` — takes a number `n` and issues `n` calls to the callee in parallel, returning the number of successful calls.

The `callee` canister exposes a simple `ping` endpoint that takes no parameters and returns nothing.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/parallel_calls
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

When running with a small number of calls (e.g. 100), sequential and parallel calls both succeed. With a large number of calls (e.g. 2000), sequential calls all succeed but most parallel calls fail because the replica imposes a limit on the number of in-flight calls a canister can make. Parallel calls are most useful in multi-subnet settings, where they significantly reduce latency.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
