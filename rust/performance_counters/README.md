# Performance counter

A canister can query one of the "performance counters", which is a deterministic monotonically increasing integer approximating the amount of work the canister has done. Developers might use this data to profile and optimize the canister performance.

```Candid
ic0.performance_counter : (counter_type : i32) -> i64
```

The argument `type` decides which performance counter to return:

- 0 : current execution instruction counter.
      The number of WebAssembly instructions the canister has executed
      since the beginning of the current Message execution.

- 1 : call context instruction counter.

  - For replicated message execution, it is the number of WebAssembly instructions
    the canister has executed within the call context of the current Message execution
    since Call context creation. The counter monotonically increases across all message
    executions in the call context until the corresponding call context is removed.

  - For non-replicated message execution, it is the number of WebAssembly instructions
    the canister has executed within the corresponding `composite_query_helper`
    in Query call. The counter monotonically increases across the executions
    of the composite query method and the composite query callbacks
    until the corresponding `composite_query_helper` returns
    (ignoring WebAssembly instructions executed within any further downstream calls
    of `composite_query_helper`).

In the future, ICP might expose more performance counters.

## Prerequisites

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install). For local testing, `dfx >= 0.22.0` is required.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Step 1: Setup project environment

Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```sh
cd examples/rust/performance_counters
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or `dfx stop` command is run.

Example output:

```sh
dfx start --clean
[...]
Dashboard: http://localhost:63387/_/dashboard
```

## Step 2: Open another terminal window in the same directory

```sh
cd examples/rust/performance_counters
```

## Step 3: Compile and deploy `performance_counters` canister

```sh
dfx deploy
```

Example output:

```sh
% dfx deploy
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      performance_counters: http://127.0.0.1/...
```

## Step 4: Call `performance_counters` canister `for_update` method

```sh
dfx canister call performance_counters for_update
```

Example output:

```sh
% dfx canister call performance_counters for_update
(6_618_678 : nat64, 19_886_107 : nat64)
```

Note, how the current message execution counter (~6M instructions) is much different from the call context counter (~19M instructions).

## Step 5: Check the replica terminal window for more details

Example replica log output:

```text
Performance counters for update call:    current (0)     call context (1)
  before the nested call:                6614001         6614201        
  > inside the 1st nested call:          12425           12625          
  after the 1st nested call:             6618836         13250387       
  > inside the 2nd nested call:          12516           12716          
  after the 2nd nested call:             6618678         19886107       
```

Note, how the current execution instruction counter (0) stays at ~6M instructions after each await point.
By contrast, the call context performance counter (1) is monotonically increasing (~6M, ~13M, ~19M instructions).

Also note, that both counters start over for each nested execution (~12K instructions).

## Step 6: Repeat the steps above calling `for_composite_query` method

```sh
dfx canister call performance_counters for_composite_query
```

Example output:

```sh
% dfx canister call performance_counters for_update
(6_621_477 : nat64, 19_893_467 : nat64)
```

Example replica log output:

```text
Perf. counters for composite query call: current (0)     call context (1)
  before the nested call:                6614001         6614201        
  > inside the 1st nested call:          13567           13767          
  after the 1st nested call:             6623158         13254766       
  > inside the 2nd nested call:          13567           13767          
  after the 2nd nested call:             6621477         19893467 
```

Note the same performance counters behavior for composite queries.

## Further learning

1. Have a look at the locally running dashboard. The URL is at the end of the `dfx start` command: `Dashboard: http://localhost/...`
2. Check out the Candid user interface for `performance_counters` canister. The URL is at the end of the `dfx deploy` command: `performance_counters: http://127.0.0.1/...`

### Canister interface

The `performance_counters` canisters provide the following interface:

- `for_update` &mdash; return all the performance counters values after two nested update calls.
- `for_composite_query` &mdash; return all the performance counters values after two nested composite query calls.

## Conclusion

Performance counters is a great tool to optimize canister performance, both for update calls and queries.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
