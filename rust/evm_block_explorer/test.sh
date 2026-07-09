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
