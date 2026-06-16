.PHONY: test

test:
	@echo "=== Test 1: list_neurons_pretty returns a text response ==="
	@result=$$(icp canister call backend list_neurons_pretty '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '(NNS Governance Neurons|Error fetching neurons)' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
