# Low Wasm memory hook

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/low_wasm_memory)

## Overview

This example demonstrates the low Wasm memory hook on the Internet Computer. The ICP runtime automatically executes a special `lowmemory` system function when a canister's available Wasm memory falls below the configured `wasm_memory_threshold`. The example shows the execution order of the `heartbeat` and `lowmemory` system functions as the canister progressively allocates memory until the threshold is reached.

For more information, see the [low Wasm memory hook specification](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#on-low-wasm-memory).

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/low_wasm_memory
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

After deployment, the canister's `heartbeat` function periodically allocates memory. Once the remaining Wasm memory falls below the configured threshold, the `lowmemory` hook fires. Query `getExecutedFunctionsOrder` to observe the execution order:

```bash
icp canister call backend getExecutedFunctionsOrder '()'
```

Repeat the call until the last entry is `onLowWasmMemory`.

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
