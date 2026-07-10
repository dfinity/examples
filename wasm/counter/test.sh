#!/usr/bin/env bash
set -e

echo "--- Initial value should be 0 ---"
result=$(icp canister call --query counter get '()')
echo "$result"
echo "$result" | grep -q '(0' && echo "PASS" || (echo "FAIL" && exit 1)

echo "--- Increment counter ---"
icp canister call counter inc '()'

echo "--- Value should be 1 after inc ---"
result=$(icp canister call --query counter get '()')
echo "$result"
echo "$result" | grep -q '(1' && echo "PASS" || (echo "FAIL" && exit 1)

echo "--- Set counter to 42 ---"
icp canister call counter set '(42 : int64)'

echo "--- Value should be 42 after set ---"
result=$(icp canister call --query counter get '()')
echo "$result"
echo "$result" | grep -q '(42' && echo "PASS" || (echo "FAIL" && exit 1)
