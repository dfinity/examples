#!/usr/bin/env bash
set -e

test_id=$(icp canister id test)
echo "Test canister ID: $test_id"

echo "=== Test 1: info returns canister history for the test canister ==="
result=$(icp canister call backend info "(principal \"$test_id\")") && \
  echo "$result" && \
  echo "$result" | grep -q 'total_num_changes' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: reflexive_transitive_controllers returns a non-empty list ==="
result=$(icp canister call backend reflexive_transitive_controllers "(principal \"$test_id\")") && \
  echo "$result" && \
  echo "$result" | grep -q 'principal' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: canister_controllers returns controllers at version 1 ==="
result=$(icp canister call backend canister_controllers "(principal \"$test_id\", variant { at_version = 1 : nat64 })") && \
  echo "$result" && \
  echo "$result" | grep -q 'controllers' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: canister_module_hash returns module hash at version 1 ==="
result=$(icp canister call backend canister_module_hash "(principal \"$test_id\", variant { at_version = 1 : nat64 })") && \
  echo "$result" && \
  echo "$result" | grep -q 'module_hash' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: canister_deployment_chain returns deployment chain at version 1 ==="
result=$(icp canister call backend canister_deployment_chain "(principal \"$test_id\", variant { at_version = 1 : nat64 })") && \
  echo "$result" && \
  echo "$result" | grep -q 'deployment_chain' && \
  echo "PASS" || (echo "FAIL" && exit 1)
