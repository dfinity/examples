---
keywords: [beginner, rust, canister logs, logging]
---

# Canister logs

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/canister_logs)

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
$ cd examples/rust/canister_logs
$ dfx start --clean

# Terminal B -- for deploying the canister and calling its methods.
$ cd examples/rust/canister_logs

# Terminal C -- for polling logs.
$ cd examples/rust/canister_logs
```

### Step 2: Deploy the canister:

```shell
# Terminal B
$ dfx deploy
```

### Step 3: Check canister has no logs yet:

```shell
# Terminal B
$ dfx canister logs canister_logs

# Expect to see no logs.
```

### Step 4: Call `print` method and check the logs:

```shell
# Terminal B
$ dfx canister call canister_logs print hi
()

# Expect to see new log entry.
$ dfx canister logs canister_logs
[0. 2024-05-22T11:37:28.080234848Z]: hi
```

### Step 5: Start constantly polling logs:

In order not to call `dfx canister logs canister_logs` after every canister call in a separate terminal window/pane C start a script that will constantly poll logs:

```shell
# Terminal C
$ ./poll_logs.sh
[0. 2024-05-22T11:37:28.080234848Z]: hi

```

### Step 6: Call `print`, `trap`, `panic` and other canister methods:

```shell
# Terminal B
$ dfx canister call canister_logs print hi!
()

$ dfx canister call canister_logs print hello!
()

$ dfx canister call canister_logs print yey!
()

$ dfx canister call canister_logs trap oops!
Error: Failed update call.
Caused by: Failed update call.
  The replica returned a rejection error: reject code CanisterError, reject message Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: oops!, error code None

$ dfx canister call canister_logs panic aaa!
Error: Failed update call.
Caused by: Failed update call.
  The replica returned a rejection error: reject code CanisterError, reject message Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: Panicked at 'aaa!', src/lib.rs:17:5, error code None

$ dfx canister call canister_logs memory_oob
Error: Failed update call.
Caused by: Failed update call.
  The replica returned a rejection error: reject code CanisterError, reject message Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped: stable memory out of bounds, error code None

$ dfx canister call canister_logs failed_unwrap
Error: Failed update call.
Caused by: Failed update call.
  The replica returned a rejection error: reject code CanisterError, reject message Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', src/lib.rs:31:47, error code None

```

Observe recorded logs that might look similar to this:

```shell
# Terminal C
[0. 2024-05-22T11:37:28.080234848Z]: hi
[1. 2024-05-22T11:43:32.152363971Z]: hello!
[2. 2024-05-22T11:43:36.317710491Z]: yey!
[3. 2024-05-22T11:43:40.592174915Z]: right before trap
[4. 2024-05-22T11:43:40.592174915Z]: [TRAP]: oops!
[5. 2024-05-22T11:43:49.904081741Z]: right before panic
[6. 2024-05-22T11:43:49.904081741Z]: Panicked at 'aaa!', src/lib.rs:17:5
[7. 2024-05-22T11:43:49.904081741Z]: [TRAP]: Panicked at 'aaa!', src/lib.rs:17:5
[8. 2024-05-22T11:43:54.400015642Z]: right before memory out of bounds
[9. 2024-05-22T11:43:54.400015642Z]: [TRAP]: stable memory out of bounds
[10. 2024-05-22T11:43:59.810358166Z]: right before failed unwrap
[11. 2024-05-22T11:43:59.810358166Z]: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', src/lib.rs:31:47
[12. 2024-05-22T11:43:59.810358166Z]: [TRAP]: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', src/lib.rs:31:47
```
