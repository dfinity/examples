#!/usr/bin/env bash
set -e

trap 'rm -rf ./snapshots' EXIT

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
# sed -i '' is portable across macOS and Linux
sed -i '' 's/Colour/Color/g' ./snapshots/stable_memory.bin
new_id=$(icp canister snapshot upload -q -i ./snapshots backend)
echo "New Snapshot ID: $new_id"
icp canister snapshot restore backend "$new_id"
icp canister start backend
result=$(icp canister call --query backend print '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'Colorless' && \
  echo "PASS" || (echo "FAIL" && exit 1)
