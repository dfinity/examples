.PHONY: test test-stats

# Fast structural test — verifies the API shape and that load() works.
# Query stats will show 0 due to the 2-epoch aggregation delay; use
# `make test-stats` to wait for aggregation and verify non-zero values.
test:
	@echo "=== Test 1/2: load() returns a non-zero timestamp ==="
	@# --query ensures this call is recorded in query_stats (update calls are not tracked)
	@result=$$(icp canister call --query backend load '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '\([0-9][0-9_]* : nat64\)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/2: get_query_stats() returns the four expected fields ==="
	@result=$$(icp canister call backend get_query_stats '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Number of calls' && \
	  echo "$$result" | grep -q 'Number of instructions' && \
	  echo "$$result" | grep -q 'Request payload bytes' && \
	  echo "$$result" | grep -q 'Response payload bytes' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

# Full demonstration — continuously makes query calls until stats become non-zero.
#
# Key requirements:
# 1. Only --query calls count (update calls not tracked in query_stats)
# 2. PocketIC uses integer division (num_calls /= 13 nodes) — need 13+ calls per round
# 3. set_epoch_from_height is only called DURING query execution, so queries must
#    keep running to flush stats when epoch boundaries are crossed
test-stats:
	@echo "=== Polling query stats (making 13 --query calls every 3s, up to 30s) ==="
	@secs=0; \
	while [ $$secs -lt 30 ]; do \
	  for i in $$(seq 1 13); do \
	    icp canister call --query backend load '()' > /dev/null; \
	  done; \
	  result=$$(icp canister call backend get_query_stats '()'); \
	  echo "$$result" | grep -qE 'Number of calls: [1-9]' && \
	    echo "$$result" && echo "PASS: query stats are non-zero (after ~$${secs}s)" && exit 0; \
	  sleep 3; \
	  secs=$$(($$secs + 3)); \
	done; \
	result=$$(icp canister call backend get_query_stats '()'); \
	echo "$$result"; \
	echo "FAIL: stats still 0 after 30s" && exit 1
