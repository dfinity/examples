# Motoko Canister Factory

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/canister_factory)

## Overview

This example demonstrates two approaches to creating canisters dynamically on the Internet Computer: high-level actor class management (using the `system` keyword) and low-level management canister calls. It also shows the critical difference between upgrading a canister (state preserved) and reinstalling it (state reset).

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org)
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/canister_factory
```

### Deploy and test

```bash
icp network start -d
icp deploy --cycles 30t
bash test.sh
icp network stop
```

The `--cycles 30t` flag funds the backend canister with 30 trillion cycles so it can forward cycles when creating child canisters.

`bash test.sh` exercises all five functions and verifies the core educational point: after `addToValue(10)` sets a child canister's value to 52, upgrading preserves it at 52 while reinstalling resets it to 42.

> If the tests fail with an out-of-cycles error, run `icp canister top-up --amount 30t backend` to add 30 trillion cycles to the backend canister and retry.

## Available functions

### Actor class management (high-level)

Uses the Motoko `system` keyword with actor classes. Simpler API, but limited to four canister settings: `controllers`, `compute_allocation`, `memory_allocation`, `freezing_threshold`.

#### `newActorClass(cycles: nat)` — create and install in one step

```bash
icp canister call backend newActorClass '(2_000_000_000_000)'
```

#### `installActorClass(cycles: nat)` — two-step: create then install

```bash
icp canister call backend installActorClass '(2_000_000_000_000)'
```

#### `upgradeActorClass(canisterId: principal)` — upgrade to CounterV2, state preserved

```bash
icp canister call backend upgradeActorClass '(principal "<canister-id>")'
```

#### `reinstallActorClass(canisterId: principal)` — reinstall with CounterV2, state reset

```bash
icp canister call backend reinstallActorClass '(principal "<canister-id>")'
```

### Management canister (low-level)

Calls `aaaaa-aa` directly via `mo:ic`. Gives full access to all canister settings including `reserved_cycles_limit`, `wasm_memory_limit`, `log_visibility`, and `wasm_memory_threshold`, which are not available through actor class management.

#### `createAndInstallCanisterManually(cycles: nat)` — create and install empty WASM

```bash
icp canister call backend createAndInstallCanisterManually '(2_000_000_000_000)'
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on protecting your application.
