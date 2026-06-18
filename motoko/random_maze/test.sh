#!/usr/bin/env bash
set -e

echo "=== Test 1: generate returns a maze string with wall characters ==="
result=$(icp canister call backend generate '(8)') && \
  echo "$result" && \
  echo "$result" | grep -q '🟥' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: generate with size 1 returns a minimal maze ==="
result=$(icp canister call backend generate '(1)') && \
  echo "$result" && \
  echo "$result" | grep -q '🟥' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: generate with size 16 returns a larger maze ==="
result=$(icp canister call backend generate '(16)') && \
  echo "$result" && \
  echo "$result" | grep -q '🟥' && \
  echo "PASS" || (echo "FAIL" && exit 1)
