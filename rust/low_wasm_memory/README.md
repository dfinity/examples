# Low Wasm memory hook

The Internet Computer can automatically execute a special type of function called low Wasm memory hook, which runs when the available Wasm memory of the canister falls below the 'wasm_memory_threshold'.

The example consists of a canister named `low_wasm_memory_hook` that increases usage of Wasm memory in every 'heartbeat' execution, until the low Wasm memory hook is run.

For more information take a look at [low Wasm memory hook specification](https://internetcomputer.org/docs/references/ic-interface-spec#on-low-wasm-memory).

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/low_wasm_memory)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Run `dfx start --background --clean && dfx deploy` to deploy the project to your local environment. 

After the deployment, the memory usage periodically increases as defined in the `heartbeat` function.

The `dfx canister update-settings` command sets the 'wasm_memory_limit' to 5MiB and 'wasm_memory_threshold' to 3MiB.
Hence whenever the Wasm memory used by the canister is above 2MiB (in other words, the remaining Wasm memory is less than 'wasm_memory_threshold'), the low Wasm memory hook will be triggered.

This Rust canister starts with ~1.5MiB of memory usage after the deployment, so it will trigger the low Wasm memory hook after allocating ~0.5MiB of memory.

You can verify that the canister settings got updated and check the current 'Memory Size' with the following command:

```sh
dfx canister status low_wasm_memory_hook
```

After a few seconds, observe the output of the `get_executed_functions_order` query:

Query the canister by calling `get_executed_functions_order` to get the order of executed functions.

```sh
dfx canister call low_wasm_memory_hook --query get_executed_functions_order
```

Repeat the call until the last executed method is `OnLowWasmMemory`.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
