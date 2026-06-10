.PHONY: test

test:
	@echo "=== Test 1: getDayData returns null for an empty date ==="
	@result=$$(icp canister call backend getDayData '("2026-06-10")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'null' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: addNote returns ok result ==="
	@result=$$(icp canister call backend addNote '("2026-06-10", "Buy groceries")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: getDayData returns stored note ==="
	@result=$$(icp canister call backend getDayData '("2026-06-10")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Buy groceries' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: getMonthData returns entry for the current month ==="
	@result=$$(icp canister call backend getMonthData '(2026, 6)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Buy groceries' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: completeNote marks note as completed ==="
	@icp canister call backend completeNote '("2026-06-10", 0)' && \
	  result=$$(icp canister call backend getDayData '("2026-06-10")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'isCompleted = true' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: getMonthData returns empty list for a different month ==="
	@result=$$(icp canister call backend getMonthData '(2025, 1)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec {}' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
