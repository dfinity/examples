.PHONY: test

test:
	@echo "=== Test 1: get_count returns 0 initially ==="
	@result=$$(icp canister call --query backend get_count '(record {})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'count = opt (0' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: increment_count returns new_count = 1 ==="
	@result=$$(icp canister call backend increment_count '(record {})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'new_count = opt (1' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: get_count returns 1 after increment ==="
	@result=$$(icp canister call --query backend get_count '(record {})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'count = opt (1' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: increment_count again returns new_count = 2 ==="
	@result=$$(icp canister call backend increment_count '(record {})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'new_count = opt (2' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: decrement_count returns new_count = 1 ==="
	@result=$$(icp canister call backend decrement_count '(record {})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'new_count = opt (1' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: get_count returns 1 after decrement ==="
	@result=$$(icp canister call --query backend get_count '(record {})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'count = opt (1' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7: get_proposal_info with missing proposal_id returns error ==="
	@result=$$(icp canister call backend get_proposal_info '(record { proposal_id = null })') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Missing proposal_id' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 8: get_proposal_titles returns titles list ==="
	@result=$$(icp canister call backend get_proposal_titles '(record { limit = null })') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'titles' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
