PWD:=$(shell pwd)

.PHONY: compile-wasm
.SILENT: compile-wasm
compile-wasm:
	icp build

# Test the APIs of this canister using the respective Rust canister tests.
# This has the advantage that the tests are consistent (less room for bugs by having only one implementation of the tests) and the checked expected behavior is consistent across Rust and Motoko.
.PHONY: test
.SILENT: test
test: compile-wasm
	@echo "Testing Motoko canister WASM: $(PWD)/.icp/cache/artifacts/ic_vetkeys_encrypted_maps_canister"
	CUSTOM_WASM_PATH=$(PWD)/.icp/cache/artifacts/ic_vetkeys_encrypted_maps_canister cargo test -p ic-vetkeys-encrypted-maps-canister
