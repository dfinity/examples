# Query Stats

This example demonstrates how a canister can read its own query statistics using `ic.canister_status`. It retrieves metrics such as the total number of query calls, instructions executed, and payload bytes.

## How query stats work

Query stats are **not updated in real-time**. The IC aggregates them with a minimum 2-epoch delay:

- Each epoch is **60 blocks** on local PocketIC (vs 600 on mainnet)
- Stats for epoch N are only committed once 2/3+ of nodes have submitted records for epoch N+1
- This means **at least 120 blocks** must pass after a query call before its contribution appears in `canister_status().query_stats`

**On a local replica**, stats will show `0` for all fields even after making many query calls — this is expected behavior, not a bug. Non-zero values are observable on IC mainnet after enough queries accumulate across multiple epochs.

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
make test
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
