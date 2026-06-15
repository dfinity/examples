.PHONY: test

# Note: these tests assume a freshly deployed canister state.
# To re-run from scratch: icp deploy --mode reinstall -y && make test
test:
	@echo "=== Test 1: subscriber registers with publisher for the 'Apples' topic ==="
	@icp canister call subscriber subscribe '("Apples")' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: initial count is zero ==="
	@result=$$(icp canister call --query subscriber getCount '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(0 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: publishing to 'Apples' triggers the callback and increases the count ==="
	@# publish fires callbacks asynchronously; sleep gives the callback time to arrive.
	@icp canister call publisher publish '(record { topic = "Apples"; value = 2 })' && \
	  sleep 2 && \
	  result=$$(icp canister call --query subscriber getCount '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(2 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: publishing to 'Bananas' is ignored (not subscribed) ==="
	@icp canister call publisher publish '(record { topic = "Bananas"; value = 3 })' && \
	  sleep 2 && \
	  result=$$(icp canister call --query subscriber getCount '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(2 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
