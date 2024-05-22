---
keywords: [beginner, motoko, canister logs, logging]
---

# Canister logs

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/canister_logs)

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).
- [x] Download the following project files from GitHub: `git clone https://github.com/dfinity/examples/`

You will need to have 3 terminal windows:
- Terminal A: Running a DFX instance and separating its output from anything else
- Terminal B: Deploying a canister and seeing its output
- Terminal C: Reading logs interactively

### Step 1: Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```shell
# Terminal A -- for running DFX and separating its output from anything else.
$ cd examples/motoko/canister_logs
$ dfx start --clean

# Terminal B -- for deploying the canister and calling its methods.
$ cd examples/motoko/canister_logs

# Terminal C -- for polling logs.
$ cd examples/motoko/canister_logs
```

### Step 2: Deploy the canister:

```shell
# Terminal B
$ dfx deploy
```

### Step 3: Check canister has no logs yet:

```shell
# Terminal B
$ dfx canister logs CanisterLogs

# Expect to see no logs.
```

### Step 4: Call `print` method and check the logs:

```shell
# Terminal B
$ dfx canister call CanisterLogs print hi
()

# Expect to see new log entry.
$ dfx canister logs CanisterLogs
[0. 2024-05-22T11:37:28.080234848Z]: hi
```

### Step 5: Start constantly polling logs:

In order not to call `dfx canister logs CanisterLogs` after every canister call in a separate terminal window/pane C start a script that will constantly poll logs:

```shell
# Terminal C
$ ./poll_logs.sh
[0. 2024-05-22T11:37:28.080234848Z]: hi

```

### Step 6: Call `print`, `trap`, `panic` and other canister methods:

```shell
# Terminal B
$ dfx canister call CanisterLogs print hi!
()

$ dfx canister call CanisterLogs print hello!
()

$ dfx canister call CanisterLogs print yey!
()

$ dfx canister call CanisterLogs trap oops!
Error: Failed update call.
Caused by: Failed update call.
  The replica returned a rejection error: reject code CanisterError, reject message Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: oops!, error code None

```

Observe recorded logs that might look similar to this:

```shell
# Terminal C
[0. 2024-05-22T12:21:52.333589617Z]: hi
[1. 2024-05-22T12:22:27.330332077Z]: hello!
[2. 2024-05-22T12:22:32.062734677Z]: yey!
[3. 2024-05-22T12:22:36.685104375Z]: right before trap
[4. 2024-05-22T12:22:36.685104375Z]: [TRAP]: oops!

```
