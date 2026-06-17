#!/usr/bin/env bash
set -e

echo "=== Test 1/1: greet() returns a greeting ==="
result=$(icp canister call backend greet '("World")') && \
  echo "$result" && \
  echo "$result" | grep -q 'Hello, World!' && \
  echo "PASS" || (echo "FAIL" && exit 1)
