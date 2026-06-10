.PHONY: test

test:
	@echo "=== Test 1/4: is_high_score returns true when leaderboard is empty ==="
	@result=$$(icp canister call backend is_high_score '(100 : nat)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'true' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/4: add_leaderboard_entry returns leaderboard with new entry ==="
	@result=$$(icp canister call backend add_leaderboard_entry '("Alice", 100 : nat)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Alice' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3/4: get_leaderboard returns stored entries ==="
	@result=$$(icp canister call backend get_leaderboard '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Alice' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4/4: get_randomness returns a blob ==="
	@result=$$(icp canister call backend get_randomness '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
