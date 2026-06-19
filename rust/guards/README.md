# Guards and async code

This example canister shows some advanced behavior between guards and asynchronous code. This example is meant for Rust
canister developers that are already familiar
with [inter-canister calls](https://docs.internetcomputer.org/guides/canister-calls/inter-canister-calls)
and the [security best practices](https://docs.internetcomputer.org/guides/security/inter-canister-calls/#inter-canister-calls-and-rollbacks) related to it.

## Guard to maintain invariants

This example canister stores a bunch of items on the heap, where each item is simply modelled as a `String`. One
invariant that the canister aims at maintaining is that each item is **processed at most once**, where processing an
item involves some asynchronous code. A concrete example of such a setting would be a minter canister processing minting
requests by contacting a ledger canister, where crucially double minting should be avoided.

One tricky part in this scenario is that an item can therefore only be marked as processed after the asynchronous code
has completed, meaning in the callback. As mentioned in the
[security best practices](https://docs.internetcomputer.org/guides/security/inter-canister-calls/#securely-handle-traps-in-callbacks),
it's not always feasible to guarantee that the callback will not trap, which in that case would break the invariant due
to the state being rolled back.

The standard solution to maintain such an invariant, despite a potential panic in the callback is to use a guard.
However, this example canister shows that it's also crucially important that the declaration of the guard happens in
another message than the callback, which is the case for true asynchronous code (e.g. inter-canister calls, `raw_rand`,
etc.). It's in particular not enough to `await` a function that's declared to be `async`, since if the future can be
polled until completion directly, everything will be executed in a single message.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- Rust toolchain with `wasm32-unknown-unknown` target (only for PocketIC tests): [rustup.rs](https://rustup.rs)

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/guards
```

### Deploy and test with icp-cli

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

`bash test.sh` exercises the guard behavior via `icp canister call` — verifying that `TrueAsyncCall` marks items as processed and `FalseAsyncCall` does not, and asserting the exact trap message in each case.

### Run PocketIC integration tests

The `backend/tests/` directory contains Rust integration tests using [PocketIC](https://docs.internetcomputer.org/guides/testing/pocket-ic). These cover the same guard/async scenarios as `test.sh` and additionally test **parallel processing prevention** — submitting two concurrent calls for the same item and asserting that one is rejected with "Item already in processing!". This concurrent scenario cannot be reliably expressed in a bash script.

First, install the [PocketIC server](https://github.com/dfinity/pocketic/releases):

```bash
# macOS
curl -sL https://github.com/dfinity/pocketic/releases/download/14.0.0/pocket-ic-x86_64-darwin.gz | gunzip > pocket-ic-server
# Linux
curl -sL https://github.com/dfinity/pocketic/releases/download/14.0.0/pocket-ic-x86_64-linux.gz | gunzip > pocket-ic-server
chmod +x pocket-ic-server
export POCKET_IC_BIN=$(pwd)/pocket-ic-server
```

Then build the canister WASM and run the tests:

```bash
cargo build --package backend --target wasm32-unknown-unknown --release
cargo test --package backend
```

### Manual test walkthrough

Below manually tests the behavior demonstrated by the `TrueAsyncCall` variant.

Set the item `"mint"` to be processed:

```shell
icp canister call backend set_non_processed_items '(vec { "mint" })'
```

As a sanity check, ensure that the item is not yet processed:

```shell
icp canister call --query backend is_item_processed '("mint")'
```

This should return `(opt false)`.

Process the item by calling the *panicking* callback with a true async call (the guard fires in `call_on_cleanup`):

```shell
icp canister call backend process_single_item_with_panicking_callback '("mint", variant { TrueAsyncCall })'
```

Ensure that the guard was executed and the item is marked as processed despite the previous panic:

```shell
icp canister call --query backend is_item_processed '("mint")'
```

This should return `(opt true)`.

Now repeat the test with `FalseAsyncCall` — here the future is polled to completion in a single message, so the
panic reverts all state changes and the guard has no effect:

```shell
icp canister call backend set_non_processed_items '(vec { "mint" })'
icp canister call backend process_single_item_with_panicking_callback '("mint", variant { FalseAsyncCall })'
icp canister call --query backend is_item_processed '("mint")'
```

This should return `(opt false)` — the item was **not** marked as processed.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
