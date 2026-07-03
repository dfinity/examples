#!/usr/bin/env bash
set -e

COUNTER_ID=$(icp canister status counter -i)
echo "Counter canister: $COUNTER_ID"

echo "=== Setup: reset counter to 0 and top up caller with cycles ==="
icp canister call counter set '(0 : nat)'
icp canister top-up --amount 1t caller

echo "=== Test 1: call_get_and_set returns old value and sets new ==="
result=$(icp canister call caller call_get_and_set "(principal \"$COUNTER_ID\", 42 : nat)") && \
  echo "$result" && \
  echo "$result" | grep -q '(0 : nat)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: set_then_get sets value and returns it ==="
result=$(icp canister call caller set_then_get "(principal \"$COUNTER_ID\", 7 : nat)") && \
  echo "$result" && \
  echo "$result" | grep -q '(7 : nat)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: counter value persisted ==="
result=$(icp canister call --query counter get '()') && \
  echo "$result" && \
  echo "$result" | grep -q '(7 : nat)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: call_increment increments counter ==="
result=$(icp canister call caller call_increment "(principal \"$COUNTER_ID\")") && \
  echo "$result" && \
  echo "$result" | grep -q 'Ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: counter is 8 after increment ==="
result=$(icp canister call --query counter get '()') && \
  echo "$result" && \
  echo "$result" | grep -q '(8 : nat)' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6: call_get (bounded-wait) returns current counter value ==="
result=$(icp canister call caller call_get "(principal \"$COUNTER_ID\")") && \
  echo "$result" && \
  echo "$result" | grep -q 'Ok = 8' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 7: stubborn_set completes successfully (no retries needed against a healthy canister) ==="
result=$(icp canister call caller stubborn_set "(principal \"$COUNTER_ID\", 99 : nat)") && \
  echo "$result" && \
  echo "$result" | grep -q 'Ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 8: counter is 99 after stubborn_set ==="
result=$(icp canister call caller call_get "(principal \"$COUNTER_ID\")") && \
  echo "$result" && \
  echo "$result" | grep -q 'Ok = 99' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 9: send_cycles transfers cycles to counter canister ==="
result=$(icp canister call caller send_cycles "(principal \"$COUNTER_ID\", 100_000_000_000 : nat64)") && \
  echo "$result" && \
  echo "$result" | grep -q 'Ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)
