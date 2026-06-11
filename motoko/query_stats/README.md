# Query Stats

This example demonstrates how a canister can read its own query statistics using `ic.canister_status`. It retrieves metrics such as the total number of query calls, instructions executed, and payload bytes.

## How query stats work

Query stats are **aggregated with a 2-epoch delay**, not updated per call:

- Each epoch is **60 blocks** on local PocketIC (vs 600 on mainnet)
- Blocks advance every ~100ms with auto-progress enabled
- Stats for epoch N are only committed once 2/3+ of nodes have submitted records for epoch N+1
- Minimum wait: **2 epochs × 60 blocks × 100ms ≈ 15–20 seconds** after the query calls

Only **query calls** are tracked — calls made without `--query` go through consensus as update calls and are not counted in `query_stats.num_calls_total`.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/query_stats
```

### Deploy and test

**Fast test** (verifies API shape; stats show 0 due to aggregation delay):
```bash
icp network start -d
icp deploy
make test
icp network stop
```

**Full demonstration** (generates load, waits ~20s, verifies non-zero stats):
```bash
icp network start -d
icp deploy
make test-stats
icp network stop
```

`make test-stats` calls `load()` 20 times with `--query`, waits 20 seconds for the aggregation window, then verifies that `Number of calls` is non-zero.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
