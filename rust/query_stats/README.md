# Query Stats

This example demonstrates how a canister can read its own query statistics using the management canister's `canister_status` endpoint. It retrieves metrics such as the total number of query calls, instructions executed, and payload bytes.

## How query stats work

Query stats are **aggregated with a 2-epoch delay**, not updated per call:

- Each epoch is **60 blocks** on local PocketIC (vs 600 on mainnet)
- Blocks advance every ~100ms with auto-progress enabled
- Stats for epoch N are only committed once 2/3+ of nodes have submitted records for epoch N+1
- Minimum wait: **2 epochs × 60 blocks × 100ms ≈ 12 seconds**; `make test-stats` polls up to 30 seconds to accommodate slower machines

Only **query calls** are tracked — calls made without `--query` go through consensus as update calls and are not counted in `query_stats.num_calls_total`.

Three things are required for stats to appear locally:

1. **Use `--query`** — `icp canister call` makes update calls by default; only query calls are tracked in `query_stats`
2. **Make 13+ calls per round** — PocketIC simulates a 13-node subnet and uses integer division (`num_calls / 13`); fewer than 13 calls round to zero
3. **Keep making queries continuously** — `set_epoch_from_height` is only invoked during query execution; queries must keep running across epoch boundaries to flush accumulated stats into the payload pipeline

`make test-stats` makes 13 calls every 3 seconds for up to 30 seconds.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/rust/query_stats
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

`make test-stats` calls `load()` 13 times with `--query` every 3 seconds (up to 30 seconds total), verifying non-zero stats once they appear.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
