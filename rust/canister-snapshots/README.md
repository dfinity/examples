# Canister snapshots

This example demonstrates the process of taking and restoring canister snapshots.
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

`bash test.sh` automates the full snapshot lifecycle documented below.

## How it works

The backend canister maintains a `CHAT` vector and a `LENGTH` counter in heap memory.
Canister snapshots capture the full state of a stopped canister — WASM module, WASM memory,
stable memory, and chunk store — and can restore it after a bad deployment or logic error.

Key constraint: **the canister must be stopped** before taking or restoring a snapshot.

## Step-by-step walkthrough

### Step 1: Populate the database

```bash
icp canister call backend append '("Hi there!")'
icp canister call --query backend dump '()'
```

Example output:

```
()
(vec { "Hi there!" })
```

### Step 2: Take a snapshot

Snapshots must not be taken of running canisters — stop it first, then restart after.

```bash
icp canister stop backend
icp canister snapshot create backend
icp canister start backend
```

Example output:

```
0000000000000000ffffffffffa000020101
```

### Step 3: Simulate data loss

`remove_spam` contains a deliberate bug: it always clears the chat via `CHAT.take()`,
even when no spam is found.

```bash
icp canister call backend remove_spam '()'
icp canister call --query backend dump '()'
```

Example output:

```
(0 : nat64)
(vec {})
```

### Step 4: Restore from snapshot

Stop the canister, restore the snapshot, then start it again.

```bash
icp canister stop backend
icp canister snapshot restore backend 0000000000000000ffffffffffa000020101
icp canister start backend
icp canister call --query backend dump '()'
```

Example output:

```
(vec { "Hi there!" })
```

The canister state has been successfully restored.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize
yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview)
for developing on the Internet Computer. This example may not implement all the best practices.
