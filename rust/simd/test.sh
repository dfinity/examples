#!/usr/bin/env bash
set -e

echo "=== Test 1: naive_f32() returns a non-zero instruction count ==="
result=$(icp canister call --query backend naive_f32 '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: optimized_f32() returns a non-zero instruction count ==="
result=$(icp canister call --query backend optimized_f32 '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: auto_vectorized_f32() returns a non-zero instruction count ==="
result=$(icp canister call --query backend auto_vectorized_f32 '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: simd_f32() returns a non-zero instruction count ==="
result=$(icp canister call --query backend simd_f32 '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: naive_u32() returns a non-zero instruction count ==="
result=$(icp canister call --query backend naive_u32 '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6: optimized_u32() returns a non-zero instruction count ==="
result=$(icp canister call --query backend optimized_u32 '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 7: auto_vectorized_u32() returns a non-zero instruction count ==="
result=$(icp canister call --query backend auto_vectorized_u32 '()') && \
  echo "$result" && \
  echo "$result" | grep -qv '(0 : nat64)' && \
  echo "PASS" || (echo "FAIL" && exit 1)
