#!/usr/bin/env bash
set -e

echo "=== Test 1/2: load() returns a non-zero timestamp ==="
# --query ensures this call is recorded in query_stats (update calls are not tracked)
result=$(icp canister call --query backend load '()') && \
  echo "$result" && \
  echo "$result" | grep -qE '\([0-9][0-9_]* : int\)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2/2: get_current_query_stats_as_string() returns the four expected fields ==="
result=$(icp canister call backend get_current_query_stats_as_string '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'Number of calls' && \
  echo "$result" | grep -q 'Number of instructions' && \
  echo "$result" | grep -q 'Request payload bytes' && \
  echo "$result" | grep -q 'Response payload bytes' && \
  echo "PASS" || (echo "FAIL" && exit 1)
