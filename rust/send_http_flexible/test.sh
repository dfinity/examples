#!/usr/bin/env bash
set -e

echo "=== Test 1: fetch_server_time collects responses from the committee ==="
result=$(icp canister call backend fetch_server_time '()')
echo "$result"
echo "$result" | grep -q "Ok" && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: the reconciled tally carries a date from the server ==="
echo "$result" | grep -qE "GMT" && echo "PASS" || (echo "FAIL" && exit 1)
