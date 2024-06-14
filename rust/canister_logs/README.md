---
keywords: [beginner, rust, canister logs, logging]
---

# Canister logs

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/canister_logs)

## Prerequisites
This example requires an installation of:

- [x] DFX version 0.19.0 or newer
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

### Step 3: Check canister logs:

Expect to see logs from timer traps.

```shell
# Terminal B
$ dfx canister logs canister_logs
[0. 2024-05-22T12:35:32.050252022Z]: right before timer trap
[1. 2024-05-22T12:35:32.050252022Z]: [TRAP]: timer trap
[2. 2024-05-22T12:35:37.680315152Z]: right before timer trap
[3. 2024-05-22T12:35:37.680315152Z]: [TRAP]: timer trap

```

### Step 4: Call `print` method and check the logs:

```shell
# Terminal B
$ dfx canister call canister_logs print hi
()

# Expect to see new log entry.
$ dfx canister logs canister_logs
...
[18. 2024-05-22T12:36:20.881326098Z]: right before timer trap
[19. 2024-05-22T12:36:20.881326098Z]: [TRAP]: timer trap
[20. 2024-05-22T12:36:26.305162772Z]: hi
[21. 2024-05-22T12:36:27.185879186Z]: right before timer trap
[22. 2024-05-22T12:36:27.185879186Z]: [TRAP]: timer trap
```

### Step 5: Start constantly polling logs:

In order not to call `dfx canister logs canister_logs` after every canister call in a separate terminal window/pane C start a script that will constantly poll logs:

```shell
# Terminal C
$ ./poll_logs.sh
...
[18. 2024-05-22T12:36:20.881326098Z]: right before timer trap
[19. 2024-05-22T12:36:20.881326098Z]: [TRAP]: timer trap
[20. 2024-05-22T12:36:26.305162772Z]: hi
[21. 2024-05-22T12:36:27.185879186Z]: right before timer trap
[22. 2024-05-22T12:36:27.185879186Z]: [TRAP]: timer trap
...
```

### Step 6: Call `print`, `trap` and other canister methods:

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
...
[45. 2024-05-22T12:37:33.0576873Z]: right before timer trap
[46. 2024-05-22T12:37:33.0576873Z]: [TRAP]: timer trap
[47. 2024-05-22T12:37:33.773343176Z]: hi!
[48. 2024-05-22T12:37:37.558075267Z]: hello!
[49. 2024-05-22T12:37:38.349121524Z]: right before timer trap
[50. 2024-05-22T12:37:38.349121524Z]: [TRAP]: timer trap
[51. 2024-05-22T12:37:41.466030479Z]: yey!
[52. 2024-05-22T12:37:43.7472275Z]: right before timer trap
[53. 2024-05-22T12:37:43.7472275Z]: [TRAP]: timer trap
[54. 2024-05-22T12:37:45.302285184Z]: right before trap
[55. 2024-05-22T12:37:45.302285184Z]: [TRAP]: oops!
[56. 2024-05-22T12:37:48.900425146Z]: right before timer trap
[57. 2024-05-22T12:37:48.900425146Z]: [TRAP]: timer trap
[58. 2024-05-22T12:37:49.736443986Z]: right before panic
[59. 2024-05-22T12:37:49.736443986Z]: Panicked at 'aaa!', src/lib.rs:37:5
[60. 2024-05-22T12:37:49.736443986Z]: [TRAP]: Panicked at 'aaa!', src/lib.rs:37:5
[61. 2024-05-22T12:37:54.122929037Z]: right before timer trap
[62. 2024-05-22T12:37:54.122929037Z]: [TRAP]: timer trap
[63. 2024-05-22T12:37:54.94948481Z]: right before memory out of bounds
[64. 2024-05-22T12:37:54.94948481Z]: [TRAP]: stable memory out of bounds
[65. 2024-05-22T12:37:59.693695919Z]: right before failed unwrap
[66. 2024-05-22T12:37:59.693695919Z]: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', src/lib.rs:51:47
[67. 2024-05-22T12:37:59.693695919Z]: [TRAP]: Panicked at 'called `Result::unwrap()` on an `Err` value: FromUtf8Error { bytes: [192, 255, 238], error: Utf8Error { valid_up_to: 0, error_len: Some(1) } }', src/lib.rs:51:47
[68. 2024-05-22T12:38:00.621855713Z]: right before timer trap
[69. 2024-05-22T12:38:00.621855713Z]: [TRAP]: timer trap
...
```
