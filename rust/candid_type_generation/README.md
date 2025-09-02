# Candid Type Generation from Remote Canisters

This example demonstrates how to dynamically generate and maintain Rust types from remote Internet Computer canisters by reading their Candid interfaces directly from their metadata. This approach is particularly valuable when building applications that depend on stable system canisters like NNS Governance.

## What This Example Illustrates

The primary goal is to show **how to keep types updated in your build process when relying on stable canisters**. Instead of manually defining types or copying them from external sources, this example:

- ✅ **Fetches the latest Candid interface** directly from deployed canisters
- ✅ **Automatically generates up-to-date Rust types** using the official tools
- ✅ **Integrates seamlessly** into your build workflow
- ✅ **Ensures type safety** with the actual deployed canister interfaces
- ✅ **Eliminates manual maintenance** of type definitions

## The Three Main Steps

### 1. **Fetch Candid Metadata from Remote Canister**
**Script: `./scripts/fetch_candid.sh`**

```bash
# Fetches the Candid interface directly from the deployed canister's metadata
dfx canister --network ic metadata rrkah-fqaaa-aaaaa-aaaaq-cai candid:service > nns_governance.did
```

**What this does:**
- Connects to the Internet Computer mainnet
- Reads the Candid interface embedded in the NNS Governance canister's WASM metadata
- Saves the complete interface definition to `nns_governance.did`
- Provides the authoritative source of truth for the canister's API

### 2. **Generate Rust Types from Candid File**  
**Script: `./scripts/generate_types.sh`**

```bash
# Generates Rust types using the official Candid tools
didc bind nns_governance.did --target rs > src/nns_governance_types.rs
```

**What this does:**
- Uses `didc` (the official Candid compiler) to parse the interface
- Generates complete Rust type definitions with proper `CandidType` and `Deserialize` derives
- Creates all necessary structs, enums, and method signatures
- Post-processes the output to add `pub` visibility, `Debug`, and `Serialize` traits

### 3. **Use Generated Types in Your Canister**
**Implementation: `src/lib.rs`**

```rust
// Import the generated types
use nns_governance_types::{ListNeurons, ListNeuronsResponse};

// Use them in inter-canister calls with full type safety
#[update]
async fn list_neurons_pretty() -> String {
    let request = ListNeurons {
        neuron_ids: vec![],
        include_neurons_readable_by_caller: true,
        // ... other fields with proper typing
    };
    
    let result: Result<(ListNeuronsResponse,), _> = call(
        governance_principal,
        "list_neurons", 
        (request,)
    ).await;
    // ...
}
```

**What this provides:**
- **Compile-time type checking** against the real canister interface
- **Auto-completion and IntelliSense** in your IDE
- **Automatic serialization/deserialization** via Candid
- **Pretty-printing capabilities** for complex responses

## Why This Approach Matters

### Traditional Problems:
❌ **Manual type maintenance** - Copy-pasting types and keeping them updated  
❌ **Version drift** - Your types become outdated as canisters evolve  
❌ **Runtime errors** - Interface mismatches discovered only at runtime  
❌ **Documentation lag** - Relying on external documentation that may be stale  

### This Solution:
✅ **Always current** - Types are generated from the actual deployed canister  
✅ **Build-time integration** - Automatically updates during your build process  
✅ **Type safety** - Compile-time guarantees that your calls will succeed  
✅ **Zero maintenance** - No manual type updates required  

## Running the Example

### Prerequisites
```bash
# Install required tools
cargo install didc
dfx --version  # Ensure dfx is installed
```

### Build and Deploy
```bash
# Automated build process (runs all 3 steps)
./build.sh

# Or run steps individually:
./scripts/fetch_candid.sh      # Step 1: Fetch Candid interface
./scripts/generate_types.sh    # Step 2: Generate Rust types  
dfx deploy                     # Step 3: Deploy and test
```

### Test the Canister
```bash
# Get information about the canister
dfx canister call candid_type_generation get_info

# Call NNS Governance using generated types (requires update call due to inter-canister calls)
dfx canister call candid_type_generation list_neurons_pretty
```

## Integration into Your Projects

### Makefile Integration
```makefile
.PHONY: update-types build

update-types:
	./scripts/fetch_candid.sh
	./scripts/generate_types.sh

build: update-types
	dfx build

deploy: build
	dfx deploy
```

### CI/CD Integration
```yaml
# GitHub Actions example
- name: Update Candid Types
  run: |
    ./scripts/fetch_candid.sh
    ./scripts/generate_types.sh
    
- name: Check for changes
  run: |
    if [[ `git status --porcelain` ]]; then
      echo "Types updated - committing changes"
      git add src/*_types.rs
      git commit -m "Auto-update: Generated types from canister metadata"
    fi
```

## Key Files

- **`scripts/fetch_candid.sh`** - Fetches Candid metadata from remote canisters
- **`scripts/generate_types.sh`** - Generates and post-processes Rust types  
- **`scripts/postprocess_types.sh`** - Adds public visibility and required derives
- **`build.sh`** - Complete automated build workflow
- **`src/nns_governance_types.rs`** - Generated types (auto-generated, don't edit)
- **`src/lib.rs`** - Example canister using the generated types

## Extending to Other Canisters

To generate types for other IC system canisters:

```bash
# ICP Ledger
dfx canister --network ic metadata ryjl3-tyaaa-aaaaa-aaaba-cai candid:service > icp_ledger.did

# SNS-WASM
dfx canister --network ic metadata qaa6y-5yaaa-aaaah-qcfzq-cai candid:service > sns_wasm.did

# Generate types for each
didc bind icp_ledger.did --target rs > src/icp_ledger_types.rs
didc bind sns_wasm.did --target rs > src/sns_wasm_types.rs
```

## Benefits for Production Applications

1. **Reliability** - Your application stays compatible as system canisters evolve
2. **Developer Experience** - Full IDE support with type checking and auto-completion  
3. **Maintainability** - Automated type updates reduce technical debt
4. **Correctness** - Compile-time guarantees prevent runtime interface errors
5. **Documentation** - Generated types serve as living documentation of the canister APIs

This pattern is essential for production applications that integrate with Internet Computer system canisters, ensuring long-term compatibility and maintainability.
