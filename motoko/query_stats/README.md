# Query Stats

This example demonstrates how a canister can read its own query statistics using `ic.canister_status`. It retrieves metrics such as the total number of query calls, instructions executed, and payload bytes.

## How query stats work

Query stats are **aggregated with a 2-epoch delay**, not updated per-call:

- Each epoch is **60 blocks** on local PocketIC (vs 600 on mainnet)
- Stats for epoch N are only committed once 2/3+ of nodes have submitted records for epoch N+1
- At least **2 epochs** (120+ blocks) must pass after a query call before it appears in `canister_status().query_stats`

**On a local replica**, the first `make test` run will show `0` for all fields because the stats from that run haven't aggregated yet. Run `make test` a second time (with the same network still running) and you'll see non-zero values from the first run.

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
```bash
icp network start -d
icp deploy
make test   # first run: stats show 0 (aggregation lag)
make test   # second run: stats show non-zero values from the first run
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
