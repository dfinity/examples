.PHONY: test

test:
	@echo "=== Test 1: get_p2pkh_address returns a valid address ==="
	@result=$$(icp canister call backend get_p2pkh_address '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get_p2tr_key_only_address returns a valid address ==="
	@result=$$(icp canister call backend get_p2tr_key_only_address '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: get_p2tr_address returns a valid address ==="
	@result=$$(icp canister call backend get_p2tr_address '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '"' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: get_balance returns a nat64 (may be 0 on local replica) ==="
	@addr=$$(icp canister call backend get_p2pkh_address '()' | tr -d '()"\n ') && \
	  result=$$(icp canister call backend get_balance "(\"$$addr\")") && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '[0-9]' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: get_current_fee_percentiles returns a vec ==="
	@result=$$(icp canister call backend get_current_fee_percentiles '()') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)
