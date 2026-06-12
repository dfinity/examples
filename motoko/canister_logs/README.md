# Canister logs

This example demonstrates canister logging on the Internet Computer. Every message written with `Debug.print` and every trap is recorded in the canister's log, which can be retrieved at any time with `icp canister logs`. The example covers:

- **Update calls** — `Debug.print` output from update methods
- **Replicated query calls** — `Debug.print` output from query methods called as updates
- **Explicit traps** — messages logged before and at the point of a `Runtime.trap`
- **Memory out-of-bounds** — automatically logged trap from accessing an unallocated region
- **Timer-triggered traps** — periodic traps set up at canister install time
- **Management canister calls** — `ic.raw_rand` logging success/failure

## Log entry format

`icp canister logs` returns JSON with a `log_records` array. Each entry has an `index`, a nanosecond `timestamp`, and a `content` string:

```json
{
  "log_records": [
    { "index": 0, "timestamp": 1781210195740276000, "content": "right before timer trap" },
    { "index": 1, "timestamp": 1781210195740276000, "content": "[TRAP]: timer trap" },
    { "index": 2, "timestamp": 1781210200674527000, "content": "right before timer trap" },
    { "index": 3, "timestamp": 1781210200674527000, "content": "[TRAP]: timer trap" }
  ]
}
```

`Debug.print` messages appear as plain text. Trap messages are prefixed with `[TRAP]:`. When a function calls `Debug.print` before trapping, both entries appear in sequence — the print message first, then the trap.

To extract just the messages in a readable format:

```bash
# Pretty-print with jq
icp canister logs backend | jq -r '.log_records[] | "[\(.index)]: \(.content)"'

# Or with python (no extra tools needed)
icp canister logs backend | python3 -m json.tool
```

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

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

To watch new log entries stream in real-time while calling methods in a separate terminal:

```bash
./poll_logs.sh
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
