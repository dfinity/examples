#!/usr/bin/env bash
set -e

echo "=== Test 1: send_http_post_request returns JSON with POST body ==="
result=$(icp canister call backend send_http_post_request '()')
echo "$result"
echo "$result" | grep -q "POST request from an ICP canister" && echo "PASS" || (echo "FAIL" && exit 1)
