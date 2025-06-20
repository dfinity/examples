# Low Wasm memory hook

The Internet Computer can automatically execute a special type of function called a low Wasm memory hook, which runs when the canister's available Wasm memory falls below the `wasm_memory_threshold`.

This Motoko example demonstrates using the low Wasm memory hook on ICP. For more information take a look at [low Wasm memory hook specification](https://internetcomputer.org/docs/references/ic-interface-spec#on-low-wasm-memory).

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/motoko/low_wasm_memory)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

After the deployment, the memory usage periodically increases as defined in the `heartbeat` function.

The `dfx canister update-settings` command sets the 'wasm_memory_limit' to 5MiB and 'wasm_memory_threshold' to 2MiB.
Hence, whenever the Wasm memory used by the canister is above 3MiB (in other words, the remaining Wasm memory is less than 'wasm_memory_threshold'), the low Wasm memory hook will be triggered.

This Motoko canister starts with ~2.3MiB of memory usage after the deployment, so it will trigger the low Wasm memory hook after allocating ~0.7MiB of memory.

You can verify that the canister settings got updated and check the current 'Memory Size' with the following command:

```sh
dfx canister status low_wasm_memory_hook
```

After a few seconds, observe the output of the `getExecutedFunctionsOrder` query:

Query the canister by calling `getExecutedFunctionsOrder` to get the order of executed functions.

```sh
dfx canister call low_wasm_memory_hook --query getExecutedFunctionsOrder
```

Repeat the call until the last executed method is `onLowWasmMemory`.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
