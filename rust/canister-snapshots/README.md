# Canister snapshots

This example demonstrates the process of taking and loading canister snapshots.
It features a canister named `chat`, which functions as a miniature database.
The `remove_spam` canister method intentionally includes a bug to simulate data loss.
The example outlines the steps to install the canister, create a snapshot,
and subsequently restore the data after the simulated data loss.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/canister-snapshots)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Run `dfx start --background --clean && dfx deploy` to deploy the project to your local environment. 

### 5. Populate the database with data.

```sh
dfx canister call chat append 'Hi there!'
dfx canister call chat dump
```

### 6. Take canister snapshot.

Canister `chat` is currently running, and snapshots should not be taken of active canisters.
Therefore, the canister should be stopped before taking a snapshot and then restarted.

```sh
dfx canister stop chat
dfx canister snapshot create chat
dfx canister start chat
```

### 7. Simulate data loss.

The `remove_spam` canister method intentionally includes a bug to simulate data loss.

```sh
dfx canister call chat remove_spam
dfx canister call chat dump
```

### 8. Restore the canister state from the snapshot

Canister `chat` is currently running, and snapshots should not be applied to active canisters.
Therefore, the canister must be stopped before applying the snapshot and then restarted.

```sh
dfx canister stop chat
dfx canister snapshot list chat
dfx canister snapshot load chat 000000000000000080000000001000010101
dfx canister start chat
dfx canister call chat dump
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
