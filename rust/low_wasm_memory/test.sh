#!/usr/bin/env bash
set -e

# Memory limits are configured in icp.yaml (wasm_memory_limit: 8mib, wasm_memory_threshold: 2mib).
# The on_low_wasm_memory hook fires when usage exceeds 8 - 2 = 6 MiB.

echo "=== Polling for OnLowWasmMemory hook (up to 60s) ==="
secs=0
while [ "$secs" -lt 60 ]; do
  result=$(icp canister call --query backend get_executed_functions_order '()') || \
    { echo "FAIL: canister call failed"; exit 1; }
  echo "$result"
  echo "$result" | grep -q 'OnLowWasmMemory' && echo "PASS" && exit 0
  sleep 3
  secs=$((secs + 3))
done
echo "FAIL: OnLowWasmMemory hook did not fire within 60s"
exit 1
