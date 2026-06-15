# Composite queries

On the Internet Computer, regular query functions are fast (no consensus) but have one strict limitation: **they cannot call other canisters**. Composite queries lift this restriction — a `composite query func` can call query methods on other canisters while keeping the speed benefit of a query call.

This example implements a distributed key-value store (`Map`) that shards its entries across four dynamically-installed `Bucket` child canisters. Looking up a key requires calling the appropriate bucket:

- `get(k)` — **composite query**: delegates to the correct `Bucket.get(k)` as a cross-canister query call. Fast, no consensus.
- `getUpdate(k)` — **update call**: same lookup, but via an update call to the bucket. Slower (goes through consensus) but provided here for comparison.

Both functions return the same result; the difference is latency and call semantics.

## Architecture

```
Map (backend)          Bucket 0  (keys 0, 4, 8, …)
  n = 4 buckets  ───► Bucket 1  (keys 1, 5, 9, …)
  key % n routes      Bucket 2  (keys 2, 6, 10, …)
                       Bucket 3  (keys 3, 7, 11, …)
```

`Map.put(k, v)` dynamically installs a `Bucket` if one does not exist for `k % 4`, then stores the entry there. `Map.get(k)` and `Map.getUpdate(k)` both route to the same bucket via `k % 4`.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/composite_query
```

### Deploy and test

```bash
icp network start -d
icp deploy --cycles 30t
make test
icp network stop
```

> `icp deploy --cycles 30t` is required because `Map` dynamically creates `Bucket` canisters — it needs extra cycles to fund their installation. If tests fail with an out-of-cycles error, run `make topup`.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
