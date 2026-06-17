#!/usr/bin/env bash
set -e

echo "=== Test 1: isHighScore returns true when leaderboard has fewer than 10 entries ==="
result=$(icp canister call --query backend isHighScore '(0)') && \
  echo "$result" && \
  echo "$result" | grep -q 'true' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: addLeaderboardEntry returns entry in leaderboard ==="
result=$(icp canister call backend addLeaderboardEntry '("Alice", 100)') && \
  echo "$result" && \
  echo "$result" | grep -q '"Alice"' && \
  echo "$result" | grep -q '100' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: getLeaderboard returns persisted entry ==="
result=$(icp canister call --query backend getLeaderboard '()') && \
  echo "$result" && \
  echo "$result" | grep -q '"Alice"' && \
  echo "$result" | grep -q '100' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: getRandomness returns a blob ==="
result=$(icp canister call backend getRandomness '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'blob' && \
  echo "PASS" || (echo "FAIL" && exit 1)
