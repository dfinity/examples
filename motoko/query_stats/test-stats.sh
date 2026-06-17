#!/usr/bin/env bash
set -e

echo "=== Polling query stats (making 13 --query calls every 3s, up to 30s) ==="
secs=0
while [ "$secs" -lt 30 ]; do
  for i in $(seq 1 13); do
    icp canister call --query backend load '()' > /dev/null
  done
  result=$(icp canister call backend get_current_query_stats_as_string '()')
  echo "$result" | grep -qE 'Number of calls: [1-9]' && \
    echo "$result" && echo "PASS: query stats are non-zero (after ~${secs}s)" && exit 0
  sleep 3
  secs=$((secs + 3))
done
result=$(icp canister call backend get_current_query_stats_as_string '()')
echo "$result"
echo "FAIL: stats still 0 after 30s"; exit 1
