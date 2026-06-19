#!/usr/bin/env bash
set -e

# Memory limits are configured in icp.yaml (wasm_memory_limit: 5mib, wasm_memory_threshold: 2mib).
# The onLowWasmMemory hook fires when usage exceeds 5 - 2 = 3 MiB.
# The canister starts with ~2.3 MiB after deployment and allocates 10 000
# elements per heartbeat, so the hook triggers after roughly 0.7 MiB more.

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
