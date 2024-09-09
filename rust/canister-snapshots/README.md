---
keywords: [advanced, rust, backup, restore, snapshots]
---

# Canister Snapshots Example

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/canister-snapshots)

## Overview

This example demonstrates the process of taking and loading canister snapshots.
It features a canister named `chat`, which functions as a miniature database.
The `remove_spam` canister method intentionally includes a bug to simulate data loss.
The example outlines the steps to install the canister, create a snapshot,
and subsequently restore the data after the simulated data loss.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx). Note: the Canister Snapshots feature requires `dfx` version `0.23.0-beta.3` or later.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

- ### Step 1: Begin by opening a terminal window and navigating into the project's directory

```sh
cd examples/rust/canister-snapshots
```

- ### Step 2: Start a clean local Internet Computer replica and a web server

```sh
dfx stop
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or `dfx stop` command is run.

Example output:

```sh
% dfx stop && dfx start --clean
Running dfx start for version 0.23.0-beta.3
[...]
Replica API running on 127.0.0.1:4943
```

- ### Step 3: Open another terminal window in the same directory

```sh
cd examples/rust/canister-snapshots
```

- ### Step 4: Compile and deploy `chat` canister

```sh
dfx deploy
```

Example output:

```sh
% dfx deploy
Deploying all canisters.
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      chat: http://127.0.0.1:4943/?canisterId=...
```

- ### Step 5: Populate the database with data

```sh
dfx canister call chat append 'Hi there!'
dfx canister call chat dump
```

Example output:

```sh
% dfx canister call chat append 'Hi there!'
()
% dfx canister call chat dump
(vec { "Hi there!" })
```

- ### Step 6: Take canister snapshot

Canister `chat` is currently running, and snapshots should not be taken of active canisters.
Therefore, the canister should be stopped before taking a snapshot and then restarted.

```sh
dfx canister stop chat
dfx canister snapshot create chat
dfx canister start chat
```

Example output:

```sh
% dfx canister stop chat
Stopping code for canister chat, with canister_id bkyz2-fmaaa-aaaaa-qaaaq-cai
% dfx canister snapshot create chat
Created a new snapshot of canister chat. Snapshot ID: 000000000000000080000000001000010101
% dfx canister start chat
Starting code for canister chat, with canister_id bkyz2-fmaaa-aaaaa-qaaaq-cai
```

- ### Step 7: Simulate data loss

The `remove_spam` canister method intentionally includes a bug to simulate data loss.

```sh
dfx canister call chat remove_spam
dfx canister call chat dump
```

Example output:

```sh
% dfx canister call chat remove_spam
(0 : nat64)
% dfx canister call chat dump
(vec {})
```

There is no more data in the database.

- ### Step 8: Restore the canister state from the snapshot

Canister `chat` is currently running, and snapshots should not be applied to active canisters.
Therefore, the canister must be stopped before applying the snapshot and then restarted.

```sh
dfx canister stop chat
dfx canister snapshot list chat
dfx canister snapshot load chat 000000000000000080000000001000010101
dfx canister start chat
dfx canister call chat dump
```

Example output:

```sh
% dfx canister stop chat
Stopping code for canister chat, with canister_id bkyz2-fmaaa-aaaaa-qaaaq-cai
% dfx canister snapshot list chat
000000000000000080000000001000010101: 1.61MiB, taken at 2024-08-27 18:19:20 UTC
% dfx canister snapshot load chat 000000000000000080000000001000010101
Loaded snapshot 000000000000000080000000001000010101 in canister chat
% dfx canister start chat
Starting code for canister chat, with canister_id bkyz2-fmaaa-aaaaa-qaaaq-cai
% dfx canister call chat dump
(vec { "Hi there!" })
```

The canister state has been successfully restored from the snapshot.

## Conclusion

Canister Snapshots are a powerful new tool for developers.
In the event of accidental data loss, bugs, or configuration errors,
canisters can be quickly restored to a previous working state.
Snapshots help ensure that critical data and services remain accessible
even in the face of unexpected events, providing developers with peace of mind.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize
yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/)
for developing on the Internet Computer. This example may not implement all the best practices.
