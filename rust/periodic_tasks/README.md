---
keywords: [advanced, rust, periodic, timer, heartbeats]
---

# Periodic tasks and timers

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/periodic_tasks)

## Overview

Unlike other blockchains, the Internet Computer can automatically execute canister smart contracts after a specified delay or periodically.

There are two ways to schedule an automatic canister execution on the IC:

1. **Timers**: one-shot or periodic canister calls with specified minimum timeout or interval.
2. **Heartbeats**: legacy periodic canister invocations with intervals close to the blockchain finalization rate (1s). Heartbeats are supported by the IC for backward compatibility and some very special use cases. Newly developed canisters should prefer using timers over the heartbeats.

This example demonstrates different ways of scheduling periodic tasks on the Internet Computer: timers and heartbeats. The example shows the difference between the two and helps to decide which method suits you the best.

The example consists of two canisters named `heartbeat` and `timer`, both implementing the same functionality: schedule a periodic task to increase a counter.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

### Example 1:

- #### Step 1: Begin by opening a terminal window and navigating into the project's directory.

```sh
cd examples/rust/periodic_tasks
```

- #### Step 2: Start a clean local Internet Computer replica and a web server:

```sh
dfx stop
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or `dfx stop` command is run.

Example output:

```sh
% dfx stop && dfx start --clean
[...]
Dashboard: http://localhost:63387/_/dashboard
```

- #### Step 3: Open another terminal window in the same directory:

```sh
cd examples/rust/periodic_tasks
```

- #### Step 4: Compile and deploy `heartbeat` and `timer` canisters, setting the interval for periodic tasks to 10s:

```sh
dfx deploy heartbeat --argument 10
dfx deploy timer --argument 10
```

The counter will start increasing every ~10s in both canisters.

Example output:

```sh
% dfx deploy heartbeat --argument 10
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      heartbeat: http://127.0.0.1/...
      timer: http://127.0.0.1/...

% dfx deploy timer --argument 10
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      heartbeat: http://127.0.0.1/...
      timer: http://127.0.0.1/...
```

- #### Step 5: After 10s, observe similar non-zero counters in both canisters:

```sh
dfx canister call heartbeat counter
dfx canister call timer counter
```

Note, as the canisters deployed one by one, there might be a minor discrepancy in the counters.

Example output:

```sh
% dfx canister call heartbeat counter               
(8 : nat32)
% dfx canister call timer counter    
(7 : nat32)
```

- #### Step 6: Compare the amount of cycles used to schedule the periodic task with 10s interval:

```sh
dfx canister call heartbeat cycles_used
dfx canister call timer cycles_used
```

Example output:

```sh
% dfx canister call heartbeat cycles_used
(10_183_957 : nat64)
% dfx canister call timer cycles_used
(2_112_067 : nat64)
```

For periodic tasks with 10 sec intervals, the `heartbeat` canister uses *more* cycles than the `timer` canister.

Not only do timers use fewer cycles, but they are also more composable. As there is no global state or methods to export, different libraries with timers could be easily used in the same project.

Also, timers provide isolation between the scheduling logic and the periodic task. If the periodic task fails, all the changes made by this task will be reverted, but the timers library state will be updated, i.e. the failed task will be removed from the list of timers to execute.

For such isolation of execution and scheduling contexts, the internal timers library uses self-canister calls:

   ```rust
   # This is a pseudo-code of a self call:
   ic_cdk::call(ic_cdk::id(), "periodic_task", ());
   ```

Despite the [costs](https://internetcomputer.org/docs/current/developer-docs/production/computation-and-storage-costs) associated with such self-canister calls, the timers library still uses fewer cycles than the heartbeats.

### Example 2: Cycles usage for tasks with 1s interval


- #### Step 1: Open a new terminal window in the example root directory:

```sh
cd examples/rust/periodic_tasks
```

- #### Step 2: Start a clean local Internet Computer replica and a web server:

```sh
dfx stop
dfx start --clean
```

This terminal will stay blocked, printing log messages, until the `Ctrl+C` is pressed or `dfx stop` command is run.

- #### Step 3: Open another terminal window in the same directory:

```sh
cd examples/rust/periodic_tasks
```

Example output:

```sh
% dfx stop && dfx start --clean
[...]
Dashboard: http://localhost:63387/_/dashboard
```

- #### Step 4:. Compile and deploy `heartbeat` and `timer` canisters, setting the interval for periodic tasks to 1s:

```sh
dfx deploy --argument 1 heartbeat
dfx deploy --argument 1 timer
```

The counter will start increasing every second in both canisters.

Example output:

```sh
% dfx deploy --argument 1 heartbeat
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      heartbeat: http://127.0.0.1/...
      timer: http://127.0.0.1/...

