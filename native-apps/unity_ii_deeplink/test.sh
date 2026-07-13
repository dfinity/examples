#!/usr/bin/env bash
set -e

echo "=== Test 1: greet returns a greeting for the anonymous principal ==="
result=$(icp canister call --query backend greet '()')
echo "$result"
echo "$result" | grep -q "Hello, 2vxsx-fae!" && echo "PASS" || (echo "FAIL: expected 'Hello, 2vxsx-fae!'" && exit 1)
