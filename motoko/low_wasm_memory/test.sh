#!/usr/bin/env bash
set -e

# Configure memory limits:
# - wasm_memory_limit: 5 MiB total Wasm memory
# - wasm_memory_threshold: 2 MiB remaining before hook fires
# The lowmemory hook fires when usage exceeds 5 - 2 = 3 MiB.
# The canister starts with ~2.3 MiB after deployment and allocates 10 000
# elements per heartbeat, so the hook triggers after roughly 0.7 MiB more.
echo "=== Configuring canister memory limits ==="
icp canister settings update backend \
  --wasm-memory-limit 5mib \
  --wasm-memory-threshold 2mib \
  -f

echo "=== Polling for onLowWasmMemory hook (up to 60s) ==="
secs=0
while [ "$secs" -lt 60 ]; do
  result=$(icp canister call --query backend getExecutedFunctionsOrder '()') || \
    { echo "FAIL: canister call failed"; exit 1; }
  echo "$result"
  echo "$result" | grep -q 'onLowWasmMemory' && echo "PASS" && exit 0
  sleep 3
  secs=$((secs + 3))
done
echo "FAIL: onLowWasmMemory hook did not fire within 60s"
exit 1
