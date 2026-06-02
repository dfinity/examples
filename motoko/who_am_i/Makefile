.PHONY: test

test:
	@echo "--- Testing whoami returns a principal ---"
	@result=$$(icp canister call backend whoami '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'principal' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "--- Testing whoami is deterministic ---"
	@result1=$$(icp canister call backend whoami '()') && \
	  result2=$$(icp canister call backend whoami '()') && \
	  [ "$$result1" = "$$result2" ] && \
	  echo "PASS: $$result1" || (echo "FAIL: $$result1 != $$result2" && exit 1)
