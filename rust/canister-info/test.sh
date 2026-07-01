#!/usr/bin/env bash
set -e

test_id=$(icp canister status test -i)
backend_id=$(icp canister status backend -i)
echo "Test canister ID: $test_id"
echo "Backend canister ID: $backend_id"

# Remove the controller added in test 6 on exit so the test canister is left
# in a clean state and the tests are idempotent across multiple runs.
trap 'icp canister settings update test --remove-controller "$backend_id" -f 2>/dev/null || true' EXIT

# ── Querying the initial canister history ────────────────────────────────────

echo "=== Test 1: info returns the full canister history ==="
info_result=$(icp canister call backend info "(principal \"$test_id\")")
echo "$info_result"
echo "$info_result" | grep -q 'total_num_changes' && echo "PASS" || (echo "FAIL" && exit 1)

# Extract the canister version of the initial code deployment (mode = install).
# We scan for the canister_version value that precedes `mode = variant { install }`
# in the history, so this is robust across multiple test runs that add history entries.
code_deploy_version=$(echo "$info_result" | awk '
  /canister_version = [0-9]+ : nat64/ { match($0, /[0-9]+/); v = substr($0, RSTART, RLENGTH) }
  /mode = variant \{ install \}/ { print v; exit }
')
echo "Code deployment version: $code_deploy_version"

echo "=== Test 2: reflexive_transitive_controllers returns the canister and its controllers ==="
result=$(icp canister call backend reflexive_transitive_controllers "(principal \"$test_id\")")
echo "$result"
echo "$result" | grep -q "$test_id" && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: canister_controllers returns the controllers at the code-deployment version ==="
result=$(icp canister call backend canister_controllers "(principal \"$test_id\", variant { at_version = $code_deploy_version : nat64 })")
echo "$result"
echo "$result" | grep -q 'controllers' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: canister_module_hash returns the deployed module hash ==="
result=$(icp canister call backend canister_module_hash "(principal \"$test_id\", variant { at_version = $code_deploy_version : nat64 })")
echo "$result"
echo "$result" | grep -q 'blob' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: canister_deployment_chain shows the code_deployment entry ==="
result=$(icp canister call backend canister_deployment_chain "(principal \"$test_id\", variant { at_version = $code_deploy_version : nat64 })")
echo "$result"
echo "$result" | grep -q 'code_deployment' && echo "PASS" || (echo "FAIL" && exit 1)

# ── Verifying that history tracks live changes ────────────────────────────────
# Add the backend canister as a controller of the test canister.
# This creates a new controllers_change entry in the canister history.

echo "=== Test 6: adding a controller creates a new history entry ==="
icp canister settings update test --add-controller "$backend_id" -f
info_result2=$(icp canister call backend info "(principal \"$test_id\")")
echo "$info_result2"
echo "$info_result2" | grep -q 'controllers_change' && echo "PASS" || (echo "FAIL" && exit 1)

new_version=$(echo "$info_result2" | grep 'canister_version' | tail -1 | grep -oE '[0-9]+' | head -1)
echo "Controller-change version: $new_version"

echo "=== Test 7: canister_controllers at the new version includes the added controller ==="
result=$(icp canister call backend canister_controllers "(principal \"$test_id\", variant { at_version = $new_version : nat64 })")
echo "$result"
echo "$result" | grep -q "$backend_id" && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 8: canister_controllers at the code-deployment version does NOT include the added controller ==="
result=$(icp canister call backend canister_controllers "(principal \"$test_id\", variant { at_version = $code_deploy_version : nat64 })")
echo "$result"
echo "$result" | grep -q "$backend_id" && (echo "FAIL: backend incorrectly appears in old history" && exit 1) || echo "PASS"
