.PHONY: test

test:
	@echo "=== Test 1: set items and verify they are not yet processed ==="
	@icp canister call backend set_non_processed_items '(vec { "mint" })' && \
	  result=$$(icp canister call --query backend is_item_processed '("mint")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'false' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: guard marks item as processed despite panicking callback (TrueAsyncCall) ==="
	@icp canister call backend set_non_processed_items '(vec { "mint" })' && \
	  icp canister call backend process_single_item_with_panicking_callback '("mint", variant { TrueAsyncCall })' 2>/dev/null; \
	  result=$$(icp canister call --query backend is_item_processed '("mint")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'true' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: guard fails to mark item as processed when no true async call (FalseAsyncCall) ==="
	@icp canister call backend set_non_processed_items '(vec { "mint" })' && \
	  icp canister call backend process_single_item_with_panicking_callback '("mint", variant { FalseAsyncCall })' 2>/dev/null; \
	  result=$$(icp canister call --query backend is_item_processed '("mint")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'false' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: guard marks first item as processed when processing multiple items (TrueAsyncCall) ==="
	@icp canister call backend set_non_processed_items '(vec { "mint1"; "mint2"; "mint3" })' && \
	  icp canister call backend process_all_items_with_panicking_callback '("mint2", variant { TrueAsyncCall })' 2>/dev/null; \
	  result1=$$(icp canister call --query backend is_item_processed '("mint1")') && \
	  result2=$$(icp canister call --query backend is_item_processed '("mint2")') && \
	  echo "mint1: $$result1, mint2: $$result2" && \
	  echo "$$result1" | grep -q 'true' && \
	  echo "$$result2" | grep -q 'false' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: set_non_processed_items resets state ==="
	@icp canister call backend set_non_processed_items '(vec { "mint" })' && \
	  icp canister call backend process_single_item_with_panicking_callback '("mint", variant { TrueAsyncCall })' 2>/dev/null; \
	  icp canister call backend set_non_processed_items '(vec { "mint" })' && \
	  result=$$(icp canister call --query backend is_item_processed '("mint")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'false' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
