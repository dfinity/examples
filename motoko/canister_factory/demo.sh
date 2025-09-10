#!/bin/bash

echo "=== Motoko Canister Factory: Complete Demo ==="
echo "This demo showcases:"
echo "1. Different canister creation approaches"
echo "2. Upgrade vs Reinstall behavior with state persistence"
echo ""

echo "=== Part 1: Different Canister Creation Approaches ==="

# 1. Start local replica
echo "1. Starting local replica..."
dfx start --background --clean

# 2. Deploy the main canister
echo "2. Deploying main canister..."
dfx deploy --with-cycles 30000000000000

# 3. Create a canister using actor class (high-level)
echo "3. Creating canister via actor class (high-level)..."
CANISTER1=$(dfx canister call backend newActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created canister via actor class: $CANISTER1"

# 4. Create a canister using manual management (low-level)
echo "4. Creating canister manually (low-level)..."
CANISTER2=$(dfx canister call backend createAndInstallCanisterManually '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created canister manually: $CANISTER2"

# 5. Create canister with two-step process
echo "5. Creating canister with two-step process..."
CANISTER3=$(dfx canister call backend installActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created canister with install process: $CANISTER3"

echo ""
echo "=== Part 2: Upgrade vs Reinstall Demonstration ==="

# 6. Create additional canisters for testing upgrade vs reinstall
echo "6. Creating test canisters for upgrade/reinstall demo..."
CANISTER_UPGRADE=$(dfx canister call backend newActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
CANISTER_REINSTALL=$(dfx canister call backend newActorClass '(2_000_000_000_000)' | grep -o 'principal "[^"]*"' | cut -d'"' -f2)
echo "Created upgrade test canister: $CANISTER_UPGRADE"
echo "Created reinstall test canister: $CANISTER_REINSTALL"

# 7. Test initial state of both canisters
echo ""
echo "=== Initial State ==="
echo "Upgrade canister initial value:"
dfx canister call $CANISTER_UPGRADE getValue
echo "Reinstall canister initial value:"
dfx canister call $CANISTER_REINSTALL getValue

# 8. Modify state by incrementing internal counters
echo ""
echo "=== Modifying State ==="
echo "Adding 10 to upgrade canister..."
dfx canister call $CANISTER_UPGRADE addToValue '(10)'
echo "Adding 20 to reinstall canister..."
dfx canister call $CANISTER_REINSTALL addToValue '(20)'

# 9. Upgrade the first canister (preserves state)
echo ""
echo "=== Performing Upgrade (State Preserved) ==="
dfx canister call backend upgradeActorClass "(principal \"$CANISTER_UPGRADE\")"
echo "Upgraded canister: $CANISTER_UPGRADE"

# 10. Test upgraded canister - should have preserved state AND new functionality
echo ""
echo "Upgraded canister value (should be preserved):"
dfx canister call $CANISTER_UPGRADE getValue
echo "Testing new substractFromValue endpoint:"
dfx canister call $CANISTER_UPGRADE substractFromValue '(5)'

# 11. Reinstall the second canister (resets state)
echo ""
echo "=== Performing Reinstall (State Reset) ==="
dfx canister call backend reinstallActorClass "(principal \"$CANISTER_REINSTALL\")"
echo "Reinstalled canister: $CANISTER_REINSTALL"

# 12. Test reinstalled canister - should have reset state BUT new functionality
echo ""
echo "Reinstalled canister value (should be reset to initial):"
dfx canister call $CANISTER_REINSTALL getValue
echo "Adding 20 to reinstall canister..."
dfx canister call $CANISTER_REINSTALL addToValue '(20)'
echo "Testing new substractFromValue endpoint:"
dfx canister call $CANISTER_REINSTALL substractFromValue '(5)'

echo ""
echo "=== Summary ==="
echo "✅ Different creation approaches demonstrated:"
echo "   - Actor class (high-level): $CANISTER1"
echo "   - Manual creation (low-level): $CANISTER2"
echo "   - Two-step process: $CANISTER3"
echo ""
echo "🔄 Upgrade: State preserved, new functionality added"
echo "🔥 Reinstall: State reset, new functionality added"
echo ""
echo "Demo completed! All approaches and behaviors demonstrated."