% dfx deploy --argument 1 timer
[...]
Deployed canisters.
URLs:
   Backend canister via Candid interface:
      heartbeat: http://127.0.0.1/...
      timer: http://127.0.0.1/...
```

- #### Step 5: After a few seconds, observe similar non-zero counters in both canisters:

```sh
dfx canister call heartbeat counter
dfx canister call timer counter
```

Note, as the canisters deployed one by one, there might be a minor discrepancy in the counters.

Example output:

```sh
% dfx canister call heartbeat counter               
(8 : nat32)
% dfx canister call timer counter    
(9 : nat32)
```

- #### Step 6: Compare the number of cycles used to schedule the periodic task with 1s interval:

```sh
dfx canister call heartbeat cycles_used
dfx canister call timer cycles_used
```

Example output:

```sh
% dfx canister call heartbeat cycles_used
(2_456_567 : nat64)
% dfx canister call timer cycles_used
(4_545_326 : nat64)
```

For periodic tasks with 1 sec interval, the `heartbeat` canister uses *less* cycles than the `timer` canister.

Despite the `heartbeat` using fewer cycles in this case, this solution is hard to compose within a big project. If there are two libraries using heartbeats internally, they won't even compile together, as they both would be trying to export the global `canister_heartbeat`  method required for the heartbeats.

Also, there is no isolation between the scheduling logic and the periodic task. If the periodic task fails, all the changes made by the task and by the `canister_heartbeat` method will be reverted. So the failed task will be executed over and over again every heartbeat.

For such isolation of execution and scheduling contexts, the timers library uses internal self-canister calls as described in `Demo 1`. Due to the [costs](https://internetcomputer.org/docs/current/developer-docs/production/computation-and-storage-costs) associated with such self-canister calls, `timer` canister uses more cycles for very frequent periodic tasks.

## Further learning
1. Have a look at the locally running dashboard. The URL is at the end of the `dfx start` command: `Dashboard: http://localhost/...`
2. Check out `heartbeat` and `timer` canisters Candid user interface. The URLs are at the end of the `dfx deploy` command: `heartbeat: http://127.0.0.1/...`
3. Find which interval makes even the costs of running periodic tasks in the `timer` and `heartbeat` canisters: `dfx deploy heartbeat --argument 5 && dfx deploy timer --argument 5`

### Canister interface

The `heartbeat` and `timer` canisters provide the following interface:

* `counter` &mdash; returns the value of the `COUNTER`, i.e. how many times the periodic task was executed (query, both canisters)
* `start_with_interval_secs` &mdash; starts a new timer with the specified interval in seconds (timer canister)
* `set_interval_secs` &mdash; sets a new interval to call the periodic task in seconds (heartbeat canister)
* `stop` &mdash; stops executing periodic task (both canisters)
* `cycles_used` &mdash; returns the number of cycles observed in the periodic task (both canisters)

Example usage:

```sh
dfx canister call timer start_with_interval_secs 5
```

## Conclusion

For code composability, execution context isolation, and cost efficiency, canister developers should prefer to use timers over heartbeats.

As shown in `Example 2`, there might be still very specific use cases for the heartbeats. Those should be considered case by case, with composability and isolation issues in mind.


## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
