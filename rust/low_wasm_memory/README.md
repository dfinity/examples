# Low Wasm memory hook

This example demonstrates the low Wasm memory hook on the Internet Computer. The ICP runtime automatically executes a special `on_low_wasm_memory` system function when a canister's available Wasm memory falls below the configured `wasm_memory_threshold`. The example shows the execution order of the `heartbeat` and `on_low_wasm_memory` system functions as the canister progressively allocates memory until the threshold is reached.

If you're interested in how this example is implemented in Motoko, check out the [Motoko variation](../../motoko/low_wasm_memory).

For more information, see the [low Wasm memory hook specification](https://docs.internetcomputer.org/references/ic-interface-spec/canister-interface/#on-low-wasm-memory).

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/low_wasm_memory
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

`icp.yaml` sets `wasm_memory_limit` to 8 MiB and `wasm_memory_threshold` to 2 MiB. The `on_low_wasm_memory` hook fires when usage exceeds 8 − 2 = 6 MiB.

`bash test.sh` polls `get_executed_functions_order` until `OnLowWasmMemory` appears as the last entry (or times out after 60 s).

To observe the execution order manually:

```bash
icp canister call --query backend get_executed_functions_order '()'
```

Repeat until the last entry is `OnLowWasmMemory`.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
