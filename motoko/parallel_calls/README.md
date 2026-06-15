# Parallel inter-canister calls

This example demonstrates parallel inter-canister calls in Motoko and highlights the key differences between sequential and parallel call patterns. Running independent calls in parallel can significantly reduce latency, especially when messages are sent across subnets. For example, a canister that swaps two tokens might want to launch both token transfers in parallel.

Two canisters are deployed:

- **`caller`** — two endpoints: `sequential_calls(n)` (issues `n` calls one at a time) and `parallel_calls(n)` (issues all `n` calls at once). The callee principal is read from `PUBLIC_CANISTER_ID:callee`, injected automatically by icp-cli during `icp deploy`.
- **`callee`** — a minimal `ping` endpoint that takes no parameters and returns nothing.

## Single-subnet behaviour

With a small number of calls (e.g. 10), sequential and parallel both succeed and return the same result. This is expected: on a single subnet, inter-canister calls have almost no latency, so there is little benefit to running them in parallel.

With a large number of calls (e.g. 2000), sequential calls all succeed, but most parallel calls fail. The replica imposes a limit on the number of in-flight calls a canister can have to a given target. Sequential calls respect this naturally (one in-flight call at a time), while 2000 parallel calls immediately exceed it.

> **Note on retries:** If the in-flight limit is hit, immediate retries will also fail. Retries should be scheduled via a timer or a heartbeat instead.

## Multi-subnet benefit

Parallel calls are most valuable across subnets, where cross-subnet latency (~2 seconds per message) makes sequential calls prohibitively slow. The `multi_subnet/` directory contains a PocketIC-based Rust test that installs `caller` and `callee` on different subnets and demonstrates the speedup:

```
Sequential calls: 90/90 successful in ~600ms
Parallel calls:   90/90 successful in ~300ms
```

The difference on ICP mainnet would be larger still.

To run the multi-subnet test locally (requires the Rust toolchain):

```bash
make test-multi-subnet
```

This builds the Motoko WASMs with `icp build` and then runs `cargo run` in `multi_subnet/` using those WASMs via the `CALLER_WASM` and `CALLEE_WASM` environment variables.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`
- Rust toolchain (only for `make test-multi-subnet`): [rustup.rs](https://rustup.rs)

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/parallel_calls
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
