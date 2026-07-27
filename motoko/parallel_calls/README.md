# Parallel inter-canister calls

This example demonstrates parallel inter-canister calls in Motoko and highlights the key differences between sequential and parallel call patterns. Running independent calls in parallel can significantly reduce latency, especially when messages are sent across subnets. For example, a canister that swaps two tokens might want to launch both token transfers in parallel.

Two canisters are deployed:

- **`caller`** — two endpoints: `sequential_calls(n)` (issues `n` calls one at a time) and `parallel_calls(n)` (issues all `n` calls at once). It reaches the callee through the typed import `import Callee "canister:callee"` — no actor type is declared in the code (see [How the callee is resolved](#how-the-callee-is-resolved)).
- **`callee`** — a minimal `ping` endpoint that takes no parameters and returns nothing.

## How the callee is resolved

The caller talks to the callee through `import Callee "canister:callee"`. The `--actor-env-alias callee PUBLIC_CANISTER_ID:callee callee/callee.did` flag in `mops.toml` tells the Motoko compiler to type that import against `callee/callee.did` at build time, and to resolve the callee's **principal at runtime** from the canister environment variable `PUBLIC_CANISTER_ID:callee`. Because the interface comes from the `.did`, no actor type is written in `caller/app.mo`.

The consequence is that the caller's Wasm contains **no callee principal** — the compiler embeds only the environment-variable *name*, so **the same Wasm artifact runs unchanged in every environment**:

- **Build time (`mops build` / `icp build`):** `moc` compiles the `canister:callee` import into calls to the IC environment-variable system API, typed against the `.did`. No principal and no target environment are known to the build.
- **Deploy time (`icp deploy`):** icp-cli sets the environment variable `PUBLIC_CANISTER_ID:callee` on the caller canister to the callee's principal. It is auto-injected for canisters defined in the project, and can be overridden per environment in `icp.yaml` (`environments[].settings.caller.environment_variables`) — for example to point the caller at a callee that already exists on mainnet.
- **Runtime:** the canister reads `PUBLIC_CANISTER_ID:callee` and binds the `Callee` reference when it is installed or upgraded, then reuses that principal for every call. Changing the variable therefore takes effect on the next deploy/upgrade.

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
bash test-multi-subnet.sh
```

This builds the Motoko WASMs with `icp build` and then runs `cargo run` in `multi_subnet/` using those WASMs via the `CALLER_WASM` and `CALLEE_WASM` environment variables.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`
- Rust toolchain (only for `bash test-multi-subnet.sh`): [rustup.rs](https://rustup.rs)

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/parallel_calls
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## Updating the Candid interface

`callee/callee.did` is the interface the caller's `canister:callee` import is typed against (see [How the callee is resolved](#how-the-callee-is-resolved)). If you change the callee's public API, regenerate it so the caller keeps compiling against the correct interface:

```bash
mops generate candid callee
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
