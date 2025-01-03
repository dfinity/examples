# Canister logs

## Prerequisites

- [x] Install the [IC
  SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install). For local testing, `dfx >= 0.22.0` is required.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Step 1: Setup project environment

Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```shell
dfx start --clean
```

You will need to have 3 terminal windows:
- Terminal A: Running a `dfx` instance and separating its output from anything else.
- Terminal B: Deploying a canister and seeing its output.
- Terminal C: Reading logs interactively.

```shell
# Terminal A -- for running DFX and separating its output from anything else.
cd examples/motoko/canister_logs

# Terminal B -- for deploying the canister and calling its methods.
cd examples/motoko/canister_logs

# Terminal C -- for polling logs.
cd examples/motoko/canister_logs
```

## Step 2: Deploy the canister

```shell
# Terminal B
dfx deploy
```

## Step 3: Check canister logs

Expect to see logs from timer traps.

```shell
# Terminal B
$ dfx canister logs CanisterLogs
[0. 2024-05-23T08:32:26.203980235Z]: right before timer trap
[1. 2024-05-23T08:32:26.203980235Z]: [TRAP]: timer trap
[2. 2024-05-23T08:32:31.836721763Z]: right before timer trap
[3. 2024-05-23T08:32:31.836721763Z]: [TRAP]: timer trap
```

## Step 4: Call `print` method and check the logs

```shell
# Terminal B
$ dfx canister call CanisterLogs print hi
()

# Expect to see new log entry.
$ dfx canister logs CanisterLogs
...
[8. 2024-05-23T08:32:46.598972616Z]: right before timer trap
[9. 2024-05-23T08:32:46.598972616Z]: [TRAP]: timer trap
[10. 2024-05-23T08:32:48.713755238Z]: hi
[11. 2024-05-23T08:32:51.623988313Z]: right before timer trap
[12. 2024-05-23T08:32:51.623988313Z]: [TRAP]: timer trap
...
```

## Step 5: Start constantly polling logs

In order not to call `dfx canister logs CanisterLogs` after every canister call in a separate terminal window/pane C start a script that will constantly poll logs:

```shell
# Terminal C
$ ./poll_logs.sh
...
[8. 2024-05-23T08:32:46.598972616Z]: right before timer trap
[9. 2024-05-23T08:32:46.598972616Z]: [TRAP]: timer trap
[10. 2024-05-23T08:32:48.713755238Z]: hi
[11. 2024-05-23T08:32:51.623988313Z]: right before timer trap
[12. 2024-05-23T08:32:51.623988313Z]: [TRAP]: timer trap
...
```

## Step 6: Call `print`, `trap` and other canister methods

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

$ dfx canister call CanisterLogs memory_oob
Error: Failed update call.
Caused by: Failed update call.
  The replica returned a rejection error: reject code CanisterError, reject message Canister bkyz2-fmaaa-aaaaa-qaaaq-cai trapped explicitly: StableMemory range out of bounds, error code None

```

Observe recorded logs that might look similar to this:

```shell
# Terminal C
...
[19. 2024-05-23T08:33:11.319493785Z]: right before timer trap
[20. 2024-05-23T08:33:11.319493785Z]: [TRAP]: timer trap
[21. 2024-05-23T08:33:14.229855179Z]: hi!
[22. 2024-05-23T08:33:16.413512126Z]: right before timer trap
[23. 2024-05-23T08:33:16.413512126Z]: [TRAP]: timer trap
[24. 2024-05-23T08:33:18.622686552Z]: hello!
[25. 2024-05-23T08:33:21.519088681Z]: right before timer trap
[26. 2024-05-23T08:33:21.519088681Z]: [TRAP]: timer trap
[27. 2024-05-23T08:33:22.96101893Z]: yey!
[28. 2024-05-23T08:33:26.601860526Z]: right before timer trap
[29. 2024-05-23T08:33:26.601860526Z]: [TRAP]: timer trap
[30. 2024-05-23T08:33:28.039227914Z]: right before trap
[31. 2024-05-23T08:33:28.039227914Z]: [TRAP]: oops!
[32. 2024-05-23T08:33:31.634215234Z]: right before timer trap
[33. 2024-05-23T08:33:31.634215234Z]: [TRAP]: timer trap
[34. 2024-05-23T08:33:35.96761902Z]: right before memory out of bounds
[35. 2024-05-23T08:33:35.96761902Z]: [TRAP]: StableMemory range out of bounds
[36. 2024-05-23T08:33:36.712223153Z]: right before timer trap
[37. 2024-05-23T08:33:36.712223153Z]: [TRAP]: timer trap
...

```
