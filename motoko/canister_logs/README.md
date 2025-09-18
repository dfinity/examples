# Canister logs

This sample project demonstrates a basic logging and error handling system for a canister deployed on the Internet Computer. The `canister_logs` project showcases how to utilize logging for debugging and monitoring canister operations. It also demonstrates the use of timers and error handling through traps.

The `canister_logs` canister is designed to periodically log messages and simulate errors using traps. It provides methods to print messages, trigger traps, and handle memory out-of-bounds errors. The project includes a script to continuously poll and display logs, making it easier to monitor canister activity in real-time.

## Prerequisites

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install). For local testing, `dfx >= 0.22.0` is required.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Run" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/motoko/canister_logs)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

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

```shell
# Terminal B
dfx deploy
```

### Step 5: Check canister logs

Expect to see logs from timer traps.

```shell
# Terminal B
$ dfx canister logs CanisterLogs
[0. 2024-05-23T08:32:26.203980235Z]: right before timer trap
[1. 2024-05-23T08:32:26.203980235Z]: [TRAP]: timer trap
[2. 2024-05-23T08:32:31.836721763Z]: right before timer trap
[3. 2024-05-23T08:32:31.836721763Z]: [TRAP]: timer trap
```

### Step 6: Call `print` method and check the logs

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

### Step 7: Start constantly polling logs

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

### Step 8: Call `print`, `trap` and other canister methods

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

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
