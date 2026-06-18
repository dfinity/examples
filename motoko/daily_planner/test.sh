#!/usr/bin/env bash
set -e

echo "=== Test 1/6: getDayData returns null for a date with no notes ==="
result=$(icp canister call backend getDayData '("2000-01-15")') && \
  echo "$result" && \
  echo "$result" | grep -q 'null' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2/6: addNote returns ok result ==="
result=$(icp canister call backend addNote '("2000-01-15", "Buy groceries")') && \
  echo "$result" && \
  echo "$result" | grep -q 'ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3/6: getDayData returns stored note ==="
result=$(icp canister call backend getDayData '("2000-01-15")') && \
  echo "$result" && \
  echo "$result" | grep -q 'Buy groceries' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4/6: getMonthData returns entry for the stored month ==="
result=$(icp canister call backend getMonthData '(2000, 1)') && \
  echo "$result" && \
  echo "$result" | grep -q 'Buy groceries' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5/6: completeNote marks note as completed ==="
icp canister call backend completeNote '("2000-01-15", 0)' && \
  result=$(icp canister call backend getDayData '("2000-01-15")') && \
  echo "$result" && \
  echo "$result" | grep -q 'isCompleted = true' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6/6: getMonthData returns empty list for a different month ==="
result=$(icp canister call backend getMonthData '(1999, 12)') && \
  echo "$result" && \
  echo "$result" | grep -q 'vec {}' && \
  echo "PASS" || (echo "FAIL" && exit 1)
