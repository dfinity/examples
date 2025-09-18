# Motoko Canister Creation Examples

This project demonstrates various approaches to creating and managing canisters on the Internet Computer using Motoko. It showcases the differences between high-level actor class management and low-level management canister operations.

## Overview

The example includes implementations of:

- **Actor Class Management**: High-level canister creation using the `system` keyword
- **Manual Canister Management**: Low-level creation using the management canister directly
- **Canister Lifecycle Operations**: Upgrade and reinstall operations

## Key Differences

### Actor Class Management (High-level)
- Simpler API with automatic WASM installation
- Limited canister settings: `controllers`, `compute_allocation`, `memory_allocation`, `freezing_threshold`
- Good for most common use cases
- [Documentation](https://internetcomputer.org/docs/motoko/language-manual#actor-class-management)

### Management Canister (Low-level)
- Full control over canister creation and settings
- Access to all settings: `reserved_cycles_limit`, `wasm_memory_limit`, `log_visibility`, `wasm_memory_threshold`
- Requires separate steps for creation and code installation
- [Documentation](https://internetcomputer.org/docs/references/ic-interface-spec#ic-create_canister)

## Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install) 0.29.0 or later
- [Mops](https://mops.one/) package manager

## Project Structure

```
├── src/
│   └── backend/
│       ├── Main.mo           # Main actor with canister creation examples
│       ├── Child.mo          # Simple actor class for demonstrations
│       └── AnotherChild.mo   # Alternative actor class for upgrades
├── dfx.json                  # DFX configuration
├── mops.toml                 # Mops package configuration
└── README.md                 # This file
```

## Getting Started

### 1. Start the local Internet Computer

```bash
dfx start --background
```

### 2. Deploy the canister

```bash
dfx deploy
```

This will deploy the main canister that contains all the canister creation examples.

## Available Functions

### 1. Actor Class Creation (High-level)

#### `newActorClass(cycles: Nat)`
Creates a new canister using actor class with automatic installation.

```bash
# Create a canister with 2 trillion cycles
dfx canister call backend newActorClass '(2_000_000_000_000)'
```

#### `installActorClass(cycles: Nat)`
Creates a canister and installs an actor class using a two-step process.

```bash
# Create and install actor class with 2 trillion cycles
dfx canister call backend installActorClass '(2_000_000_000_000)'
```

### 2. Canister Lifecycle Management

#### Understanding Upgrade vs Reinstall

This example demonstrates the critical difference between **upgrading** and **reinstalling** canisters:

**🔄 Upgrade (`#upgrade`)**
- **State is PRESERVED**: All mutable variables keep their current values
- **New functionality is added**: The `substractFromValue` endpoint becomes available
- **Existing functionality remains**: All original endpoints continue working
- **Use case**: Adding features, bug fixes, optimizations while keeping data

**🔥 Reinstall (`#reinstall`)**
- **State is RESET**: All mutable variables return to their initial values
- **New functionality is added**: The `substractFromValue` endpoint becomes available  
- **Existing functionality resets**: Original endpoints work but with fresh state
- **Use case**: Breaking changes, schema migrations, fresh start scenarios

#### `upgradeActorClass(canisterId: Principal)`
Upgrades an existing canister to use a different actor class (preserves state).

```bash
# Upgrade a canister (replace with actual canister ID)
dfx canister call backend upgradeActorClass '(principal "rdmx6-jaaaa-aaaaa-aaadq-cai")'
```

#### `reinstallActorClass(canisterId: Principal)`
Reinstalls an existing canister with a different actor class (destroys state).

```bash
# Reinstall a canister (replace with actual canister ID)
dfx canister call backend reinstallActorClass '(principal "rdmx6-jaaaa-aaaaa-aaadq-cai")'
```

### 3. Manual Canister Management (Low-level)

#### `createAndInstallCanisterManually(cycles: Nat)`
Creates a canister manually using the management canister with full control over settings.

```bash
# Create canister manually with advanced settings
dfx canister call backend createAndInstallCanisterManually '(2_000_000_000_000)'
```

## Example Workflow

Here's a complete example demonstrating all canister creation approaches and the differences between upgrade vs reinstall:

### Part 1: Different Canister Creation Approaches

```bash
# 1. Start local replica
dfx start --background --clean

# 2. Deploy the main canister
dfx deploy --with-cycles 30000000000000

# 3. Create a canister using actor class (high-level)
CANISTER1=$(dfx canister call backend newActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created canister via actor class: $CANISTER1"

# 4. Create a canister using manual management (low-level)
CANISTER2=$(dfx canister call backend createAndInstallCanisterManually '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created canister manually: $CANISTER2"

# 5. Create canister with two-step process
CANISTER3=$(dfx canister call backend installActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created canister with install process: $CANISTER3"
```

### Part 2: Upgrade vs Reinstall Demonstration

```bash
# 6. Create additional canisters for testing upgrade vs reinstall
CANISTER_UPGRADE=$(dfx canister call backend newActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
CANISTER_REINSTALL=$(dfx canister call backend newActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created upgrade test canister: $CANISTER_UPGRADE"
echo "Created reinstall test canister: $CANISTER_REINSTALL"

# 7. Test initial state of both canisters
echo "=== Initial State ==="
echo "Upgrade canister initial value:"
dfx canister call $CANISTER_UPGRADE getValue
echo "Reinstall canister initial value:"
dfx canister call $CANISTER_REINSTALL getValue

# 8. Modify state by incrementing internal counters
echo "=== Modifying State ==="
echo "Adding 10 to upgrade canister..."
dfx canister call $CANISTER_UPGRADE addToValue '(10)'
echo "Adding 20 to reinstall canister..."
dfx canister call $CANISTER_REINSTALL addToValue '(20)'

# 9. Upgrade the first canister (preserves state)
echo "=== Performing Upgrade (State Preserved) ==="
dfx canister call backend upgradeActorClass "(principal \"$CANISTER_UPGRADE\")"
echo "Upgraded canister: $CANISTER_UPGRADE"

# 10. Test upgraded canister - should have preserved state AND new functionality
echo "Upgraded canister value (should be preserved):"
dfx canister call $CANISTER_UPGRADE getValue
echo "Testing new substractFromValue endpoint:"
dfx canister call $CANISTER_UPGRADE substractFromValue '(5)'

# 11. Reinstall the second canister (resets state)
echo "=== Performing Reinstall (State Reset) ==="
dfx canister call backend reinstallActorClass "(principal \"$CANISTER_REINSTALL\")"
echo "Reinstalled canister: $CANISTER_REINSTALL"

# 12. Test reinstalled canister - should have reset state BUT new functionality
echo "Reinstalled canister value (should be reset to initial):"
dfx canister call $CANISTER_REINSTALL getValue
echo "Adding 20 to reinstall canister..."
dfx canister call $CANISTER_REINSTALL addToValue '(20)'
echo "Testing new substractFromValue endpoint:"
dfx canister call $CANISTER_REINSTALL substractFromValue '(5)'

echo "=== Summary ==="
echo "✅ Different creation approaches: actor class, manual, two-step"
echo "🔄 Upgrade: State preserved, new functionality added"
echo "🔥 Reinstall: State reset, new functionality added"
```

## Understanding the Code

### Actor Classes
- `Child.mo`: A simple persistent actor class with mutable state for demonstrating initial installations and state behavior
- `AnotherChild.mo`: An extended actor class with additional `substractFromValue` functionality used for upgrade/reinstall demonstrations

### State Behavior Demonstration
The example shows how:
- **Child**: Has basic functionality (`getValue`, `addToValue`) and mutable state
- **AnotherChild**: Extends Child with new functionality (`substractFromValue`) 
- **Upgrade**: Migrates from Child to AnotherChild while preserving any modified state
- **Reinstall**: Migrates from Child to AnotherChild but resets state to initial values

### Main Functions
- **High-level functions** use Motoko's `system` keyword with actor classes
- **Low-level functions** interact directly with the management canister
- **Lifecycle functions** demonstrate upgrade and reinstall capabilities

## Cycles Management

All functions require cycles to create canisters. The examples use 2 trillion cycles (2_000_000_000_000), which is sufficient for most development purposes. In production, you'll want to calculate appropriate cycle amounts based on your canister's needs.

## Troubleshooting

### Common Issues

1. **Insufficient cycles**: Increase the cycle amount in function calls
2. **Invalid canister ID**: Ensure you're using the correct Principal format
3. **Deploy failures**: Check that dfx is running and properly configured

### Getting Help

- [Internet Computer Documentation](https://internetcomputer.org/docs)
- [Motoko Documentation](https://internetcomputer.org/docs/motoko)
- [DFX Command Reference](https://internetcomputer.org/docs/building-apps/developer-tools/dfx/)
- [Developer Forum](https://forum.dfinity.org/)

## Related Examples

For more Motoko examples, visit the [official examples repository](https://github.com/dfinity/examples/tree/master/motoko).

## License

This project is licensed under the Apache 2.0 license. See LICENSE for more details.