#!/usr/bin/env bash
set -e

echo "=== Test 1: get_ecdsa_public_key returns a hex-encoded public key ==="
result=$(icp canister call backend get_ecdsa_public_key '()')
echo "$result"
echo "$result" | grep -qE '[0-9a-f]{60}' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: get_schnorr_public_key returns a hex-encoded public key ==="
result=$(icp canister call backend get_schnorr_public_key '()')
echo "$result"
echo "$result" | grep -qE '[0-9a-f]{60}' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: sign_message_with_ecdsa returns a hex-encoded signature ==="
result=$(icp canister call backend sign_message_with_ecdsa '("hello")')
echo "$result"
echo "$result" | grep -qE '[0-9a-f]{60}' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: sign_message_with_schnorr returns a hex-encoded signature ==="
result=$(icp canister call backend sign_message_with_schnorr '("hello")')
echo "$result"
echo "$result" | grep -qE '[0-9a-f]{60}' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: get_evm_block returns correct data for Ethereum mainnet block 1 ==="
result=$(icp canister call backend get_evm_block '(1)')
echo "$result"
echo "$result" | grep -q "Ok" && echo "PASS (Ok variant)" || (echo "FAIL (expected Ok)" && exit 1)
echo "$result" | grep -q "0x88e96d4537bea4d9c05d12549907b32561d3bf31f45aae734cdc119f13406cb6" && echo "PASS (hash)" || (echo "FAIL (wrong hash)" && exit 1)
echo "$result" | grep -q "0x05a56e2d52c817161883f50c441c3228cfe54d9f" && echo "PASS (miner)" || (echo "FAIL (wrong miner)" && exit 1)
