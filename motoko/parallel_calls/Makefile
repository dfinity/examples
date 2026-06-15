.PHONY: test test-multi-subnet

test:
	@echo "=== Test 1: sequential_calls(10) — all 10 succeed ==="
	@result=$$(icp canister call caller sequential_calls '(10)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(10 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: parallel_calls(10) — all 10 succeed (same result as sequential at low n) ==="
	@result=$$(icp canister call caller parallel_calls '(10)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(10 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: sequential_calls(2000) — all succeed (one in-flight at a time) ==="
	@result=$$(icp canister call caller sequential_calls '(2000)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(2_000 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: parallel_calls(2000) — fewer succeed (in-flight limit exceeded) ==="
	@# The replica limits in-flight calls per canister, so most parallel calls fail at n=2000.
	@# Extract the number and assert it is strictly less than 2000.
	@result=$$(icp canister call caller parallel_calls '(2000)') && \
	  echo "$$result" && \
	  n=$$(echo "$$result" | grep -oE '[0-9_]+' | head -1 | tr -d '_') && \
	  [ "$$n" -lt 2000 ] && \
	  echo "PASS" || (echo "FAIL" && exit 1)

test-multi-subnet:
	@echo "=== Building Motoko WASMs ==="
	icp build
	@echo "=== Running multi-subnet PocketIC test ==="
	CALLER_WASM=.mops/.build/caller.wasm \
	CALLEE_WASM=.mops/.build/callee.wasm \
	  cargo run --manifest-path multi_subnet/Cargo.toml
