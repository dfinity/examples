#!/usr/bin/env bash
set -e

TOKEN_A=$(icp canister status token_a -i)
TOKEN_B=$(icp canister status token_b -i)
ALICE=$(icp identity principal --identity icrc2-alice)
BOB=$(icp identity principal --identity icrc2-bob)
SWAP=$(icp canister status backend -i)

echo "=== Test 1: balances returns empty lists initially ==="
result=$(icp canister call --query backend balances '()') && \
  echo "$result" && \
  echo "$result" | grep -q 'vec {}' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 2: swap with no deposits returns InsufficientBalance ==="
result=$(icp canister call backend swap "(record { \
  user_a = principal \"$ALICE\"; \
  user_b = principal \"$BOB\" \
})") && \
  echo "$result" && \
  echo "$result" | grep -q 'InsufficientBalance' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 3: icrc2-alice approves and deposits token_a ==="
# Approve amount = deposit (100_000_000) + transfer_fee (10_000) = 100_010_000.
# The ledger charges a fee on icrc2_transfer_from, so the approval must cover it.
icp canister call --identity icrc2-alice token_a icrc2_approve "(record { \
  amount = 100_010_000 : nat; \
  spender = record { owner = principal \"$SWAP\"; subaccount = null }; \
  expires_at = null; expected_allowance = null; fee = null; \
  from_subaccount = null; memo = null; created_at_time = null \
})"
result=$(icp canister call --identity icrc2-alice backend deposit "(record { \
  token = principal \"$TOKEN_A\"; \
  from = record { owner = principal \"$ALICE\"; subaccount = null }; \
  amount = 100_000_000 : nat; \
  spender_subaccount = null; fee = null; memo = null; created_at_time = null \
})") && \
  echo "$result" && \
  echo "$result" | grep -q 'ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 4: icrc2-bob approves and deposits token_b ==="
icp canister call --identity icrc2-bob token_b icrc2_approve "(record { \
  amount = 100_010_000 : nat; \
  spender = record { owner = principal \"$SWAP\"; subaccount = null }; \
  expires_at = null; expected_allowance = null; fee = null; \
  from_subaccount = null; memo = null; created_at_time = null \
})"
result=$(icp canister call --identity icrc2-bob backend deposit "(record { \
  token = principal \"$TOKEN_B\"; \
  from = record { owner = principal \"$BOB\"; subaccount = null }; \
  amount = 100_000_000 : nat; \
  spender_subaccount = null; fee = null; memo = null; created_at_time = null \
})") && \
  echo "$result" && \
  echo "$result" | grep -q 'ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 5: swap alice's token_a for bob's token_b ==="
result=$(icp canister call backend swap "(record { \
  user_a = principal \"$ALICE\"; \
  user_b = principal \"$BOB\" \
})") && \
  echo "$result" && \
  echo "$result" | grep -q 'ok' && \
  echo "PASS" || (echo "FAIL" && exit 1)

echo "=== Test 6: alice withdraws token_b (received via swap) ==="
# The backend deducts (amount + transfer_fee) from alice's internal balance.
# Internal balance = 100_000_000; withdraw 99_990_000 so deduction = 99_990_000 + 10_000 = 100_000_000.
before=$(icp canister call --query token_b icrc1_balance_of \
  "(record { owner = principal \"$ALICE\"; subaccount = null })" | grep -oE '[0-9_]+' | tr -d '_')
result=$(icp canister call --identity icrc2-alice backend withdraw "(record { \
  token = principal \"$TOKEN_B\"; \
  to = record { owner = principal \"$ALICE\"; subaccount = null }; \
  amount = 99_990_000 : nat; \
  fee = null; memo = null; created_at_time = null \
})")
echo "$result"
echo "$result" | grep -q 'ok' || (echo "FAIL" && exit 1)
after=$(icp canister call --query token_b icrc1_balance_of \
  "(record { owner = principal \"$ALICE\"; subaccount = null })" | grep -oE '[0-9_]+' | tr -d '_')
delta=$((after - before))
echo "token_b balance: before=$before after=$after delta=$delta"
[ "$delta" -eq 99990000 ] || (echo "FAIL: expected delta 99990000" && exit 1)
echo "PASS"

echo "=== Test 7: bob withdraws token_a (received via swap) ==="
before=$(icp canister call --query token_a icrc1_balance_of \
  "(record { owner = principal \"$BOB\"; subaccount = null })" | grep -oE '[0-9_]+' | tr -d '_')
result=$(icp canister call --identity icrc2-bob backend withdraw "(record { \
  token = principal \"$TOKEN_A\"; \
  to = record { owner = principal \"$BOB\"; subaccount = null }; \
  amount = 99_990_000 : nat; \
  fee = null; memo = null; created_at_time = null \
})")
echo "$result"
echo "$result" | grep -q 'ok' || (echo "FAIL" && exit 1)
after=$(icp canister call --query token_a icrc1_balance_of \
  "(record { owner = principal \"$BOB\"; subaccount = null })" | grep -oE '[0-9_]+' | tr -d '_')
delta=$((after - before))
echo "token_a balance: before=$before after=$after delta=$delta"
[ "$delta" -eq 99990000 ] || (echo "FAIL: expected delta 99990000" && exit 1)
echo "PASS"
