.PHONY: test

# Tests capture the ID from create so they are idempotent across re-runs
# (the backend uses an auto-incrementing counter, so the ID differs each run).
test:
	@echo "=== Test 1: create a superhero ==="
	@set -e; \
	  out=$$(icp canister call backend create '(record { name = "Superman"; superpowers = vec { "invulnerability"; "superhuman strength" } })'); \
	  echo "$$out"; \
	  id=$$(echo "$$out" | grep -oE '[0-9]+' | head -1); \
	  echo "Created ID: $$id"; \
	  echo "PASS"; \
	\
	  echo "=== Test 2: read the created superhero ==="; \
	  result=$$(icp canister call --query backend read "($$id : nat32)"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'Superman' && echo "PASS" || (echo "FAIL" && exit 1); \
	\
	  echo "=== Test 3: update the superhero ==="; \
	  result=$$(icp canister call backend update "($$id : nat32, record { name = \"Superman\"; superpowers = vec { \"invulnerability\"; \"superhuman strength\"; \"flight\"; \"x-ray vision\" } })"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'true' && echo "PASS" || (echo "FAIL" && exit 1); \
	\
	  echo "=== Test 4: read updated superhero to verify state change ==="; \
	  result=$$(icp canister call --query backend read "($$id : nat32)"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'x-ray vision' && echo "PASS" || (echo "FAIL" && exit 1); \
	\
	  echo "=== Test 5: delete the superhero ==="; \
	  result=$$(icp canister call backend delete "($$id : nat32)"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'true' && echo "PASS" || (echo "FAIL" && exit 1); \
	\
	  echo "=== Test 6: delete non-existent superhero returns false ==="; \
	  result=$$(icp canister call backend delete "($$id : nat32)"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'false' && echo "PASS" || (echo "FAIL" && exit 1); \
	\
	  echo "=== Test 7: read deleted superhero returns null ==="; \
	  result=$$(icp canister call --query backend read "($$id : nat32)"); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'null' && echo "PASS" || (echo "FAIL" && exit 1)
