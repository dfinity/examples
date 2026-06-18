#!/usr/bin/env bash
set -e

echo "=== Test 1: getBalance returns the canister's cycle balance ==="
result=$(icp canister call --query backend getBalance '()') && \
  echo "$result" && \
  echo "$result" | grep -qE '^\([0-9][0-9_]* : nat\)$' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: acceptCycles accepts 1M cycles sent via the proxy canister ==="
PROXY=$(icp network status --json | jq -r .proxy_canister_principal) && \
  result=$(icp canister call backend acceptCycles '()' --proxy "$PROXY" --cycles 1_000_000) && \
  echo "$result" && \
  echo "$result" | grep -qE 'accepted = [1-9]' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: sendCycles forwards 5M to itself — within limit, refunded = 0 ==="
# The canister calls its own acceptCycles as receiver (inter-canister self-call).
# 5M offered < 10M limit, so all cycles are accepted and none are refunded.
BACKEND_ID=$(icp canister status backend -i) && \
  result=$(icp canister call backend sendCycles "(func \"$BACKEND_ID\".\"acceptCycles\", 5_000_000)") && \
  echo "$result" && \
  echo "$result" | grep -q 'refunded = 0' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: sendCycles forwards 15M to itself — over limit, refunded = 5_000_000 ==="
# 15M offered > 10M limit, so 10M are accepted and exactly 5M are refunded.
BACKEND_ID=$(icp canister status backend -i) && \
  result=$(icp canister call backend sendCycles "(func \"$BACKEND_ID\".\"acceptCycles\", 15_000_000)") && \
  echo "$result" && \
  echo "$result" | grep -q 'refunded = 5_000_000' && \
  echo "PASS" || (echo "FAIL" && exit 1)
