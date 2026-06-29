# Candid Type Generation

This example shows how to automatically generate Rust types from a Candid interface definition (`.did` file), eliminating the need to manually copy and maintain type definitions from external canisters.

The example deploys a canister that calls the [NNS Governance](https://dashboard.internetcomputer.org/canister/rrkah-fqaaa-aaaaa-aaaaq-cai) canister on the IC mainnet using types generated directly from its live Candid interface.

## How it works

```
candid/nns_governance.did          ← Candid interface fetched from the live canister
        ↓  (build.rs)
$OUT_DIR/nns_governance.rs         ← Rust types generated at build time
        ↓  (include! in declarations/mod.rs)
declarations::nns_governance::*    ← Types available in your canister code
```

### build.rs

The build script uses [`ic-cdk-bindgen`](https://crates.io/crates/ic-cdk-bindgen) to generate Rust types from the `.did` file before the main compilation starts:

```rust
Config::new("nns_governance", "candid/nns_governance.did")
    .static_callee(Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap())
    .set_type_selector_config("candid/nns_governance.toml")
    .generate();
```

- `Config::new(name, candid_path)` — `name` becomes the output filename (`nns_governance.rs`)
- `.static_callee(principal)` — embeds the target canister ID as a constant in the generated code. Use `.dynamic_callee("ENV_VAR_NAME")` when the canister ID varies across environments.
- `.set_type_selector_config(path)` — TOML file controlling which traits are derived on generated types
- `.generate()` — writes to `$OUT_DIR/nns_governance.rs` (outside of `src/`, not committed to git)

The `cargo:rerun-if-changed` directives ensure the build script only re-runs when the `.did` or `.toml` files change, keeping incremental rebuilds fast.

### Type Selector Configuration

`candid/nns_governance.toml` controls how Candid types are mapped to Rust:

```toml
[rust]
visibility = "pub"
attributes = "#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]"
```

By default, `ic-cdk-bindgen` derives `CandidType` and `Deserialize` on all types. The `attributes` field adds `Debug`, `Clone`, and `serde::Serialize` — necessary here because the example uses `serde_json::to_string_pretty` to display the response.

See the [Type Selector specification](https://github.com/dfinity/candid/blob/master/spec/Type-selector.md#rust-binding-configuration) for the full configuration syntax.

### Including the generated code

The generated file lives in Cargo's build output directory and is brought into scope via `include!`:

```rust
// backend/src/declarations/mod.rs
pub mod nns_governance {
    include!(concat!(env!("OUT_DIR"), "/nns_governance.rs"));
}
```

The `include!` macro pastes the file contents at compile time, exactly as if you had written the code directly. This is the standard pattern for `ic-cdk-bindgen` 0.2.0+. The types are then available in your canister:

```rust
use declarations::nns_governance::{ListNeurons, ListNeuronsResponse};
```

### Using the generated types

Generated code for a `.static_callee` canister provides top-level async functions and a `CANISTER_ID` constant:

```rust
// Generated in $OUT_DIR/nns_governance.rs
pub const CANISTER_ID: Principal = Principal::from_slice(&[/* ... */]);

pub async fn list_neurons(arg: &ListNeurons) -> CallResult<ListNeuronsResponse> {
    Ok(Call::bounded_wait(CANISTER_ID, "list_neurons").with_arg(arg).await?.candid()?)
}
// ... one function per canister method
```

Calling a method is a single typed function call — no raw `Principal` arguments or string method names:

```rust
declarations::nns_governance::list_neurons(&request).await
```

## Fetching and updating Candid definitions

`candid/nns_governance.did` is already checked in and ready to use. To refresh it from the live canister:

```bash
bash scripts/fetch_candid.sh
```

This runs:
```bash
icp canister metadata --network ic rrkah-fqaaa-aaaaa-aaaaq-cai candid:service \
  > candid/nns_governance.did
```

After refreshing, rebuild to regenerate the Rust types:
```bash
icp build backend
```

If the Candid interface changed in a breaking way (a method removed, a field type changed), you will get a **compile error** — not a runtime failure. This is the key safety property of the approach: interface mismatches are caught before the canister is ever deployed.

**Reproducibility note**: Always commit the `.did` file. The generated Rust (`$OUT_DIR/nns_governance.rs`) is deterministically derived from it at build time, so committing the `.did` is sufficient to reproduce any historical build exactly. Regenerating from mainnet at an arbitrary time would produce different types if the canister interface had changed, making it impossible to reproduce historical WASM bytes — which matters for any third-party verification.

**Possible enhancement**: Run a bot to fetch the latest `.did` file from mainnet on a regular cadence and open a PR if the project still compiles. This keeps types current with zero manual effort.

## Advantages over manual type copying

|  | Manual copying | `ic-cdk-bindgen` |
|---|---|---|
| Keeping types current | Track upstream changes manually | Re-run `fetch_candid.sh` |
| Missing dependent types | Easy to overlook | Full interface generated |
| Interface mismatches | Runtime errors | Build-time compile errors |
| Maintenance | Ongoing manual work | Minimal |

## Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [ic-mops](https://mops.one/docs/install): `npm install -g ic-mops`

## Deploy and test

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/candid_type_generation
icp network start -d
icp deploy
bash test.sh
icp network stop
```

> **Note**: On a local replica, the inter-canister call to NNS Governance on the IC mainnet will not succeed. The test verifies the canister compiled and deployed correctly by checking that it returns either a successful JSON response or an expected error. To test with live neuron data, deploy to the IC mainnet:
> ```bash
> icp deploy --network ic
> icp canister call --network ic backend list_neurons_pretty '()'
> ```

## Security considerations and best practices

For information about security best practices when developing ICP canisters, see
https://docs.internetcomputer.org/guides/security/overview
