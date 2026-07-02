#!/usr/bin/env bash
set -e

echo "=== Test 1: setup stable memory with initial data ==="
result=$(icp canister call backend setup '()') && \
  echo "$result" && \
  echo "$result" | grep -q '()' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: print returns the initial quote ==="
result=$(icp canister call --query backend print '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'Colourless' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: snapshot create/download/upload/load workflow ==="
mkdir -p ./snapshots
icp canister stop backend
snapshot_id=$(icp canister snapshot create backend | grep -oE '[0-9a-f]{36}')
echo "Snapshot ID: $snapshot_id"
icp canister snapshot download --dir ./snapshots backend "$snapshot_id"
sed -i 's/Colour/Color/g' ./snapshots/stable_memory.bin
new_id=$(icp canister snapshot upload --dir ./snapshots backend | grep -oE '[0-9a-f]{36}')
echo "New Snapshot ID: $new_id"
icp canister snapshot load backend "$new_id"
icp canister start backend
result=$(icp canister call --query backend print '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'Colorless' && \
  echo "PASS" || (echo "FAIL" && exit 1)
rm -rf ./snapshots
