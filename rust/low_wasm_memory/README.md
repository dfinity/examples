# Low Wasm memory hook

The Internet Computer can automatically execute a special type of function called low Wasm memory hook, which runs when the available Wasm memory of the canister falls below the 'wasm-memory-threshold'.

This example demonstrates the ways of using low Wasm memory hook on the Internet Computer.

The example consists of a canister named `low_wasm_memory_hook` implementing the functionality that it increases usage of Wasm memory in every 'heartbeat' execution, until the low Wasm memory hook is run.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Example 1: Low Wasm memory hook

- ### Step 1: Setup project environment

Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```sh
cd examples/rust/low_wasm_memory_hook
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or `dfx stop` command is run.

Example output:

```sh
dfx start --clean
[...]
Dashboard: http://localhost:63387/_/dashboard
```

- #### Step 2: Open another terminal window in the same directory:

```sh
cd examples/rust/low_wasm_memory_hook
```

- #### Step 3: Compile and deploy the `low_wasm_memory_hook` canister, setting the interval for periodic tasks to 10s:

```sh
dfx deploy low_wasm_memory_hook
```

Example output:

```sh
% dfx deploy low_wasm_memory_hook
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      heartbeat: http://127.0.0.1/...
      timer: http://127.0.0.1/...
```

- #### Setp 4: Update canister settings

Update canister settings:

```sh
dfx canister update-settings low_wasm_memory_hook --wasm-memory-limit 3000000 --wasm-memory-threshold 2000000
```

and test that setting are correctly updated:

```sh
dfx canister status low_wasm_memory_hook
```

Example output:

```sh
% dfx canister status low_wasm_memory_hook
Canister status call result for low_wasm_memory_hook.
Status: Running
Controllers: bnz7o-iuaaa-aaaaa-qaaaa-cai k7ujo-pl7jf-zqnnx-gdutf-uk5ck-4ngld-xq5hi-276ph-32z4y-ckaue-uae
Memory allocation: 0 Bytes
Compute allocation: 0 %
Freezing threshold: 2_592_000 Seconds
Idle cycles burned per day: 1_508_309 Cycles
Memory Size: 1_918_740 Bytes
Balance: 3_061_265_520_627 Cycles
Reserved: 0 Cycles
Reserved cycles limit: 5_000_000_000_000 Cycles
Wasm memory limit: 3_000_000 Bytes
Wasm memory threshold: 2_000_000 Bytes
Module hash: 0x715518791ccdc6b26cdc66e1c8405f2965208caf611ef4b698c612419dcbbf75
Number of queries: 0
Instructions spent in queries: 0
Total query request payload size: 0 Bytes
Total query response payload size: 0 Bytes
Log visibility: controllers
```


In the example above we set the 'wasm-memory-limit' to 3MB and 'wasm-memory-threshold' to 2MB. Hence whenever the Wasm memory
used by the canister is above 1MB (in other words, the remaining Wasm memory is less than 'wasm-memory-threshold') 
the low Wasm memory hook will run.



- #### Step 4: After 10s, observe similar non-zero counters in both canisters:

Query the canister calling 'get_executed_functions_order' to get the order of executed functions.

```sh
dfx canister call low_wasm_memory_hook --query get_executed_functions_order
```

Repeat the call until the last executed method is 'OnLowWasmMemory' hook.

Example output:

```sh
% dfx canister call low_wasm_memory_hook --query get_executed_functions_order
(
  vec { 
        variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { Heartbeat }; variant { OnLowWasmMemory };},
)
```

## Further learning

1. Have a look at the locally running dashboard. The URL is at the end of the `dfx start` command: `Dashboard: http://localhost/...`
2. Check out `low_wasm_memory_hook` canisters Candid user interface. The URLs are at the end of the `dfx deploy` command: `low_wasm_memory_hook: http://127.0.0.1/...`

### Canister interface

The `low_wasm_memory_hook` canister provide the following interface:

* `get_executed_functions_order` ; returns the vector with values of `FnType`(`enum` with variants `Heartbeat` and `OnLowWasmMemory` ) representing the order of functions executed.

Example usage:

```sh
dfx canister call low_wasm_memory_hook --query get_executed_functions_order
```

## Conclusion

For more information take a look at [low Wasm memory hook specification](https://internetcomputer.org/docs/references/ic-interface-spec#on-low-wasm-memory).

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
