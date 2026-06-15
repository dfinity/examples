# Composite queries

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/composite_query)

## Overview

This example demonstrates composite queries in Motoko. It implements a distributed key-value store (`Map`) that dynamically installs `Bucket` actor class instances to hold its entries. The `Map.get` function is a composite query that calls the query function `Bucket.get` on a child canister — something only possible with composite queries. This shows how composite queries enable fast, read-only cross-canister calls without the latency of update calls.

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

> If tests fail with an out-of-cycles error, run `make topup` to add 30 trillion cycles to the backend canister and retry.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
