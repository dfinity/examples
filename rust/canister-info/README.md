# Canister Info

This example demonstrates how to use the IC's [`canister_info`](https://docs.internetcomputer.org/concepts/system-management-canister/#canister-info) management call to retrieve information about any canister, including its full change history.

Two canisters are deployed:

- **`backend`** — provides five query/update endpoints for inspecting any canister by principal: `info`, `reflexive_transitive_controllers`, `canister_controllers`, `canister_module_hash`, and `canister_deployment_chain`. See the source code for detailed doc comments on each method.
- **`test`** — a minimal pre-built canister with no logic, deployed purely to provide a subject for the backend's canister info queries in `test.sh`.

The `backend` passes the target canister's principal ID as an argument to the management canister's `canister_info` call — it does not call the `test` canister directly. This means the backend works with the ID of any deployed canister, not just `test`.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/canister-info
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

After deploying, you can inspect any canister by passing its principal to the backend. For example, to inspect the `test` canister itself:

```bash
test_id=$(icp canister id test)
icp canister call backend info "(principal \"$test_id\")"
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. This example may not implement all the best practices.
