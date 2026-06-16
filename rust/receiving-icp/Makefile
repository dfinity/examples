.PHONY: test

test:
	@echo "=== Test 1: account returns a non-empty account identifier ==="
	@result=$$(icp canister call --query backend account '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: subaccount(0, 0) returns same account as account() ==="
	@result_account=$$(icp canister call --query backend account '()') && \
	  result_sub=$$(icp canister call --query backend subaccount '(0, 0)') && \
	  echo "account: $$result_account" && \
	  echo "subaccount(0,0): $$result_sub" && \
	  [ "$$result_account" = "$$result_sub" ] && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: subaccount(1, 0) returns a different account than subaccount(0, 0) ==="
	@result_sub0=$$(icp canister call --query backend subaccount '(0, 0)') && \
	  result_sub1=$$(icp canister call --query backend subaccount '(1, 0)') && \
	  echo "subaccount(0,0): $$result_sub0" && \
	  echo "subaccount(1,0): $$result_sub1" && \
	  [ "$$result_sub0" != "$$result_sub1" ] && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get_balance returns a numeric balance (0 for fresh canister) ==="
	@result=$$(icp canister call backend get_balance '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '0' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: get_balance_of_subaccount(0, 0) returns 0 for fresh canister ==="
	@result=$$(icp canister call backend get_balance_of_subaccount '(0, 0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '0' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
