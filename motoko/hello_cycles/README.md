# Hello, cycles!

On the Internet Computer, canisters pay for computation and storage with cycles. This example demonstrates the three fundamental cycle management operations in Motoko:

1. **Inspect the balance** — read how many cycles the canister currently holds.
2. **Accept incoming cycles** — when a caller attaches cycles to a call, the canister must explicitly claim them. Unclaimed cycles are automatically refunded to the caller.
3. **Send cycles to another canister** — attach cycles to an outgoing inter-canister call and learn how many were refunded (not accepted by the receiver).

Operations 2 and 3 are two perspectives on the same transaction:
- The **receiver** calls `Cycles.accept()` and returns how many it took (`accepted`).
- The **sender** reads `Cycles.refunded()` after the call returns to learn how many cycles came back.

## Functions

- `get_balance()` — returns the canister's current cycle balance as `Nat`.
- `accept_cycles()` — accepts up to 10 million cycles from the caller; returns `{ accepted : Nat64 }`. The caller can inspect how many were claimed; any excess is refunded automatically.
- `send_cycles(receiver, amount)` — forwards `amount` cycles from this canister's balance to `receiver`; returns `{ refunded : Nat }`. A non-zero `refunded` means the receiver did not accept all of the offered cycles.

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

The tests cover both perspectives:

- **Test 2 (receiver side)**: calls `accept_cycles` through the local [proxy canister](https://cli.internetcomputer.org/0.3/guides/proxy-canister/) with 1M attached cycles — verifies `accepted > 0`. External callers cannot attach cycles directly; icp-cli routes the cycles via the proxy, which is automatically deployed on the local network.
- **Test 3 (sender side)**: calls `send_cycles` pointing to the canister's own `accept_cycles` with 5M cycles — verifies `refunded = 0` (all accepted within the 10M limit).
- **Test 4 (refund case)**: same but with 15M cycles — verifies `refunded = 5_000_000` (5M over the limit).

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
