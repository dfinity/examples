#!/usr/bin/env bash
set -e

backend=$(icp canister status backend -i)
echo "Backend: $backend"

# Extract the nat64 value from Candid output like "(100_000_000 : nat64)"
e8s_from_result() {
  echo "$1" | grep -oE '[0-9_]+' | tr -d '_' | head -1
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

echo "=== Test 4: fund main account with 1 ICP — get_balance increases by 100_000_000 e8s ==="
before=$(e8s_from_result "$(icp canister call backend get_balance '()')")
icp token transfer 1 "$main_hex"
after=$(e8s_from_result "$(icp canister call backend get_balance '()')")
delta=$((after - before))
echo "  before=$before after=$after delta=$delta"
[ "$delta" -eq 100000000 ] && echo "PASS" || (echo "FAIL: expected delta +100000000 e8s" && exit 1)

echo "=== Test 5: get_balance_of_subaccount(0, 0) agrees with get_balance() ==="
balance_main=$(e8s_from_result "$(icp canister call backend get_balance '()')")
balance_sub0=$(e8s_from_result "$(icp canister call backend get_balance_of_subaccount '(0, 0)')")
echo "  get_balance(): $balance_main  get_balance_of_subaccount(0,0): $balance_sub0"
[ "$balance_main" -eq "$balance_sub0" ] && echo "PASS" || (echo "FAIL: balances differ" && exit 1)

echo "=== Test 6: fund subaccount(1, 0) via account ID hex — get_balance_of_subaccount increases by 100_000_000 e8s ==="
echo "  Subaccount(1,0) account ID: $sub1_hex"
before=$(e8s_from_result "$(icp canister call backend get_balance_of_subaccount '(1, 0)')")
icp token transfer 1 "$sub1_hex"
after=$(e8s_from_result "$(icp canister call backend get_balance_of_subaccount '(1, 0)')")
delta=$((after - before))
echo "  before=$before after=$after delta=$delta"
[ "$delta" -eq 100000000 ] && echo "PASS" || (echo "FAIL: expected delta +100000000 e8s" && exit 1)

echo "=== Test 7: subaccount(2, 0) is unfunded — proves subaccounts are independent ==="
balance_sub0=$(e8s_from_result "$(icp canister call backend get_balance_of_subaccount '(0, 0)')")
balance_sub2=$(e8s_from_result "$(icp canister call backend get_balance_of_subaccount '(2, 0)')")
echo "  subaccount(0,0): $balance_sub0 e8s (funded)"
echo "  subaccount(2,0): $balance_sub2 e8s (never funded)"
[ "$balance_sub0" -gt 0 ] || (echo "FAIL: subaccount(0,0) should have balance" && exit 1)
[ "$balance_sub2" -eq 0 ] && echo "PASS" || (echo "FAIL: unfunded subaccount(2,0) should have 0 balance" && exit 1)
