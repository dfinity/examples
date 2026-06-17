.PHONY: test

test:
	@echo "=== Test 1: account returns a 64-char hex account identifier ==="
	@result=$$(icp canister call --query backend account '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '"[0-9a-f]{64}"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: subaccount(0, 0) returns same account as account() ==="
	@result_account=$$(icp canister call --query backend account '()') && \
	  result_sub=$$(icp canister call --query backend subaccount '(0, 0)') && \
	  echo "account:        $$result_account" && \
	  echo "subaccount(0,0): $$result_sub" && \
	  [ "$$result_account" = "$$result_sub" ] && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: subaccount(1, 0) differs from subaccount(0, 0) ==="
	@result_sub0=$$(icp canister call --query backend subaccount '(0, 0)') && \
	  result_sub1=$$(icp canister call --query backend subaccount '(1, 0)') && \
	  echo "subaccount(0,0): $$result_sub0" && \
	  echo "subaccount(1,0): $$result_sub1" && \
	  [ "$$result_sub0" != "$$result_sub1" ] && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get_balance returns 0 for unfunded canister ==="
	@result=$$(icp canister call backend get_balance '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qF '(0 : nat64)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: fund canister with 1 ICP — get_balance returns 100_000_000 e8s ==="
	@set -e; \
	  backend=$$(icp canister status backend -i); \
	  icp token transfer 1 "$$backend"; \
	  result=$$(icp canister call backend get_balance '()'); \
	  echo "$$result"; \
	  echo "$$result" | grep -qF '(100000000 : nat64)' && echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: get_balance_of_subaccount(0, 0) also returns 100_000_000 e8s ==="
	@result=$$(icp canister call backend get_balance_of_subaccount '(0, 0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qF '(100000000 : nat64)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7: get_balance_of_subaccount(1, 0) returns 0 (different subaccount) ==="
	@result=$$(icp canister call backend get_balance_of_subaccount '(1, 0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qF '(0 : nat64)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
