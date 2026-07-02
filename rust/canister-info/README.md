# Canister Info

This example demonstrates how to use the IC's [`canister_info`](https://docs.internetcomputer.org/references/management-canister/#canister_info) management call to retrieve information about any canister, including its recent change history (the IC retains up to 20 changes per canister).

Two canisters are deployed:

- **`backend`** — provides five update endpoints for inspecting any canister by principal: `info`, `reflexive_transitive_controllers`, `canister_controllers`, `canister_module_hash`, and `canister_deployment_chain`. See the source code for detailed doc comments on each method.
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
test_id=$(icp canister status test -i)
icp canister call backend info "(principal \"$test_id\")"
```

## Canister history limit

The IC retains only the **most recent 20 changes** per canister. The `info` function requests up to 20 changes via `num_requested_changes: Some(20)`. Once a canister exceeds 20 changes, older entries are dropped permanently.

If you need a full audit trail for a frequently-changing canister, you must poll `info` regularly and persist the history yourself before entries fall off. See the [canister history documentation](https://docs.internetcomputer.org/guides/canister-management/lifecycle/#canister-history) for details.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. This example may not implement all the best practices.
