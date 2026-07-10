#!/usr/bin/env bash
set -e

echo "=== Test 1: greet returns expected greeting ==="
result=$(icp canister call --query backend greet '("World")')
echo "$result"
echo "$result" | grep -q "Hello, World!" && echo "PASS" || (echo "FAIL" && exit 1)
