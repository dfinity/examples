.PHONY: test

test:
	@echo "=== Test 1: qrcode_query without logo or gradient ==="
	@result=$$(icp canister call --query backend qrcode_query '("test", record {add_logo=false; add_gradient=false; add_transparency=null})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Image = blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: qrcode_query with logo, no gradient ==="
	@result=$$(icp canister call --query backend qrcode_query '("test", record {add_logo=true; add_gradient=false; add_transparency=null})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Image = blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: qrcode_query with gradient, no logo ==="
	@result=$$(icp canister call --query backend qrcode_query '("test", record {add_logo=false; add_gradient=true; add_transparency=null})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Image = blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: qrcode_query with logo and gradient ==="
	@result=$$(icp canister call --query backend qrcode_query '("test", record {add_logo=true; add_gradient=true; add_transparency=null})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Image = blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: qrcode_query with transparency enabled ==="
	@result=$$(icp canister call --query backend qrcode_query '("test", record {add_logo=true; add_gradient=true; add_transparency=opt true})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Image = blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: qrcode_query with transparency disabled ==="
	@result=$$(icp canister call --query backend qrcode_query '("test", record {add_logo=true; add_gradient=true; add_transparency=opt false})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Image = blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7: qrcode update call (waits for consensus) ==="
	@result=$$(icp canister call backend qrcode '("test", record {add_logo=true; add_gradient=true; add_transparency=null})') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'Image = blob' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
