# Composite queries

On the Internet Computer, regular query functions are fast (no consensus) but have one strict limitation: **they cannot call other canisters**. Composite queries lift this restriction — a `#[query(composite = true)]` function can call query methods on other canisters while keeping the speed benefit of a query call.

For more background see [Composite queries](https://docs.internetcomputer.org/guides/canister-calls/parallel-inter-canister-calls/#composite-queries) in the ICP developer docs.

This example implements a distributed key-value store (`backend`) that shards its entries across five dynamically-installed `Bucket` child canisters. Looking up a key requires calling the appropriate bucket:

- `get(k)` — **composite query**: delegates to the correct `Bucket.get(k)` as a cross-canister query call. Fast, no consensus.
- `get_update(k)` — **update call**: same lookup, but via an update call to the bucket. Slower (goes through consensus) but provided here for comparison.

Both functions return the same result; the difference is latency and call semantics.

## Architecture

```
backend (Map)
  n = 5 buckets      ┌── Bucket 0  (keys 0, 5, 10, …)
  key % n routes ────┼── Bucket 1  (keys 1, 6, 11, …)
                     ├── Bucket 2  (keys 2, 7, 12, …)
                     ├── Bucket 3  (keys 3, 8, 13, …)
                     └── Bucket 4  (keys 4, 9, 14, …)
```

`backend.put(k, v)` creates all five `Bucket` canisters on the first call, then stores the entry in the one responsible for `k % 5`. `backend.get(k)` and `backend.get_update(k)` both route to the same bucket via `k % 5`.

The `Bucket` WASM is embedded directly into the `backend` WASM binary at compile time via `include_bytes!`. Only the `backend` canister is deployed — it installs `Bucket` canisters programmatically on the first `put` call. The backend divides its available cycle balance equally among the buckets and itself, mirroring the Motoko approach.

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

> `icp deploy --cycles 30t` is required because `backend` dynamically creates `Bucket` canisters — it needs extra cycles to fund their installation. If tests fail with an out-of-cycles error, run `icp canister top-up --amount 30t backend`.

Note that the first call to `put` is slow, since all five `Bucket` partitions are created at that point. `bash test.sh` can be re-run on the same deployment — tests 2–7 overwrite the same keys with the same values and are idempotent.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
