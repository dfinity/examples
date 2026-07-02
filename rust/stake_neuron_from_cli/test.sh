#!/usr/bin/env bash
set -e

echo "=== Test 1/2: compute_subaccount returns a 64-char hex string ==="
result=$(icp canister call --query backend compute_subaccount \
  '(principal "aaaaa-aa", 0 : nat64)') && \
  echo "$result" && \
  echo "$result" | grep -qE '"[0-9a-f]{64}"' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2/2: compute_subaccount is deterministic (same inputs → same output) ==="
r1=$(icp canister call --query backend compute_subaccount \
  '(principal "aaaaa-aa", 42 : nat64)') && \
  r2=$(icp canister call --query backend compute_subaccount \
  '(principal "aaaaa-aa", 42 : nat64)') && \
  echo "Run 1: $r1" && \
  echo "Run 2: $r2" && \
  [ "$r1" = "$r2" ] && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo ""
echo "NOTE: stake_neuron requires the canister to hold ICP and NNS Governance"
echo "to be reachable. Test against mainnet by calling:"
echo "  icp canister call --network ic backend stake_neuron '(<amount_e8s> : nat64, <nonce> : nat64)'"
