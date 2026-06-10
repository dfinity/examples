.PHONY: test

test:
	@echo "=== Test 1: load() returns a non-zero timestamp ==="
	@result=$$(icp canister call backend load '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '[0-9]' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: get_current_query_stats_as_string() returns stats data ==="
	@result=$$(icp canister call backend get_current_query_stats_as_string '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Number of calls' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: query stats include all expected fields ==="
	@result=$$(icp canister call backend get_current_query_stats_as_string '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Number of instructions' && \
	  echo "$$result" | grep -q 'Request payload bytes' && \
	  echo "$$result" | grep -q 'Response payload bytes' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
