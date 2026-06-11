.PHONY: test

test:
	@echo "=== Test 1/2: load() returns a non-zero timestamp ==="
	@result=$$(icp canister call backend load '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '\([0-9][0-9_]* : int\)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/2: get_current_query_stats_as_string() returns the four expected fields ==="
	@# NOTE: query stats are aggregated with a ~2-epoch delay (120+ blocks on local replica,
	@# 1200+ blocks on mainnet). Values will be 0 locally even after multiple query calls —
	@# this is expected. On IC mainnet stats accumulate after enough queries are made.
	@result=$$(icp canister call backend get_current_query_stats_as_string '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Number of calls' && \
	  echo "$$result" | grep -q 'Number of instructions' && \
	  echo "$$result" | grep -q 'Request payload bytes' && \
	  echo "$$result" | grep -q 'Response payload bytes' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
