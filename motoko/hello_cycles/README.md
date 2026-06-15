# Hello, cycles!

On the Internet Computer, canisters pay for computation and storage with cycles. This example demonstrates the three fundamental cycle management operations in Motoko:

1. **Inspect the balance** — read the canister's current cycle balance with `Cycles.balance()`.
2. **Accept incoming cycles** — when a caller attaches cycles to a call, the canister must explicitly claim them with `Cycles.accept()`. Unclaimed cycles are refunded.
3. **Forward cycles to another canister** — attach cycles to an outgoing inter-canister call using the `(with cycles = N)` syntax, then read `Cycles.refunded()` to learn how many were not accepted.

The three public functions:

- `wallet_balance`: returns the canister's current cycle balance as a `Nat`.
- `wallet_receive`: accepts up to 10 million cycles from the caller and returns `{ accepted : Nat64 }`. The name follows a convention used by cycle-receiving canisters; the return type intentionally differs from a plain `() -> ()` so callers can confirm how many cycles were accepted.
- `transfer`: sends `amount` cycles to any shared function whose Candid signature is `() -> ()`, then returns `{ refunded : Nat }` — the cycles the receiver did not accept.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/hello_cycles
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
