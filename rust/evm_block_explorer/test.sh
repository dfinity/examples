#!/usr/bin/env bash
set -e

echo "=== Test 1: get_evm_block returns correct data for Ethereum mainnet block 1 ==="
result=$(icp canister call backend get_evm_block '(1)')
echo "$result"
echo "$result" | grep -q "Ok" && echo "PASS (Ok variant)" || (echo "FAIL (expected Ok)" && exit 1)
echo "$result" | grep -q "0x88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6" && echo "PASS (hash)" || (echo "FAIL (wrong hash)" && exit 1)
echo "$result" | grep -q "0x05a56e2d52c817161883f50c441c3228cfe54d9f" && echo "PASS (miner)" || (echo "FAIL (wrong miner)" && exit 1)
