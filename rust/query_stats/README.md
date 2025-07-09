# Query statistics

This example shows to work with the query stats feature. It consists of a single canister called `query_stats`. It exports the following candid interface:

```candid
service : {
    "get_query_stats" : () -> (text);
    "load" : () -> (nat64) query;
};

```

The `load` function just returns a timestamp. It just exists such that there is a query endpoint to call. The method `get_query_stats` queries the status endpoint and returns the collected query statistics.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/query_stats)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
