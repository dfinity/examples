# Unit Testable Rust Canister

This repository demonstrates how to structure a Rust canister for comprehensive unit testing by isolating
non-deterministic dependencies behind interfaces.

## Architecture

### Dependency Injection Structure

The canister uses a dependency injection pattern that avoids complex generics throughout the codebase:

```rust
pub struct CanisterApi {
    pub governance: Box<dyn GovernanceApiTrait>,
    pub storage: Box<dyn StorageApiTrait>,
    // other dependencies...
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

- **Inter-canister calls** → `GovernanceApiTrait`
- **Stable memory operations** → `StorageApiTrait`
- **Time-based operations** → `TimeApiTrait`

**Benefit**: The entire dependency tree can be mocked, allowing you to test all canister logic in pure Rust unit tests
without any IC integration.

Technically Stable Memory can be fully test in Rust, but in cases where more complex logic is needed to update the
contents
of stable memory in a way that works for tests, you can simplify your testing by putting it behind an interface that
abstracts away the actual storage implementation. This makes it easier to evolve your storage layer without
needing to update tests.

## Testing Strategy

### Unit Tests (Fast & Comprehensive)

Unit tests run in milliseconds and can test complex business logic by mocking all external dependencies:

```rust
#[test]
fn test_complex_governance_logic() {
    let mut mock_governance = MockGovernanceApi::new();
    mock_governance.expect_get_proposal_info()
        .returning(|_| Ok(mock_proposal()));

    let api = CanisterApi::new_with_mocks(mock_governance, /* other mocks */);

    // Test complex logic without any IC integration
    let result = complex_function(&api);
    assert_eq!(result, expected_result);
}
```

### Integration Tests (Slower, End-to-End)

Integration tests use PocketIC to verify the complete system works together:

```rust
#[test]
fn test_end_to_end_workflow() {
    let pic = PocketIc::new();
    let canister_id = deploy_canister(&pic);

    // Test actual inter-canister calls
    let response = pic.update_call(canister_id, "method", args);
    // assertions...
}
```

## Performance Comparison

- **Unit tests**: ~1ms per test, run in parallel
- **Integration tests**: ~1-5 seconds per test, require canister deployment and inter-canister call setup

The architecture enables testing 95%+ of your logic in fast unit tests, with only a few integration tests needed to
verify system integration.

## Project Structure

```
src/
├── lib.rs              # Canister entry points and initialization
├── canister_api.rs     # Main API struct and dependency injection
├── counter.rs          # Counter trait and implementation (abstraction over storage)
├── governance.rs       # NNS Governance trait and implementations
├── stable_memory.rs    # Storage operations and trait definitions
├── types/
│   ├── mod.rs         # Request/response types and external canister types
│   └── nns_governance.rs  # NNS Governance canister type definitions, generated from governance candid.
└── tests/
    └── integration_tests.rs # Slower end-to-end tests
```

## Type Generation

This example includes generated types for external canisters (NNS Governance).
For automatic type generation from Candid files, see the `candid-type-generation` branch which demonstrates:

- Generating Rust types from `.did` files
- Maintaining type compatibility across canister updates
- Integration with existing codebases

## Key Benefits of this canister structure

1. **Simple Function Signatures**: No complex generic constraints in business logic
2. **Comprehensive Testability**: Mock entire dependency tree for isolated testing
3. **Fast Development Cycle**: Most logic tested in milliseconds, not seconds
4. **Easy Debugging**: Unit tests can isolate specific scenarios without IC complexity
5. **Maintainable Code**: Clear separation between business logic and IC integration

## Running Tests

```bash
# Fast unit tests (recommended for development)
cargo test --lib

# All tests (including integration tests)
cargo test
```

The unit tests demonstrate testing the same functionality as integration tests but with significantly better performance
and easier setup.
