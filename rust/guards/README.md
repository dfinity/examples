# Guards and Async Code

## Summary

This example canister shows some advanced behaviour between guards and asynchronous code. This example is meant for Rust
canister developers that are already familiar
with [asynchronous code](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/async-code/)
and the security best-practices related
to [inter-canister calls and rollbacks](https://internetcomputer.org/docs/current/developer-docs/security/rust-canister-development-security-best-practices#inter-canister-calls-and-rollbacks).

## Guard to Maintain Invariants

This example canister stores a bunch of items on the heap, where each item is simply modelled as a `String`. One
invariant that the canister aims at maintaining is that each item is **processed at most once**, where processing an
item involves some asynchronous code. A concrete example of such a setting would be a minter canister processing minting
requests by contacting a ledger canister, where crucially double minting should be avoided.

One tricky part in this scenario is that an item can therefore only be marked as processed after the asynchronous code
has completed, meaning in the callback. As mentioned in
the [security best-practices](https://internetcomputer.org/docs/current/developer-docs/security/rust-canister-development-security-best-practices#securely-handle-traps-in-callbacks),
it's not always feasible to guarantee that the callback will not trap, which in that case would break the invariant due
to the state being rollbacked.

The standard solution to maintain such an invariant, despite a potential panic in the callback is to use a guard.
However, this example canister shows that it's also crucially important that the declaration of the guard happens in
another message than the callback, which is the case for true asynchronous code (e.g. inter-canister calls, `raw_rand`,
etc.). It's in particular not enough to `await` a function that's declared to be `async`, since if the future can polled
until completion directly, everything will be executed in a single message.

## Automated Integration Tests

To run the integration tests under `tests/` install [PocketIC server](https://github.com/dfinity/pocketic) and then run

```shell
cargo build --target wasm32-unknown-unknown --release && cargo test
```

## Manual Testing with `dfx`

### Setup

Start `dfx`

```shell
dfx start --background
```

and then proceed to deploying the canister

```shell
dfx deploy
```

You should now be able to query the canister, e.g., to check if an item is processed

```shell
dfx canister call guards is_item_processed 'mint'
```

which should return `(null)` since the canister currently has an empty state.

### Test

As an example, we show how the behaviour tested in `should_process_single_item_and_mark_it_as_processed` can be tested
manually.

Set the item `"mint"` to be processed:

```shell
dfx canister call guards set_non_processed_items 'vec { "mint" }'
```

As a sanity check, ensure that the item is not yet processed:

```shell
dfx canister call guards is_item_processed 'mint'
```

should return `(opt false)`.

Process the item by calling the *panicking* callback:

```shell
dfx canister call guards process_single_item_with_panicking_callback  '("mint", variant { TrueAsyncCall })'
```

Since the queried endpoint panicks on purpose, expect some error message similar to

```text
2024-05-29 11:54:39.817800 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] Panicked at 'panicking callback!', src/lib.rs:47:5
Error: Failed update call.
Caused by: Failed update call.
  The replica returned a rejection error: reject code CanisterError, reject message Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: Panicked at 'panicking callback!', src/lib.rs:47:5, error code None
```

Ensure that the guard was executed to ensure that the item is marked as processed despite the previous panic

```shell
dfx canister call guards is_item_processed 'mint'
```

should return `(opt true)`.
