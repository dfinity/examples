.PHONY: test

ICP_LEDGER := ryjl3-tyaaa-aaaaa-aaaba-cai

# Query the ICRC-1 balance (e8s) for a given principal.
balance_of = $$(icp canister call --query $(ICP_LEDGER) icrc1_balance_of \
  "(record { owner = principal \"$(1)\"; subaccount = null })" \
  | grep -oE '[0-9]+' | tr -d '_' | head -1)

# Tests use a single shell block so all variables stay in scope.
# Balance checks are delta-based so the tests are idempotent across re-runs.
test:
	@set -e; \
	  backend=$$(icp canister status backend -i); \
	  recipient=$$(icp identity principal); \
	  echo "Backend:   $$backend"; \
	  echo "Recipient: $$recipient"; \
	\
	  echo "=== Test 1: fund backend with 2 ICP — balance increases by 200_000_000 e8s ==="; \
	  before=$$(icp canister call --query $(ICP_LEDGER) icrc1_balance_of \
	    "(record { owner = principal \"$$backend\"; subaccount = null })" \
	    | grep -oE '[0-9]+' | tr -d '_' | head -1); \
	  icp token transfer 2 "$$backend"; \
	  after=$$(icp canister call --query $(ICP_LEDGER) icrc1_balance_of \
	    "(record { owner = principal \"$$backend\"; subaccount = null })" \
	    | grep -oE '[0-9]+' | tr -d '_' | head -1); \
	  delta=$$((after - before)); \
	  echo "Backend balance: before=$$before after=$$after delta=$$delta"; \
	  [ "$$delta" -eq 200000000 ] || (echo "FAIL: expected delta +200000000 e8s" && exit 1); \
	  echo "PASS"; \
	\
	  echo "=== Test 2: transfer 1 ICP to recipient — backend loses 100_010_000 e8s (amount + fee), recipient gains 100_000_000 e8s ==="; \
	  backend_before=$$(icp canister call --query $(ICP_LEDGER) icrc1_balance_of \
	    "(record { owner = principal \"$$backend\"; subaccount = null })" \
	    | grep -oE '[0-9]+' | tr -d '_' | head -1); \
	  recipient_before=$$(icp canister call --query $(ICP_LEDGER) icrc1_balance_of \
	    "(record { owner = principal \"$$recipient\"; subaccount = null })" \
	    | grep -oE '[0-9]+' | tr -d '_' | head -1); \
	  result=$$(icp canister call backend transfer "(record { \
	    amount = record { e8s = 100_000_000 : nat64 }; \
	    toPrincipal = principal \"$$recipient\"; \
	    toSubaccount = null \
	  })"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'ok' || (echo "FAIL: transfer rejected" && exit 1); \
	  backend_after=$$(icp canister call --query $(ICP_LEDGER) icrc1_balance_of \
	    "(record { owner = principal \"$$backend\"; subaccount = null })" \
	    | grep -oE '[0-9]+' | tr -d '_' | head -1); \
	  recipient_after=$$(icp canister call --query $(ICP_LEDGER) icrc1_balance_of \
	    "(record { owner = principal \"$$recipient\"; subaccount = null })" \
	    | grep -oE '[0-9]+' | tr -d '_' | head -1); \
	  backend_delta=$$((backend_after - backend_before)); \
	  recipient_delta=$$((recipient_after - recipient_before)); \
	  echo "Backend balance:   before=$$backend_before after=$$backend_after delta=$$backend_delta"; \
	  echo "Recipient balance: before=$$recipient_before after=$$recipient_after delta=$$recipient_delta"; \
	  [ "$$backend_delta" -eq -100010000 ] || (echo "FAIL: expected backend delta -100010000 e8s (amount + fee)" && exit 1); \
	  [ "$$recipient_delta" -eq 100000000 ] || (echo "FAIL: expected recipient delta +100000000 e8s" && exit 1); \
	  echo "PASS"
