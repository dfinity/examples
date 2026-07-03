#!/usr/bin/env bash
set -e

trap 'icp canister start backend 2>/dev/null || true; rm -rf ./snapshots' EXIT

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

echo "=== Test 3: snapshot create/download/upload/restore workflow ==="
mkdir -p ./snapshots
icp canister stop backend
snapshot_id=$(icp canister snapshot create -q backend)
echo "Snapshot ID: $snapshot_id"
icp canister snapshot download -o ./snapshots backend "$snapshot_id"
# Redirect through sed to avoid sed -i portability differences between macOS and Linux.
sed 's/Colour/Color/g' ./snapshots/stable_memory.bin > ./snapshots/stable_memory.bin.tmp
mv ./snapshots/stable_memory.bin.tmp ./snapshots/stable_memory.bin
new_id=$(icp canister snapshot upload -q -i ./snapshots backend)
echo "New Snapshot ID: $new_id"
icp canister snapshot restore backend "$new_id"
icp canister start backend
result=$(icp canister call --query backend print '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'Colorless' && \
  echo "PASS" || (echo "FAIL" && exit 1)
