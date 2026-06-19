#!/usr/bin/env bash
set -e

echo "=== Test 1: append a message ==="
result=$(icp canister call backend append '("Hi there!")') && \
  echo "$result" && \
  echo "$result" | grep -q '()' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: dump returns the appended message ==="
result=$(icp canister call --query backend dump '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'Hi there' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: take snapshot (stop, create, start) ==="
icp canister stop backend
snapshot_id=$(icp canister snapshot create backend | grep -oE '[0-9a-f]{36}')
echo "Snapshot ID: $snapshot_id"
icp canister start backend

echo "=== Test 4: simulate data loss via remove_spam ==="
result=$(icp canister call backend remove_spam '()') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: dump is empty after remove_spam bug ==="
result=$(icp canister call --query backend dump '()') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6: restore snapshot and verify data ==="
snapshot_id=$(icp canister snapshot list backend | grep -oE '^[0-9a-f]+')
icp canister stop backend && \
  icp canister snapshot restore backend "$snapshot_id" && \
  icp canister start backend
result=$(icp canister call --query backend dump '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'Hi there' && \
  echo "PASS" || (echo "FAIL" && exit 1)
