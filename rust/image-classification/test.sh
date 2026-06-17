#!/usr/bin/env bash
set -e

echo "=== Test 1: run() classifies the built-in test image ==="
result=$(icp canister call --query backend run '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'tractor' && \
  echo "PASS" || (echo "FAIL" && exit 1)
