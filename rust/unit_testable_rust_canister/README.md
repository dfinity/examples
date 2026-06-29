# Unit Testable Rust Canister

This example demonstrates how to structure a Rust canister for comprehensive unit testing by isolating
non-deterministic dependencies behind interfaces. It uses dependency injection so that inter-canister
calls and stable memory operations can all be mocked in fast pure-Rust unit tests.

## Architecture

### Dependency Injection Structure

The canister uses a dependency injection pattern that avoids complex generics throughout the codebase:

```rust
pub struct CanisterApi {
    governance: Arc<dyn GovernanceApi>,
    counter: Arc<dyn Counter>,
}
```

**Benefit**: Functions that use `CanisterApi` don't need nested generics. Instead of:

```rust
fn complex_function<G, S, C>(api: &C) -> Result<T, E>
where
    C: CanisterApi<G, S>,
    G: GovernanceApiTrait,
    S: StorageApiTrait,
{
    // implementation
}
```

You can write simple functions:

```rust
fn complex_function(api: &CanisterApi) -> Result<T, E> {
    // implementation
}
```

### Interface-Based Design

Non-deterministic operations are abstracted behind traits:

- **Inter-canister calls** → `GovernanceApi`
- **Stable memory operations** → `Counter` (backed by `StableMemoryCounter`)

**Benefit**: The entire dependency tree can be mocked, allowing you to test all canister logic in pure Rust unit tests
without any IC integration.

Technically stable memory can be fully tested in Rust, but in cases where more complex logic is needed to update the
contents of stable memory in a way that works for tests, you can simplify your testing by putting it behind an
interface that abstracts away the actual storage implementation. This makes it easier to evolve your storage layer
without needing to update tests.

## Testing Strategy

### Unit Tests (Fast & Comprehensive)

Unit tests run in milliseconds and can test complex business logic by mocking all external dependencies:

```rust
#[test]
fn test_counter_endpoints() {
    let governance = Arc::new(MockGovernanceApi::new());
    let counter = Arc::new(TestCounter::new());
    let api = CanisterApi::new(governance, counter);

    let response = api.get_count();
    assert_eq!(response.count, Some(0));

    let response = api.increment_count();
    assert_eq!(response.new_count, Some(1));
}
```

### Integration Tests (Slower, End-to-End)

Integration tests use PocketIC to verify the complete system works together, including actual
inter-canister calls to a locally deployed NNS Governance canister:

```rust
#[test]
fn test_counter_functionality() {
    let pic = PocketIcBuilder::new().with_nns_subnet().build();
    let canister_id = deploy_backend_canister(&pic);

    let response: GetCountResponse = query(&pic, canister_id, "get_count", encode_one(GetCountRequest {}).unwrap());
    assert_eq!(response.count, Some(0));
}
```

## Performance Comparison

- **Unit tests**: ~1ms per test, run in parallel
- **Integration tests**: ~1-5 seconds per test, require canister deployment and inter-canister call setup

The architecture enables testing 95%+ of your logic in fast unit tests, with only a few integration tests needed to
verify system integration.

## Keeping Up With Mainnet Canister Changes

In the PocketIC integration tests, we rely on setting up Governance proposals via init arguments. That capability
could be removed in the future, as it's not part of the stable interface of the canister. In that case, mocking out
canisters would become harder, as you would need to also create a ledger and neurons and proposals. This setup can
be error-prone, and would need to be kept in sync with mainnet.

In the future, PocketIC may contain working versions of system canisters as a setup option which would alleviate this
difficulty.

Speed will always, however, remain a large advantage of leaning more heavily on unit tests, using integration tests for
minimal testing.

## Project Structure

```
backend/
├── Cargo.toml
├── backend.did           # Candid interface
├── src/
│   ├── lib.rs            # Canister entry points and initialization
│   ├── canister_api.rs   # Main API struct and dependency injection
│   ├── counter.rs        # Counter trait and implementation (abstraction over storage)
│   ├── governance.rs     # NNS Governance trait and implementations
│   ├── stable_memory.rs  # Storage operations and trait definitions
│   └── types/
│       ├── mod.rs            # Request/response types
│       └── nns_governance.rs # NNS Governance canister type definitions
└── tests/
    └── integration_tests.rs  # Slower end-to-end tests using PocketIC
```

## Type Generation

This example includes generated types for external canisters (NNS Governance).
For automatic type generation from Candid files, see the [candid_type_generation](../../rust/candid_type_generation) example which demonstrates:

- Generating Rust types from `.did` files
- Maintaining type compatibility across canister updates
- Integration with existing codebases

## Key Benefits of this canister structure

1. **Simple Function Signatures**: No complex generic constraints in business logic
2. **Comprehensive Testability**: Mock entire dependency tree for isolated testing
3. **Fast Development Cycle**: Most logic tested in milliseconds, not seconds
4. **Easy Debugging**: Unit tests can isolate specific scenarios without IC complexity
5. **Maintainable Code**: Clear separation between business logic and IC integration

## Build and deploy from the command line

### Prerequisites
- [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- Rust toolchain with `wasm32-unknown-unknown` target

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/unit_testable_rust_canister
```

### Run unit tests

```bash
# Fast unit tests (recommended for development)
cargo test --lib
```

### Run PocketIC integration tests

Integration tests use PocketIC and require the [PocketIC server](https://github.com/dfinity/pocketic/releases). Install it first:

```bash
# macOS (Apple Silicon)
curl -sL https://github.com/dfinity/pocketic/releases/download/15.0.0/pocket-ic-arm64-darwin.gz | gunzip > pocket-ic-server
# macOS (Intel)
curl -sL https://github.com/dfinity/pocketic/releases/download/15.0.0/pocket-ic-x86_64-darwin.gz | gunzip > pocket-ic-server
# Linux (x86_64)
curl -sL https://github.com/dfinity/pocketic/releases/download/15.0.0/pocket-ic-x86_64-linux.gz | gunzip > pocket-ic-server
chmod +x pocket-ic-server
export POCKET_IC_BIN=$(pwd)/pocket-ic-server
```

Then:

```bash
# Build WASM first (required by integration tests)
cargo build --target wasm32-unknown-unknown --release

# Run integration tests
cargo test --test integration_tests

# Or run everything
cargo test
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## Security considerations and best practices

For information on security best practices when developing on ICP, see the [security overview](https://docs.internetcomputer.org/guides/security/overview).
