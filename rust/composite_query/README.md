# Composite queries

On the Internet Computer, regular query functions are fast (no consensus) but have one strict limitation: **they cannot call other canisters**. Composite queries lift this restriction — a `#[query(composite = true)]` function can call query methods on other canisters while keeping the speed benefit of a query call.

For more background see [Composite queries](https://docs.internetcomputer.org/guides/canister-calls/parallel-inter-canister-calls/#composite-queries) in the ICP developer docs.

This example implements a distributed key-value store (`caller`) that shards its entries across five dynamically-installed `callee` child canisters. Looking up a key requires calling the appropriate callee:

- `get(k)` — **composite query**: delegates to the correct `callee.get(k)` as a cross-canister query call. Fast, no consensus.
- `get_update(k)` — **update call**: same lookup, but via an update call to the callee. Slower (goes through consensus) but provided here for comparison.

Both functions return the same result; the difference is latency and call semantics.

## Architecture

```
caller
  n = 5 callees      ┌── callee 0  (keys 0, 5, 10, …)
  key % n routes ────┼── callee 1  (keys 1, 6, 11, …)
                     ├── callee 2  (keys 2, 7, 12, …)
                     ├── callee 3  (keys 3, 8, 13, …)
                     └── callee 4  (keys 4, 9, 14, …)
```

`caller.put(k, v)` dynamically installs a `callee` canister if one does not exist for `k % 5`, then stores the entry there via an update call. `caller.get(k)` and `caller.get_update(k)` both route to the same callee via `k % 5`.

The `callee` WASM is embedded directly into the `caller` WASM binary at compile time via `include_bytes!`. This means only the `caller` canister needs to be deployed — it installs the `callee` canisters programmatically on first use.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/composite_query
```

### Deploy and test

```bash
icp network start -d
icp deploy --cycles 30t
bash test.sh
icp network stop
```

> `icp deploy --cycles 30t` is required because `caller` dynamically creates `callee` canisters — it needs extra cycles to fund their installation. If tests fail with an out-of-cycles error, run `icp canister top-up --amount 30t caller`.

Note that the first call to `put` is slow, since all five callee partitions are created at that point.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
