# Canister logs

This example demonstrates canister logging on the Internet Computer. Every message written with `Debug.print` and every trap is recorded in the canister's log, which can be retrieved at any time with `icp canister logs`. The example covers:

- **Update calls** — `Debug.print` output from update methods
- **Replicated query calls** — `Debug.print` output from query methods called as updates
- **Explicit traps** — messages logged before and at the point of a `Runtime.trap`
- **Memory out-of-bounds** — automatically logged trap from accessing an unallocated region
- **Timer-triggered traps** — periodic traps set up at canister install time
- **Management canister calls** — `ic.raw_rand` logging success/failure

## Log entry format

Each log entry has a sequence number, a timestamp, and the message:

```
[0. 2024-05-23T08:32:26.203980235Z]: right before timer trap
[1. 2024-05-23T08:32:26.203980235Z]: [TRAP]: timer trap
[2. 2024-05-23T08:32:31.836721763Z]: right before timer trap
[3. 2024-05-23T08:32:31.836721763Z]: [TRAP]: timer trap
```

`Debug.print` messages appear as plain text. Trap messages are prefixed with `[TRAP]:`. When a function calls `Debug.print` before trapping, both entries appear in sequence — the print message first, then the trap.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/canister_logs
```

### Deploy and test
```bash
icp network start -d
icp deploy
make test
icp network stop
```

To inspect the raw log entries at any point:

```bash
icp canister logs backend
```

To watch logs stream in real-time while calling methods in a separate terminal:

```bash
./poll_logs.sh
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP dapp.
