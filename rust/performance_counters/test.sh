#!/usr/bin/env bash
set -e

echo "=== Test 1: for_update returns non-zero instruction counters ==="
result=$(icp canister call backend for_update '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64, 0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: for_composite_query returns non-zero instruction counters ==="
result=$(icp canister call --query backend for_composite_query '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64, 0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: example returns non-zero instruction counters ==="
result=$(icp canister call --query backend example '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64, 0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)
