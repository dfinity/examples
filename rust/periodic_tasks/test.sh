#!/usr/bin/env bash
set -e

echo "=== Test 1: Polling until timer counter is non-zero (up to 60s) ==="
secs=0
while [ $secs -lt 60 ]; do
  result=$(icp canister call --query timer counter '()')
  echo "$result"
  echo "$result" | grep -qv '(0 : nat32)' && echo "PASS" && break
  sleep 3; secs=$((secs + 3))
done
echo "$result" | grep -qv '(0 : nat32)' || (echo "FAIL: timer counter did not increment within 60s" && exit 1)

echo "=== Test 2: Polling until heartbeat counter is non-zero (up to 60s) ==="
secs=0
while [ $secs -lt 60 ]; do
  result=$(icp canister call --query heartbeat counter '()')
  echo "$result"
  echo "$result" | grep -qv '(0 : nat32)' && echo "PASS" && break
  sleep 3; secs=$((secs + 3))
done
echo "$result" | grep -qv '(0 : nat32)' || (echo "FAIL: heartbeat counter did not increment within 60s" && exit 1)

echo "=== Test 3: Polling until timer cycles_used is non-zero (up to 60s) ==="
secs=0
while [ $secs -lt 60 ]; do
  result=$(icp canister call --query timer cycles_used '()')
  echo "$result"
  echo "$result" | grep -qv '(0 : nat)' && echo "PASS" && break
  sleep 3; secs=$((secs + 3))
done
echo "$result" | grep -qv '(0 : nat)' || (echo "FAIL: timer cycles_used did not update within 60s" && exit 1)

echo "=== Test 4: Polling until heartbeat cycles_used is non-zero (up to 60s) ==="
secs=0
while [ $secs -lt 60 ]; do
  result=$(icp canister call --query heartbeat cycles_used '()')
  echo "$result"
  echo "$result" | grep -qv '(0 : nat)' && echo "PASS" && break
  sleep 3; secs=$((secs + 3))
done
echo "$result" | grep -qv '(0 : nat)' || (echo "FAIL: heartbeat cycles_used did not update within 60s" && exit 1)

echo "=== Test 5: Timer start_with_interval_secs accepts a new interval ==="
result=$(icp canister call timer start_with_interval_secs '(5 : nat64)') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6: Heartbeat set_interval_secs accepts a new interval ==="
result=$(icp canister call heartbeat set_interval_secs '(5 : nat64)') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 7: Timer stop returns successfully ==="
result=$(icp canister call timer stop '()') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 8: Heartbeat stop returns successfully ==="
result=$(icp canister call heartbeat stop '()') && \
  echo "$result" && \
  echo "PASS" || (echo "FAIL" && exit 1)
