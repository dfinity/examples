---
keywords: [beginner, rust, query statistics]
---

# Query statistics

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/query_stats)

## Overview 

This example shows to work with the query stats feature.

## Architecture

The example consists of a single canister called `query_stats`.
It exports the following candid interface:

```candid
service : {
    "get_query_stats" : () -> (text);
    "load" : () -> (nat64) query;
};

```

The `load` function just returns a timestamp.
It just exists such that there is a query endpoint to call.
The `get_query_stats` is the function that queries the status endpoint and returns the collected query statistics.

### Prerequisites 
This example requires an installation of:
- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Install `node.js` (to build the web frontend).


 ### Step 1: Start a local canister execution environment:

```
dfx start --background
```

 ### Step 2: Register, build, and deploy the project with the command:

```
dfx deploy
```

 ### Step 3: Call the canisters load function a few times to generate query traffic:

```
dfx canister call query_stats load
```

 ### Step 4: Observe the following result:

```
dfx canister call query_stats get_query_stats
```

 ### Step 5: After a while, the values should become populated

```
"Number of calls: 19 - Number of instructions 414_083 - Request payload bytes: 114 - Response payload bytes: 270"
```

Alternatively, you can use the candid interface to make those calls.

### Troubleshooting

On the local `dfx` replica, the aggregation epoch is set to 60 seconds.
So calling the load function a couple of times should result in values showing up a couple minutes later.

On mainnet, the aggregation epoch is 10 minutes, thus it will up to half an hour before the values appear on mainnet.

### Possible next steps

Query statistics are simple counters that increase.
Their raw values are not that useful, you may want to implement some sort of metering system.

One way to go from here is to get the query stats in a regular interval using a timer and compare to the last values, calculating rates.

### Resources
- [ic-cdk](https://docs.rs/ic-cdk/latest/ic_cdk/).

## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
