# Low Wasm memory hook

The Internet Computer can automatically execute a special type of function called a low Wasm memory hook, which runs when the canister's available Wasm memory falls below the `wasm_memory_threshold`.

This Motoko example demonstrates using the low Wasm memory hook on ICP. If you're interested in how this example is implemented in Rust, check out the [Rust variation](../../rust/low_wasm_memory).

The example consists of a canister named `low_wasm_memory_hook` that increases usage of Wasm memory in every heartbeat execution until the low Wasm memory hook is run.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Using the example

In this example, the canister will periodically increase its memory usage (as defined in the `heartbeat` function) until the low Wasm memory hook is run
when the memory usage exceeds the `wasm_memory_threshold`.

- #### Step 1: Setup project environment

Navigate into the folder containing the project's files and start a local developer environment with the commands:

```sh
cd examples/motoko/low_wasm_memory
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or the `dfx stop` command is run.

- #### Step 2: Open another terminal window in the same directory:

```sh
cd examples/motoko/low_wasm_memory
```

- #### Step 3: Create the `low_wasm_memory_hook` canister

```sh
dfx canister create low_wasm_memory_hook
```

Example output:

```sh
Created a wallet canister on the "local" network for user "default" with ID "uqqxf-5h777-77774-qaaaa-cai"
low_wasm_memory_hook canister created with canister id: uxrrr-q7777-77774-qaaaq-cai
```

- #### Step 4: Update canister settings

Update the canister's settings to modify the Wasm memory limit and threshold:

```sh
dfx canister update-settings low_wasm_memory_hook --wasm-memory-limit 5000000 --wasm-memory-threshold 2000000
```

- #### Step 5: Compile and deploy the `low_wasm_memory_hook` canister:

```sh
dfx deploy low_wasm_memory_hook
```

Example output:

```sh
Deploying: low_wasm_memory_hook
All canisters have already been created.
Building canister 'low_wasm_memory_hook'.
Installed code for canister low_wasm_memory_hook, with canister ID uxrrr-q7777-77774-qaaaq-cai
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    low_wasm_memory_hook: http://127.0.0.1:4943/?canisterId=u6s2n-gx777-77774-qaaba-cai&id=uxrrr-q7777-77774-qaaaq-cai
```

After the deployment, the memory usage periodically increases as defined in the `heartbeat` function.

The `dfx canister update-settings` command sets the 'wasm_memory_limit' to 5MiB and 'wasm_memory_threshold' to 2MiB.
Hence, whenever the Wasm memory used by the canister is above 3MiB (in other words, the remaining Wasm memory is less than 'wasm_memory_threshold'), the low Wasm memory hook will be triggered.

You can verify that the canister settings got updated and check the current 'Memory Size' with the following command:

```sh
dfx canister status low_wasm_memory_hook
```

Example output:

```sh
Canister status call result for low_wasm_memory_hook.
Status: Running
Controllers: 3apfx-fn75o-xtwmf-svjzg-hu5xt-bg2dr-ubczq-uhvlq-2a5gf-ya4fn-dqe uqqxf-5h777-77774-qaaaa-cai
Memory allocation: 0 Bytes
Compute allocation: 0 %
Freezing threshold: 2_592_000 Seconds
Idle cycles burned per day: 24_269_759 Cycles
Memory Size: 2_374_914 Bytes
Balance: 2_996_231_412_893 Cycles
Reserved: 0 Cycles
Reserved cycles limit: 5_000_000_000_000 Cycles
Wasm memory limit: 5_000_000 Bytes
Wasm memory threshold: 2_000_000 Bytes
Module hash: 0x0c100162e1be161b9ef6fd95efb07f0541903c77a47e85a49a800705d6a09a11
Number of queries: 0
Instructions spent in queries: 0
Total query request payload size: 0 Bytes
Total query response payload size: 0 Bytes
Log visibility: controllers
```

- #### Step 6: After a few seconds, observe the output of the `getExecutedFunctionsOrder` query:

Query the canister by calling `getExecutedFunctionsOrder` to get the order of executed functions.

```sh
dfx canister call low_wasm_memory_hook --query getExecutedFunctionsOrder
```

Repeat the call until the last executed method is `onLowWasmMemory`.

Example output:

```sh
(
  vec { variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { heartbeat }; variant { onLowWasmMemory };},
)
```

To repeat the example, you can redeploy the canister and reset its state with the following command:

```sh
dfx deploy low_wasm_memory_hook --mode reinstall
```

## Further learning
1. Have a look at the locally running dashboard. The URL is at the end of the `dfx start` command: `Dashboard: http://localhost:...`
2. Check out `low_wasm_memory_hook` canister's Candid user interface. The URLs are at the end of the `dfx deploy` command: `low_wasm_memory_hook: http://127.0.0.1:...`

### Canister interface

The `low_wasm_memory_hook` canister provides the following interface:
* `getExecutedFunctionsOrder` ; returns the vector with values of `FnType` (a variant with `#heartbeat` or `#onLowWasmMemory`) representing the order of functions executed.

Example usage:
```sh
dfx canister call low_wasm_memory_hook --query getExecutedFunctionsOrder
```

## Resources
For more information take a look at [low Wasm memory hook specification](https://internetcomputer.org/docs/references/ic-interface-spec#on-low-wasm-memory).

## Security considerations and best practices
If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.