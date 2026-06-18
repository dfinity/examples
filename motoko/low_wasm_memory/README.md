# Low Wasm memory hook

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
bash test.sh
icp network stop
```

`bash test.sh` does two things:

1. **Configures the canister settings**: sets `wasm_memory_limit` to 5 MiB and `wasm_memory_threshold` to 2 MiB. This means the `lowmemory` hook fires when remaining Wasm memory falls below 2 MiB (i.e. when usage exceeds 3 MiB). The canister starts with ~2.3 MiB of usage after deployment, so the hook triggers after allocating roughly 0.7 MiB more.

2. **Polls `getExecutedFunctionsOrder`** until `onLowWasmMemory` appears as the last entry (or times out after 60 s).

To observe the execution order manually:

```bash
icp canister call --query backend getExecutedFunctionsOrder '()'
```

Repeat until the last entry is `onLowWasmMemory`.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
