#!/usr/bin/env bash
set -e

backend=$(icp canister status backend -i)
echo "Backend canister: $backend"

echo "=== Test 1: compute_subaccount matches known reference value ==="
# Fixed reference: principal "aaaaa-aa" (management canister), nonce 0.
# The expected value is a pre-computed SHA-256 domain-separated hash.
# If this changes, the subaccount computation is broken and real ICP could be sent
# to the wrong address.
result=$(icp canister call --query backend compute_subaccount \
  '(principal "aaaaa-aa", 0 : nat64)')
echo "$result"
echo "$result" | grep -q '"b8c5a0fbf187460e550de4c606ab9ba102f7826c43ee644b80f275eb952c0aa8"' && \
  echo "PASS" || (echo "FAIL: subaccount hash does not match expected reference value" && exit 1)

echo "=== Test 2: compute_subaccount changes with nonce ==="
s0=$(icp canister call --query backend compute_subaccount "(principal \"$backend\", 0 : nat64)")
s1=$(icp canister call --query backend compute_subaccount "(principal \"$backend\", 1 : nat64)")
echo "Nonce 0: $s0"
echo "Nonce 1: $s1"
[ "$s0" != "$s1" ] && echo "PASS" || (echo "FAIL: different nonces produced the same subaccount" && exit 1)

echo "=== Test 3: stake_neuron rejects amount below 1 ICP minimum ==="
result=$(icp canister call backend stake_neuron '(99_999_999 : nat64, 0 : nat64)')
echo "$result"
echo "$result" | grep -q 'Err' && echo "PASS" || (echo "FAIL: expected rejection for sub-minimum amount" && exit 1)

echo "=== Setup: fund backend with 2 ICP ==="
icp token transfer 2 "$backend"

echo "=== Test 4: stake_neuron stakes 1 ICP and subaccount_hex matches compute_subaccount ==="
# Re-running this test tops up the existing neuron rather than creating a new one —
# claim_or_refresh reuses the same neuron_id for the same (controller, nonce) pair.
# Each re-run adds 1 ICP to the neuron's stake; the neuron_id stays the same.
expected_subaccount=$(icp canister call --query backend compute_subaccount \
  "(principal \"$backend\", 0 : nat64)" | grep -oE '[0-9a-f]{64}')
result=$(icp canister call backend stake_neuron '(100_000_000 : nat64, 0 : nat64)')
echo "$result"
echo "$result" | grep -q 'Ok' || (echo "FAIL: stake_neuron did not return Ok" && exit 1)
echo "$result" | grep -q "neuron_id" || (echo "FAIL: missing neuron_id in result" && exit 1)
echo "$result" | grep -q "$expected_subaccount" && \
  echo "PASS" || (echo "FAIL: subaccount_hex in result does not match compute_subaccount" && exit 1)
