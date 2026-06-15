# Hello, cycles!

On the Internet Computer, canisters pay for computation and storage with cycles. This example demonstrates the three fundamental cycle management operations in Motoko:

1. **Inspect the balance** — read how many cycles the canister currently holds.
2. **Accept incoming cycles** — when a caller attaches cycles to a call, the canister must explicitly claim them. Unclaimed cycles are automatically refunded to the caller.
3. **Send cycles to another canister** — attach cycles to an outgoing inter-canister call and learn how many were refunded (not accepted by the receiver).

Operations 2 and 3 are two perspectives on the same transaction:
- The **receiver** calls `Cycles.accept()` and returns how many it took (`accepted`).
- The **sender** reads `Cycles.refunded()` after the call returns to learn how many cycles came back.

## Functions

- `getBalance()` — returns the canister's current cycle balance as `Nat`.
- `acceptCycles()` — accepts up to 10 million cycles from the caller; returns `{ accepted : Nat64 }`. Any excess is refunded automatically.
- `sendCycles(receiver, amount)` — forwards `amount` cycles from this canister's balance to `receiver`; returns `{ refunded : Nat }`. A non-zero `refunded` means the receiver did not accept all of the offered cycles.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`
- jq (used in `make test` to read the proxy canister principal)

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

- **Test 2 (receiver side)**: calls `acceptCycles` through the local [proxy canister](https://cli.internetcomputer.org/0.3/guides/proxy-canister/) with 1M attached cycles — verifies `accepted > 0`. External callers cannot attach cycles directly; icp-cli routes the cycles via the proxy, which is automatically deployed on the local network.
- **Tests 3 & 4 (sender side)**: calls `sendCycles` with the canister's **own** `acceptCycles` as the receiver (an inter-canister self-call — no second canister needed). The 5M/15M cycles leave the canister via `sendCycles` and arrive at `acceptCycles`, which accepts up to the 10M limit. Test 3 sends 5M → `refunded = 0` (within limit). Test 4 sends 15M → `refunded = 5_000_000` (5M over the limit). In practice, `sendCycles` would target a different canister.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
