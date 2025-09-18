# Guards and async code

This example canister shows some advanced behavior between guards and asynchronous code. This example is meant for Rust
canister developers that are already familiar
with [asynchronous code](https://internetcomputer.org/docs/references/async-code)
and the security best-practices related
to [inter-canister calls and rollbacks](https://internetcomputer.org/docs/building-apps/security/inter-canister-calls#inter-canister-calls-and-rollbacks).

## Guard to maintain invariants

This example canister stores a bunch of items on the heap, where each item is simply modelled as a `String`. One
invariant that the canister aims at maintaining is that each item is **processed at most once**, where processing an
item involves some asynchronous code. A concrete example of such a setting would be a minter canister processing minting
requests by contacting a ledger canister, where crucially double minting should be avoided.

One tricky part in this scenario is that an item can therefore only be marked as processed after the asynchronous code
has completed, meaning in the callback. As mentioned in
the [security best-practices](https://internetcomputer.org/docs/building-apps/security/inter-canister-calls#securely-handle-traps-in-callbacks),
it's not always feasible to guarantee that the callback will not trap, which in that case would break the invariant due
to the state being rolled back.

The standard solution to maintain such an invariant, despite a potential panic in the callback is to use a guard.
However, this example canister shows that it's also crucially important that the declaration of the guard happens in
another message than the callback, which is the case for true asynchronous code (e.g. inter-canister calls, `raw_rand`,
etc.). It's in particular not enough to `await` a function that's declared to be `async`, since if the future can polled
until completion directly, everything will be executed in a single message.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/guards)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Automated integration tests

To run the integration tests under `tests/` install [PocketIC server](https://github.com/dfinity/pocketic) and then run:

```shell
cargo build --target wasm32-unknown-unknown --release && cargo test
```

### Test

Below tests the behavior in `should_process_single_item_and_mark_it_as_processed` manually.

Set the item `"mint"` to be processed:

```shell
dfx canister call guards set_non_processed_items 'vec { "mint" }'
```

As a sanity check, ensure that the item is not yet processed:

```shell
dfx canister call guards is_item_processed 'mint'
```

This should return `(opt false)`.

Process the item by calling the *panicking* callback:

```shell
dfx canister call guards process_single_item_with_panicking_callback  '("mint", variant { TrueAsyncCall })'
```

Ensure that the guard was executed to ensure that the item is marked as processed despite the previous panic:

```shell
dfx canister call guards is_item_processed 'mint'
```

This should return `(opt true)`.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
