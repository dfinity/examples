.PHONY: configure test

# Set wasm_memory_limit to 5 MiB and wasm_memory_threshold to 2 MiB.
# The low Wasm memory hook fires when remaining Wasm memory falls below the
# threshold (i.e. when usage exceeds limit - threshold = 3 MiB).
configure:
	icp canister settings update backend \
	  --wasm-memory-limit 5mib \
	  --wasm-memory-threshold 2mib \
	  -f

test: configure
	@echo "=== Polling for onLowWasmMemory hook (up to 60s) ==="
	@secs=0; \
	while [ $$secs -lt 60 ]; do \
	  result=$$(icp canister call --query backend getExecutedFunctionsOrder '()'); \
	  echo "$$result"; \
	  echo "$$result" | grep -q 'onLowWasmMemory' && echo "PASS" && exit 0; \
	  sleep 3; secs=$$((secs + 3)); \
	done; \
	echo "FAIL: onLowWasmMemory hook did not fire within 60s"; exit 1
