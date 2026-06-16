.PHONY: test

test:
	@echo "=== Test 1: Reset counter to 0 ==="
	@result=$$(icp canister call counter set '(0 : nat)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '()' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: call_get_and_set returns old value and sets new ==="
	@COUNTER_ID=$$(icp canister id counter) && \
	result=$$(icp canister call caller call_get_and_set "(principal \"$$COUNTER_ID\", 42 : nat)") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(0 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: set_then_get sets value and returns it ==="
	@COUNTER_ID=$$(icp canister id counter) && \
	result=$$(icp canister call caller set_then_get "(principal \"$$COUNTER_ID\", 7 : nat)") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(7 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: counter value persisted after set_then_get ==="
	@result=$$(icp canister call --query counter get '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(7 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: call_increment increments counter ==="
	@COUNTER_ID=$$(icp canister id counter) && \
	result=$$(icp canister call caller call_increment "(principal \"$$COUNTER_ID\")") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: counter value is 8 after increment ==="
	@result=$$(icp canister call --query counter get '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '(8 : nat)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7: call_get returns current counter value ==="
	@COUNTER_ID=$$(icp canister id counter) && \
	result=$$(icp canister call caller call_get "(principal \"$$COUNTER_ID\")") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Ok = 8' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 8: stubborn_set retries until success ==="
	@COUNTER_ID=$$(icp canister id counter) && \
	result=$$(icp canister call caller stubborn_set "(principal \"$$COUNTER_ID\", 42 : nat)") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 9: counter value is 42 after stubborn_set ==="
	@COUNTER_ID=$$(icp canister id counter) && \
	result=$$(icp canister call caller call_get "(principal \"$$COUNTER_ID\")") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Ok = 42' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 10: send_cycles increases counter canister balance ==="
	@COUNTER_ID=$$(icp canister id counter) && \
	result=$$(icp canister call caller send_cycles "(principal \"$$COUNTER_ID\", 1_000_000_000_000 : nat64)") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
