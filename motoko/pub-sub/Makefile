.PHONY: test

test:
	@echo "=== Test 1: subscriber initializes and registers with publisher ==="
	@icp canister call subscriber init '("Apples")' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: initial count is zero ==="
	@result=$$(icp canister call subscriber getCount '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(0 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: publish to subscribed topic increases count ==="
	@icp canister call publisher publish '(record { topic = "Apples"; value = 2 })' && \
	  sleep 2 && \
	  result=$$(icp canister call subscriber getCount '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(2 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: publish to different topic does not increase count ==="
	@icp canister call publisher publish '(record { topic = "Bananas"; value = 3 })' && \
	  sleep 2 && \
	  result=$$(icp canister call subscriber getCount '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(2 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
