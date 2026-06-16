.PHONY: test

test:
	@echo "=== Test 1: list_images returns empty list initially ==="
	@result=$$(icp canister call --query backend list_images '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec {}' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: upload_image stores an image and returns an ID ==="
	@result=$$(icp canister call backend upload_image '("test.png", "image/png", blob "\89\50\4e\47")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '^\(([0-9]+) : nat64\)$$' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: list_images includes the uploaded image ==="
	@result=$$(icp canister call --query backend list_images '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'test.png' && \
	  echo "$$result" | grep -q 'image/png' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: upload a second image and list_images returns both ==="
	@icp canister call backend upload_image '("photo.jpg", "image/jpeg", blob "\ff\d8\ff")' > /dev/null
	@result=$$(icp canister call --query backend list_images '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'test.png' && \
	  echo "$$result" | grep -q 'photo.jpg' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: http_request serves uploaded image by ID ==="
	@result=$$(icp canister call --query backend http_request '(record { method = "GET"; url = "/image/1"; headers = vec {}; body = blob "" })') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '200' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: http_request returns 404 for missing image ==="
	@result=$$(icp canister call --query backend http_request '(record { method = "GET"; url = "/image/9999"; headers = vec {}; body = blob "" })') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '404' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
