#!/usr/bin/env bash
set -e

echo "=== Test 1: send_http_get_request returns JSON with greeting ==="
result=$(icp canister call backend send_http_get_request '()')
echo "$result"
echo "$result" | grep -q "hello-from-icp" && echo "PASS" || (echo "FAIL" && exit 1)
