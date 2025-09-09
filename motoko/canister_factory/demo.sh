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

# 6. Upgrade the first canister
dfx canister call backend upgradeActorClass "(principal \"$CANISTER1\")"
echo "Upgraded canister: $CANISTER1"

# 7. Reinstall the third canister
dfx canister call backend reinstallActorClass "(principal \"$CANISTER3\")"
echo "Reinstalled canister: $CANISTER3"