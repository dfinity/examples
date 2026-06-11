.PHONY: test test-stats

# Fast structural test — verifies the API shape and that load() works.
# Query stats will show 0 due to the 2-epoch aggregation delay; use
# `make test-stats` to wait for aggregation and verify non-zero values.
test:
	@echo "=== Test 1/2: load() returns a non-zero timestamp ==="
	@# --query ensures this call is recorded in query_stats (update calls are not tracked)
	@result=$$(icp canister call --query backend load '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '\([0-9][0-9_]* : int\)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2/2: get_current_query_stats_as_string() returns the four expected fields ==="
	@result=$$(icp canister call backend get_current_query_stats_as_string '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Number of calls' && \
	  echo "$$result" | grep -q 'Number of instructions' && \
	  echo "$$result" | grep -q 'Request payload bytes' && \
	  echo "$$result" | grep -q 'Response payload bytes' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

# Full demonstration — generates query load, waits for the 2-epoch aggregation
# window (~15s), then verifies non-zero stats.
#
# Query stats require 120+ blocks to aggregate (2 epochs × 60 blocks × 100ms = 12s minimum).
# Only --query calls count; update calls are not tracked in query_stats.
test-stats:
	@echo "=== Generating query load (13 calls) ==="
	@# PocketIC simulates a 13-node subnet and uses integer division when spreading
	@# stats across nodes (num_calls /= 13). Fewer than 13 calls rounds to 0.
	@for i in $$(seq 1 13); do \
	  icp canister call --query backend load '()' > /dev/null; \
	done
	@echo "=== Waiting for 2-epoch aggregation (up to 60s) ==="
	@# Each epoch = 60 blocks; auto-progress advances blocks every 100ms minimum.
	@# On slower machines each block may take 150-200ms: 120 blocks × 200ms = 24s worst case.
	@secs=0; \
	while [ $$secs -lt 60 ]; do \
	  result=$$(icp canister call backend get_current_query_stats_as_string '()'); \
	  echo "$$result" | grep -qE 'Number of calls: [1-9]' && \
	    echo "$$result" && echo "PASS: query stats are non-zero (after ~$${secs}s)" && exit 0; \
	  sleep 3; \
	  secs=$$(($$secs + 3)); \
	done; \
	result=$$(icp canister call backend get_current_query_stats_as_string '()'); \
	echo "$$result"; \
	echo "FAIL: stats still 0 after 60s" && exit 1
