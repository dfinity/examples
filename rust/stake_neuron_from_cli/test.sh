#!/usr/bin/env bash
set -e

backend=$(icp canister status backend -i)
echo "Backend canister: $backend"

echo "=== Test 1: compute_subaccount returns a 64-char hex string ==="
result=$(icp canister call --query backend compute_subaccount \
  "(principal \"$backend\", 0 : nat64)") && \
  echo "$result" && \
  echo "$result" | grep -qE '"[0-9a-f]{64}"' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: compute_subaccount is deterministic (same inputs → same output) ==="
r1=$(icp canister call --query backend compute_subaccount \
  "(principal \"$backend\", 42 : nat64)")
r2=$(icp canister call --query backend compute_subaccount \
  "(principal \"$backend\", 42 : nat64)")
echo "Run 1: $r1"
echo "Run 2: $r2"
[ "$r1" = "$r2" ] && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Setup: fund backend with 2 ICP ==="
icp token transfer 2 "$backend"

echo "=== Test 3: stake_neuron — stakes 1 ICP and returns a neuron ID ==="
# Minimum stake enforced by NNS Governance is 100_000_000 e8s (1 ICP).
# The transfer also deducts 10_000 e8s in fees, so we need at least 100_010_000 e8s.
result=$(icp canister call backend stake_neuron '(100_000_000 : nat64, 0 : nat64)') && \
  echo "$result" && \
  echo "$result" | grep -q 'Ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)
