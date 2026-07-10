#!/usr/bin/env bash
set -e

# These tests require Ollama to be running locally with the llama3.1:8b model.
# See README.md for setup instructions.

echo "=== Test 1: prompt returns a non-empty response ==="
result=$(icp canister call backend prompt '("What is 2 + 2?")')
echo "$result"
echo "$result" | grep -qE '"[^"]+"' && echo "PASS" || (echo "FAIL: expected a quoted string response" && exit 1)

echo "=== Test 2: chat returns a non-empty response ==="
result=$(icp canister call backend chat '(vec { variant { user = record { content = "Say hello" } } })')
echo "$result"
echo "$result" | grep -qE '"[^"]+"' && echo "PASS" || (echo "FAIL: expected a quoted string response" && exit 1)
