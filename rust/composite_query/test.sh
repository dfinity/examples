#!/usr/bin/env bash
set -e

echo "=== Test 1: put inserts a key-value pair (also creates Bucket partitions) ==="
# The return value is the previous value for key 1 (null on first run, a nat on subsequent runs).
result=$(icp canister call backend put '(1 : nat, 1337 : nat)') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: put a second value for the same key returns the old value ==="
result=$(icp canister call backend put '(1 : nat, 42 : nat)') && \
  echo "$result" && \
  echo "$result" | grep -q '1_337' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: get (composite query) retrieves the stored value ==="
result=$(icp canister call --query backend get '(1 : nat)') && \
  echo "$result" && \
  echo "$result" | grep -q 'opt (42' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: get_update returns the same value via an update call ==="
result=$(icp canister call backend get_update '(1 : nat)') && \
  echo "$result" && \
  echo "$result" | grep -q 'opt (42' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: get returns null for a missing key ==="
result=$(icp canister call --query backend get '(99 : nat)') && \
  echo "$result" && \
  echo "$result" | grep -q 'null' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6: composite query routes correctly across partitions ==="
icp canister call backend put '(2 : nat, 100 : nat)' && \
  icp canister call backend put '(3 : nat, 200 : nat)' && \
  icp canister call backend put '(4 : nat, 300 : nat)' && \
  result1=$(icp canister call --query backend get '(1 : nat)') && \
  result2=$(icp canister call --query backend get '(2 : nat)') && \
  result3=$(icp canister call --query backend get '(3 : nat)') && \
  result4=$(icp canister call --query backend get '(4 : nat)') && \
  echo "$result1" && echo "$result2" && echo "$result3" && echo "$result4" && \
  echo "$result1" | grep -q 'opt (42' && \
  echo "$result2" | grep -q 'opt (100' && \
  echo "$result3" | grep -q 'opt (200' && \
  echo "$result4" | grep -q 'opt (300' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 7: lookup (query) returns the partition index and a non-empty canister ID ==="
result=$(icp canister call --query backend lookup '(1 : nat)') && \
  echo "$result" && \
  echo "$result" | grep -q '1 :' && \
  echo "$result" | grep -qE '[a-z0-9]{5}-[a-z0-9]{5}' && \
  echo "PASS" || (echo "FAIL" && exit 1)
