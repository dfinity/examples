# Canister snapshots

This example demonstrates the process of taking and loading canister snapshots.
It features a canister that functions as a miniature chat database.
The `remove_spam` canister method intentionally includes a bug to simulate data loss.
The example outlines the steps to deploy the canister, create a snapshot,
and subsequently restore the data after the simulated data loss.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/rust/canister-snapshots
```

### Deploy and test
```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## How it works

The backend canister maintains a `CHAT` vector and a `LENGTH` counter in heap memory.
The `remove_spam` function contains a deliberate bug: it clears the chat when no spam
is found (because `CHAT.take()` always empties the thread-local even when no messages
are removed), demonstrating unintended data loss.

Canister snapshots let you capture the full state of a stopped canister and reload it
after a bad deployment or logic error:

1. **Stop** the canister before snapshotting — snapshots must not be taken of running canisters.
2. **Create** the snapshot: `icp canister snapshot create backend`
3. **Start** the canister again for normal operation.
4. After data loss, **stop** the canister, **restore** the snapshot, then **start** it again.

Canister Snapshots are a powerful tool for developers.
In the event of accidental data loss, bugs, or configuration errors,
canisters can be quickly restored to a previous working state.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize
yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview)
for developing on the Internet Computer. This example may not implement all the best practices.
