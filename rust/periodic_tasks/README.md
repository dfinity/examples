# Periodic tasks and timers

Unlike other blockchains, the Internet Computer can automatically execute canister smart contracts after a specified delay or periodically.

There are two ways to schedule an automatic canister execution on the IC:

1. **Timers**: one-shot or periodic canister calls with specified minimum timeout or interval.
2. **Heartbeats**: legacy periodic canister invocations with intervals close to the blockchain finalization rate (1s). Heartbeats are supported by the IC for backward compatibility and some very special use cases. Newly developed canisters should prefer using timers over the heartbeats.

This example demonstrates both scheduling approaches. It consists of two canisters, `heartbeat` and `timer`, both implementing the same functionality: schedule a periodic task to increase a counter.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```sh
git clone https://github.com/dfinity/examples
cd examples/rust/periodic_tasks
```

### Deploy and test

```sh
icp network start -d
icp deploy
bash test.sh
icp network stop
```

Both canisters are deployed with an initial interval of 10 seconds. After deployment, the counters start incrementing automatically — the `bash test.sh` command polls until the counters and cycles-usage values are non-zero.

## Comparing timers and heartbeats

### Cycles usage at 10-second intervals

For periodic tasks with a 10-second interval, the `heartbeat` canister uses *more* cycles than the `timer` canister:

```sh
icp canister call --query timer cycles_used '()'
# (2_112_067 : nat64)

icp canister call --query heartbeat cycles_used '()'
# (10_183_957 : nat64)
```

Not only do timers use fewer cycles, but they are also more composable. As there is no global state or methods to export, different libraries with timers can be used in the same project.

Timers also provide isolation between the scheduling logic and the periodic task. If the periodic task fails, all changes made by the task are reverted, but the timers library state is updated — the failed task is removed from the timer list. The internal timers library achieves this isolation via self-canister calls:

```rust
// Pseudo-code of the internal self-call:
ic_cdk::call(ic_cdk::id(), "periodic_task", ());
```

### Cycles usage at 1-second intervals

At 1-second intervals the picture inverts: the `heartbeat` canister uses *fewer* cycles than `timer`:

```sh
icp canister call --query timer cycles_used '()'
# (4_545_326 : nat64)

icp canister call --query heartbeat cycles_used '()'
# (2_456_567 : nat64)
```

Despite the `heartbeat` using fewer cycles at very high frequencies, this solution is hard to compose. If two libraries both export `canister_heartbeat`, they cannot be used in the same project. There is also no isolation: if the periodic task fails, the `canister_heartbeat` changes are reverted and the task fires again on every subsequent heartbeat.

The breakeven interval — where timer and heartbeat costs are approximately equal — is around 5 seconds.

### Canister interface

Both canisters expose:

- `counter` — returns how many times the periodic task has been executed (query)
- `stop` — stops the periodic task (update)
- `cycles_used` — returns cycles consumed by the periodic task (query)

The `timer` canister also exposes:

- `start_with_interval_secs` — starts a new timer with the given interval in seconds (update)

The `heartbeat` canister also exposes:

- `set_interval_secs` — adjusts the heartbeat-check interval in seconds (update)

## Conclusion

For code composability, execution context isolation, and cost efficiency, canister developers should prefer timers over heartbeats. Heartbeats may still be useful in very specific cases requiring sub-second periodic execution — these should be evaluated individually with composability and isolation trade-offs in mind.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. This example may not implement all the best practices.
