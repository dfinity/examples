# Candid Type Generation

This example demonstrates automatic generation of Rust types from Candid interface definitions, eliminating the need to
manually copy and maintain type definitions from external canisters.

## Build Process Overview

The type generation happens automatically during the Rust build process through a `build.rs` script that runs before
compilation.

### build.rs

The build script performs three key functions:

1. **Validates Candid Files**: Checks that required `.did` files exist in the `candid/` directory
2. **Sets Environment Variables**: Configures canister IDs and Candid file paths for the bindgen tool
3. **Generates Types**: Uses `ic-cdk-bindgen` to create Rust type definitions

```rust
// Sets up the NNS Governance canister for type generation
let mut nns_governance = Config::new("nns_governance");
nns_governance.binding.set_type_attributes(
"#[derive(CandidType, Deserialize, Debug, Clone, serde::Serialize)]".to_string(),
);
```

Generated types are automatically written to `src/declarations/` and can be imported like any other Rust module.

## Candid Fetching Process

### fetch_candid.sh

The fetch script retrieves live Candid interface definitions directly from deployed canisters:

1. **Connects to Mainnet**: Uses `dfx canister --network ic` to access deployed canisters
2. **Fetches Metadata**: Retrieves the `candid:service` metadata from the target canister
3. **Saves Interface**: Writes the Candid definition to `candid/nns_governance.did`

```bash
# Fetch the live Candid interface from NNS Governance
dfx canister --network ic metadata "rrkah-fqaaa-aaaaa-aaaaq-cai" candid:service > candid/nns_governance.did
```

This ensures your types stay synchronized with the actual deployed canister interface.

## Usage Workflow

1. **Fetch Candid Definitions**:
   ```bash
   ./scripts/fetch_candid.sh
   ```

2. **Build Project**: The build script automatically generates types during compilation:
   ```bash
   cargo build
   ```

3. **Use Generated Types**: Import and use the generated types in your code:
   ```rust
   use declarations::nns_governance::{ListNeurons, ListNeuronsResponse};
   ```

## Advantages Over Manual Type Copying

### Type Generation Benefits

- **Always Current**: Types reflect the actual deployed canister interface
- **No Manual Maintenance**: Automatic updates when you re-fetch Candid files
- **Build-Time Validation**: Compilation fails if interfaces are incompatible
- **Reduced Errors**: Eliminates copy-paste mistakes and version mismatches

### Compared to Monorepo Copying

Manual copying from the IC monorepo has several disadvantages:

- **Manual Synchronization**: Requires manually tracking updates and changes
- **Incomplete Types**: Easy to miss dependent types or recent additions
- **Maintenance Overhead**: Regular manual updates needed to stay current

### Type Safety Guarantees

Generated types provide compile-time guarantees that your code matches the actual canister interface, preventing runtime
errors from interface mismatches.

## Dependencies

The project uses `ic-cdk-bindgen` as a build dependency to handle the type generation:

```toml
[build-dependencies]
ic-cdk-bindgen = "0.1.3"
```

This tool automatically generates idiomatic Rust types with appropriate derives and serialization support.

## Possible enhancements

Run a bot to update the candid files from mainnet on a regular cadence, and merge these changes in if they compile.

Note that you want to have the candid file checked in to ensure build reproducibility between versions, as an updated
candid file on an old version would result in different rust and then also different bytes, which would make it
impossible to reproduce historical builds. That could create problems for any sort of 3rd party verification.

