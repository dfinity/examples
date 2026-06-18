#!/usr/bin/env bash
set -e

# Tests use a single shell block so all variables and functions stay in scope.
# Balance checks are delta-based so the tests are idempotent across re-runs.
#
# Transfer amount is 99_990_000 e8s so that amount + fee (10_000) = 100_000_000 e8s
# = exactly 1 ICP deducted per transfer. Two tests drain the 2 ICP funding to zero.

backend=$(icp canister status backend -i)
recipient=$(icp identity principal)
echo "Backend:   $backend"
echo "Recipient: $recipient"

get_balance() {
  icp token balance -q --of-principal "$1" \
    | awk '{printf "%.0f", $1 * 100000000}'
}

echo "=== Test 1: toAccountIdHex matches icp identity account-id ==="
cli_hex=$(icp identity account-id --format ledger)
backend_hex=$(icp canister call --query backend toAccountIdHex \
  "(principal \"$recipient\", null)" | grep -oE '[0-9a-f]{64}')
echo "  icp identity account-id:  $cli_hex"
echo "  backend toAccountIdHex:   $backend_hex"
[ "$cli_hex" = "$backend_hex" ] || (echo "FAIL: hex values differ" && exit 1)
echo "  PASS"

echo "=== Setup: fund backend with 2 ICP ==="
icp token transfer 2 "$backend"

echo "=== Test 2: transferToPrincipal — deducts exactly 1 ICP (99_990_000 amount + 10_000 fee) ==="
backend_before=$(get_balance "$backend")
recipient_before=$(get_balance "$recipient")
result=$(icp canister call backend transferToPrincipal \
  "(record { e8s = 99_990_000 : nat64 }, principal \"$recipient\", null)")
echo "  result: $result"
echo "$result" | grep -q 'ok' || (echo "FAIL: transfer rejected" && exit 1)
backend_after=$(get_balance "$backend")
recipient_after=$(get_balance "$recipient")
echo "  backend:   $backend_before → $backend_after  (delta $((backend_after - backend_before)))"
echo "  recipient: $recipient_before → $recipient_after  (delta $((recipient_after - recipient_before)))"
[ "$((backend_after - backend_before))" -eq -100000000 ] || (echo "FAIL: expected backend delta -100000000 e8s" && exit 1)
[ "$((recipient_after - recipient_before))" -eq 99990000 ] || (echo "FAIL: expected recipient delta +99990000 e8s" && exit 1)
echo "  PASS"

echo "=== Test 3: transferToAccountId — same recipient, same deduction ==="
echo "  AccountIdentifier hex: $cli_hex"
backend_before=$(get_balance "$backend")
recipient_before=$(get_balance "$recipient")
result=$(icp canister call backend transferToAccountId \
  "(record { e8s = 99_990_000 : nat64 }, \"$cli_hex\")")
echo "  result: $result"
echo "$result" | grep -q 'ok' || (echo "FAIL: transfer rejected" && exit 1)
backend_after=$(get_balance "$backend")
recipient_after=$(get_balance "$recipient")
echo "  backend:   $backend_before → $backend_after  (delta $((backend_after - backend_before)))"
echo "  recipient: $recipient_before → $recipient_after  (delta $((recipient_after - recipient_before)))"
[ "$((backend_after - backend_before))" -eq -100000000 ] || (echo "FAIL: expected backend delta -100000000 e8s" && exit 1)
[ "$((recipient_after - recipient_before))" -eq 99990000 ] || (echo "FAIL: expected recipient delta +99990000 e8s" && exit 1)
echo "  PASS — same recipient reached via AccountIdentifier hex as via principal"
