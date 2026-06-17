#!/usr/bin/env bash
set -e

backend=$(icp canister status backend -i)
echo "Backend: $backend"

e8s_from_icp() {
  echo "$1" | awk '{printf "%.0f", $1 * 100000000}'
}

echo "=== Test 1: account returns a 64-char hex account identifier ==="
result=$(icp canister call --query backend account '()')
echo "$result"
echo "$result" | grep -qE '"[0-9a-f]{64}"' && echo "PASS" || (echo "FAIL" && exit 1)
main_hex=$(echo "$result" | grep -oE '[0-9a-f]{64}')

echo "=== Test 2: subaccount(0, 0) returns same account as account() ==="
result_sub=$(icp canister call --query backend subaccount '(0, 0)')
echo "account:         $result"
echo "subaccount(0,0): $result_sub"
[ "$result" = "$result_sub" ] && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: subaccount(1, 0) differs from subaccount(0, 0) ==="
result_sub1=$(icp canister call --query backend subaccount '(1, 0)')
echo "subaccount(0,0): $result_sub"
echo "subaccount(1,0): $result_sub1"
sub1_hex=$(echo "$result_sub1" | grep -oE '[0-9a-f]{64}')
[ "$result_sub" != "$result_sub1" ] && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: get_balance returns 0 before funding ==="
result=$(icp canister call backend get_balance '()')
echo "$result"
echo "$result" | grep -qF '(0 : nat64)' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: fund main account with 1 ICP — get_balance returns 100_000_000 e8s ==="
icp token transfer 1 "$main_hex"
result=$(icp canister call backend get_balance '()')
echo "$result"
echo "$result" | grep -qF '(100_000_000 : nat64)' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6: get_balance_of_subaccount(0, 0) matches get_balance() ==="
result=$(icp canister call backend get_balance_of_subaccount '(0, 0)')
echo "$result"
echo "$result" | grep -qF '(100_000_000 : nat64)' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 7: get_balance_of_subaccount(1, 0) returns 0 before funding ==="
result=$(icp canister call backend get_balance_of_subaccount '(1, 0)')
echo "$result"
echo "$result" | grep -qF '(0 : nat64)' && echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 8: fund subaccount(1, 0) via account ID hex — get_balance_of_subaccount returns 100_000_000 e8s ==="
echo "  Subaccount(1,0) account ID: $sub1_hex"
icp token transfer 1 "$sub1_hex"
result=$(icp canister call backend get_balance_of_subaccount '(1, 0)')
echo "$result"
echo "$result" | grep -qF '(100_000_000 : nat64)' && echo "PASS" || (echo "FAIL" && exit 1)

# Cross-check: icp token balance with account ID hex should agree
echo "=== Cross-check: icp token balance with account ID hex ==="
balance_cli=$(icp token balance -q "$sub1_hex")
balance_e8s=$(e8s_from_icp "$balance_cli")
echo "  icp token balance: $balance_cli ($balance_e8s e8s)"
[ "$balance_e8s" -eq 100000000 ] && echo "PASS" || (echo "FAIL: expected 100000000 e8s" && exit 1)
