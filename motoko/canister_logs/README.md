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

### Step 3: Check canister logs:

Expect to see logs from timer traps.

```shell
# Terminal B
$ dfx canister logs CanisterLogs
[0. 2024-05-23T08:14:40.60349175Z]: right before timer trap
[1. 2024-05-23T08:14:40.60349175Z]: [TRAP]: timer trap
[2. 2024-05-23T08:14:46.160555676Z]: right before timer trap
[3. 2024-05-23T08:14:46.160555676Z]: [TRAP]: timer trap
```

### Step 4: Call `print` method and check the logs:

```shell
# Terminal B
$ dfx canister call CanisterLogs print hi
()

# Expect to see new log entry.
$ dfx canister logs CanisterLogs
...
[14. 2024-05-23T08:15:15.829022842Z]: right before timer trap
[15. 2024-05-23T08:15:15.829022842Z]: [TRAP]: timer trap
[16. 2024-05-23T08:15:19.728382106Z]: hi
[17. 2024-05-23T08:15:21.079923035Z]: right before timer trap
[18. 2024-05-23T08:15:21.079923035Z]: [TRAP]: timer trap
...
```

### Step 5: Start constantly polling logs:

In order not to call `dfx canister logs CanisterLogs` after every canister call in a separate terminal window/pane C start a script that will constantly poll logs:

```shell
# Terminal C
$ ./poll_logs.sh
...
[14. 2024-05-23T08:15:15.829022842Z]: right before timer trap
[15. 2024-05-23T08:15:15.829022842Z]: [TRAP]: timer trap
[16. 2024-05-23T08:15:19.728382106Z]: hi
[17. 2024-05-23T08:15:21.079923035Z]: right before timer trap
[18. 2024-05-23T08:15:21.079923035Z]: [TRAP]: timer trap
...
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
...
[49. 2024-05-23T08:16:40.702368542Z]: right before timer trap
[50. 2024-05-23T08:16:40.702368542Z]: [TRAP]: timer trap
[51. 2024-05-23T08:16:42.785727368Z]: hi!
[52. 2024-05-23T08:16:45.69960063Z]: right before timer trap
[53. 2024-05-23T08:16:45.69960063Z]: [TRAP]: timer trap
[54. 2024-05-23T08:16:47.222911354Z]: hello!
[55. 2024-05-23T08:16:50.875445893Z]: right before timer trap
[56. 2024-05-23T08:16:50.875445893Z]: [TRAP]: timer trap
[57. 2024-05-23T08:16:51.570602735Z]: yey!
[58. 2024-05-23T08:16:55.783308056Z]: right before timer trap
[59. 2024-05-23T08:16:55.783308056Z]: [TRAP]: timer trap
[60. 2024-05-23T08:16:56.536701165Z]: right before trap
[61. 2024-05-23T08:16:56.536701165Z]: [TRAP]: oops!
[62. 2024-05-23T08:17:00.759041661Z]: right before timer trap
[63. 2024-05-23T08:17:00.759041661Z]: [TRAP]: timer trap
[64. 2024-05-23T08:17:05.657467481Z]: right before timer trap
[65. 2024-05-23T08:17:05.657467481Z]: [TRAP]: timer trap
...

```
