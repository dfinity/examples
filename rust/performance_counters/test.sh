#!/usr/bin/env bash
set -e

# Extracts the two nat64 values from a result like "(7_016_932 : nat64, 21_071_365 : nat64)"
# and asserts that the call context counter (1) is strictly greater than the current
# execution counter (0) — the key behavioral difference this example demonstrates.
assert_context_exceeds_current() {
  local result="$1" label="$2"
  local current context
  current=$(echo "$result" | grep -oE '[0-9][0-9_]*' | tr -d '_' | sed -n '1p')
  context=$(echo "$result" | grep -oE '[0-9][0-9_]*' | tr -d '_' | sed -n '2p')
  if [ "$context" -gt "$current" ]; then
    echo "PASS (current=$current, call_context=$context)"
  else
    echo "FAIL: expected call context counter ($context) > current counter ($current)"
    exit 1
  fi
}

echo "=== Test 1: for_update — call context counter accumulates across await boundaries ==="
result=$(icp canister call backend for_update '()') && \
  echo "$result" && \
  assert_context_exceeds_current "$result" "for_update"

echo "=== Test 2: for_composite_query — same counter semantics apply as for update calls ==="
result=$(icp canister call --query backend for_composite_query '()') && \
  echo "$result" && \
  assert_context_exceeds_current "$result" "for_composite_query"

echo "=== Test 3: example — call context counter accumulates across nested calls ==="
result=$(icp canister call --query backend example '()') && \
  echo "$result" && \
  assert_context_exceeds_current "$result" "example"
