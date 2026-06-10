.PHONY: test topup

topup:
	icp canister top-up --amount 30t backend

test:
	@echo "--- Testing canister factory ---"
	@echo ""
	@echo "[1/5] newActorClass: creates a Counter canister via actor class"
	@result=$$(icp canister call backend newActorClass '(2_000_000_000_000)') && \
	  echo "  $$result" && \
	  echo "$$result" | grep -q 'principal' && \
	  echo "  PASS" || (echo "  FAIL" && exit 1)
	@echo ""
	@echo "[2/5] installActorClass: two-step canister creation"
	@result=$$(icp canister call backend installActorClass '(2_000_000_000_000)') && \
	  echo "  $$result" && \
	  echo "$$result" | grep -q 'principal' && \
	  echo "  PASS" || (echo "  FAIL" && exit 1)
	@echo ""
	@echo "[3/5] createAndInstallCanisterManually: low-level creation with empty WASM"
	@result=$$(icp canister call backend createAndInstallCanisterManually '(2_000_000_000_000)') && \
	  echo "  $$result" && \
	  echo "$$result" | grep -q 'principal' && \
	  echo "  PASS" || (echo "  FAIL" && exit 1)
	@echo ""
	@echo "[4/5] upgradeActorClass: Counter → CounterV2, state PRESERVED (value stays 52)"
	@canister_id=$$(icp canister call backend newActorClass '(2_000_000_000_000)' | grep -o '"[^"]*"' | tr -d '"') && \
	  echo "  created: $$canister_id" && \
	  add_result=$$(icp canister call "$$canister_id" addToValue '(10)') && \
	  echo "  addToValue(10): $$add_result (expected: 52)" && \
	  echo "$$add_result" | grep -q '52' && \
	  icp canister call backend upgradeActorClass "(principal \"$$canister_id\")" > /dev/null && \
	  result=$$(icp canister call "$$canister_id" getValue '()') && \
	  echo "  getValue after upgrade: $$result (expected: 52)" && \
	  echo "$$result" | grep -q '52' && \
	  echo "  PASS" || (echo "  FAIL" && exit 1)
	@echo ""
	@echo "[5/5] reinstallActorClass: Counter → CounterV2, state RESET (value back to 42)"
	@canister_id=$$(icp canister call backend newActorClass '(2_000_000_000_000)' | grep -o '"[^"]*"' | tr -d '"') && \
	  echo "  created: $$canister_id" && \
	  add_result=$$(icp canister call "$$canister_id" addToValue '(10)') && \
	  echo "  addToValue(10): $$add_result (expected: 52)" && \
	  echo "$$add_result" | grep -q '52' && \
	  icp canister call backend reinstallActorClass "(principal \"$$canister_id\")" > /dev/null && \
	  result=$$(icp canister call "$$canister_id" getValue '()') && \
	  echo "  getValue after reinstall: $$result (expected: 42)" && \
	  echo "$$result" | grep -q '42' && \
	  echo "  PASS" || (echo "  FAIL" && exit 1)
