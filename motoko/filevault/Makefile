.PHONY: test

test:
	@echo "=== Test 1: getFiles returns empty list for new user ==="
	@result=$$(icp canister call backend getFiles '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec {}' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: checkFileExists returns false for non-existent file ==="
	@result=$$(icp canister call backend checkFileExists '("test.txt")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'false' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: uploadFileChunk stores a file chunk ==="
	@result=$$(icp canister call backend uploadFileChunk '("test.txt", blob "\68\65\6c\6c\6f", 0, "text/plain")') && \
	  echo "$$result" && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: checkFileExists returns true after upload ==="
	@result=$$(icp canister call backend checkFileExists '("test.txt")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'true' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: getFiles shows uploaded file ==="
	@result=$$(icp canister call backend getFiles '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'test.txt' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: getTotalChunks returns 1 after single chunk upload ==="
	@result=$$(icp canister call backend getTotalChunks '("test.txt")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q '1' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 7: getFileChunk returns chunk data ==="
	@result=$$(icp canister call backend getFileChunk '("test.txt", 0)') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'opt' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 8: getFileType returns file type ==="
	@result=$$(icp canister call backend getFileType '("test.txt")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'text/plain' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 9: deleteFile removes the file ==="
	@result=$$(icp canister call backend deleteFile '("test.txt")') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'true' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 10: getFiles is empty after deletion ==="
	@result=$$(icp canister call backend getFiles '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec {}' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
