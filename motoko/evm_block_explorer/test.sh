#!/usr/bin/env bash
set -e

# Block 20000000 is post-Merge. PublicNode serves several regional node pools and some of
# them only retain history from shortly before the Merge (block ~15500000), so querying
# early blocks such as block 1 fails depending on which pool the outcall lands in.
BLOCK=20000000
BLOCK_HASH=0xd24fd73f794058a3807db926d8898c6481e902b7edb91ce0d479d6760f276183
BLOCK_MINER=0x95222290dd7278aa3ddd389cc1e1d165cc4bafe5

echo "=== Test 1: get_evm_block returns correct data for Ethereum mainnet block $BLOCK ==="
result=$(icp canister call backend get_evm_block "($BLOCK)")
echo "$result"
echo "$result" | grep -q "Ok" && echo "PASS (Ok variant)" || (echo "FAIL (expected Ok)" && exit 1)
echo "$result" | grep -q "$BLOCK_HASH" && echo "PASS (hash)" || (echo "FAIL (wrong hash)" && exit 1)
echo "$result" | grep -q "$BLOCK_MINER" && echo "PASS (miner)" || (echo "FAIL (wrong miner)" && exit 1)
