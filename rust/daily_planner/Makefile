.PHONY: test

test:
	@echo "=== Test 1/6: get_day_data returns null for a date with no notes ==="
	@result=$$(icp canister call --query backend get_day_data '("2000-01-15")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'null' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/6: add_note returns ok result ==="
	@result=$$(icp canister call backend add_note '("2000-01-15", "Buy groceries")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3/6: get_day_data returns stored note ==="
	@result=$$(icp canister call --query backend get_day_data '("2000-01-15")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Buy groceries' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4/6: get_month_data returns entry for the stored month ==="
	@result=$$(icp canister call --query backend get_month_data '(2000 : nat, 1 : nat)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Buy groceries' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5/6: complete_note marks note as completed ==="
	@icp canister call backend complete_note '("2000-01-15", 0 : nat)' && \
	  result=$$(icp canister call --query backend get_day_data '("2000-01-15")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'is_completed = true' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6/6: get_month_data returns empty list for a different month ==="
	@result=$$(icp canister call --query backend get_month_data '(1999 : nat, 12 : nat)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec {}' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
