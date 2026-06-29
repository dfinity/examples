# Canister logs

This example demonstrates canister logging and error handling on the Internet Computer. It shows how to use `ic_cdk::println!` for diagnostic output, how traps, panics, and memory errors are automatically captured in the canister log, and how periodic timers generate log entries.

The canister exposes methods to print messages, trigger explicit traps, cause panics, simulate stable-memory out-of-bounds errors, and call `raw_rand`. A key behavioral distinction is demonstrated: log messages from **update calls and replicated queries** are recorded in the canister log, while messages from **non-replicated query calls** are not.

See also the [Motoko version](../../motoko/canister_logs).

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/canister_logs
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

### Check canister logs

After deploying, inspect logs directly:

```bash
icp canister logs backend
```

Expect to see periodic timer trap entries every 5 seconds:

```
[0. 2024-05-23T08:32:26Z]: right before timer trap
[1. 2024-05-23T08:32:26Z]: [TRAP]: timer trap
[2. 2024-05-23T08:32:31Z]: right before timer trap
[3. 2024-05-23T08:32:31Z]: [TRAP]: timer trap
```

To follow logs in real time:

```bash
icp canister logs --follow backend
```

### Call canister methods manually

```bash
# Print a message (update call — recorded in logs)
icp canister call backend print '("hello!")'

# Print via replicated query (recorded in logs)
icp canister call backend print_query '("hello from query!")'

# Print via non-replicated query (NOT recorded in logs)
icp canister call --query backend print_query '("this will not appear in logs")'

# Trigger an explicit trap (error returned, message recorded in logs)
icp canister call backend trap '("oops!")'

# Trigger a Rust panic (error returned, message recorded in logs)
icp canister call backend panic '("something went wrong")'

# Trigger a stable-memory out-of-bounds error (recorded in logs)
icp canister call backend memory_oob '()'

# Trigger a failed unwrap (recorded in logs)
icp canister call backend failed_unwrap '()'

# Call raw_rand (result returned, success message recorded in logs)
icp canister call backend raw_rand '()'
```

## Security considerations and best practices

When building production canisters, review the [ICP security best practices](https://docs.internetcomputer.org/guides/security/overview).
