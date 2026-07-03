# Inter-canister calls

This example demonstrates how to make inter-canister calls in Rust on the Internet Computer. It shows both bounded-wait and unbounded-wait call styles, retry logic, and how to send cycles to another canister via the management canister. Two canisters are deployed: a `counter` canister that exposes a simple counter interface, and a `caller` canister that calls the counter using different inter-canister call patterns.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/inter-canister-calls
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
