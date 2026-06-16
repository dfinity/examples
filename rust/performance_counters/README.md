# Performance counters

A canister can query one of the "performance counters", which is a deterministic monotonically increasing integer approximating the amount of work the canister has done. Developers can use this data to profile and optimize canister performance.

```Candid
ic0.performance_counter : (counter_type : i32) -> i64
```

The argument `counter_type` decides which performance counter to return:

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

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/performance_counters
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

### Calling the canister

Call `backend` canister `for_update` method:

```sh
icp canister call backend for_update '()'
```

Example output:

```sh
(6_618_678 : nat64, 19_886_107 : nat64)
```

Note how the current message execution counter (~6M instructions) is much different from the call context counter (~19M instructions).

The canister also prints detailed logs. Example log output for `for_update`:

```text
Performance counters for update call:    current (0)     call context (1)
  before the nested call:                6614001         6614201        
  > inside the 1st nested call:          12425           12625          
  after the 1st nested call:             6618836         13250387       
  > inside the 2nd nested call:          12516           12716          
  after the 2nd nested call:             6618678         19886107       
```

Note how the current execution instruction counter (0) stays at ~6M instructions after each await point.
By contrast, the call context performance counter (1) is monotonically increasing (~6M, ~13M, ~19M instructions).

Also note that both counters start over for each nested execution (~12K instructions).

Call `for_composite_query` method:

```sh
icp canister call --query backend for_composite_query '()'
```

Example output:

```sh
(6_621_477 : nat64, 19_893_467 : nat64)
```

Example log output:

```text
Perf. counters for composite query call: current (0)     call context (1)
  before the nested call:                6614001         6614201        
  > inside the 1st nested call:          13567           13767          
  after the 1st nested call:             6623158         13254766       
  > inside the 2nd nested call:          13567           13767          
  after the 2nd nested call:             6621477         19893467 
```

Note the same performance counters behavior for composite queries.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
