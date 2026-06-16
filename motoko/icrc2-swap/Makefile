.PHONY: deploy test

# Deploy all canisters in the correct order:
# 1. token_a and token_b (ICRC-1/2 ledger canisters) with their init args
# 2. backend (swap) with the token canister IDs as init args
#
# Note: use `make deploy` (not `icp deploy`) — the ledger canisters require
# init args that are only provided by this target.
deploy:
	$(eval MINTER := $(shell icp identity principal))
	icp deploy token_a --args "(variant { Init = record { \
	  token_name = \"Token A\"; \
	  token_symbol = \"A\"; \
	  minting_account = record { owner = principal \"$(MINTER)\" }; \
	  initial_balances = vec { \
	    record { record { owner = principal \"$(MINTER)\" }; 100_000_000_000_000 : nat }; \
	  }; \
	  metadata = vec {}; \
	  transfer_fee = 10_000 : nat; \
	  archive_options = record { \
	    trigger_threshold = 2000 : nat64; \
	    num_blocks_to_archive = 1000 : nat64; \
	    controller_id = principal \"$(MINTER)\"; \
	  }; \
	  feature_flags = opt record { icrc2 = true }; \
	}})"
	icp deploy token_b --args "(variant { Init = record { \
	  token_name = \"Token B\"; \
	  token_symbol = \"B\"; \
	  minting_account = record { owner = principal \"$(MINTER)\" }; \
	  initial_balances = vec { \
	    record { record { owner = principal \"$(MINTER)\" }; 100_000_000_000_000 : nat }; \
	  }; \
	  metadata = vec {}; \
	  transfer_fee = 10_000 : nat; \
	  archive_options = record { \
	    trigger_threshold = 2000 : nat64; \
	    num_blocks_to_archive = 1000 : nat64; \
	    controller_id = principal \"$(MINTER)\"; \
	  }; \
	  feature_flags = opt record { icrc2 = true }; \
	}})"
	$(eval TOKEN_A := $(shell icp canister status token_a -i))
	$(eval TOKEN_B := $(shell icp canister status token_b -i))
	icp deploy backend --args "(record { token_a = principal \"$(TOKEN_A)\"; token_b = principal \"$(TOKEN_B)\" })"

# Full integration test: Alice deposits token_a, Bob deposits token_b,
# they swap 1:1, and each withdraws the other's token.
test:
	$(eval TOKEN_A := $(shell icp canister status token_a -i))
	$(eval TOKEN_B := $(shell icp canister status token_b -i))
	$(eval MINTER := $(shell icp identity principal))
	$(eval SWAP := $(shell icp canister status backend -i))

	@# Create test identities (idempotent)
	@icp identity new test-alice 2>/dev/null || true
	@icp identity new test-bob   2>/dev/null || true
	$(eval ALICE := $(shell icp identity principal --identity test-alice))
	$(eval BOB   := $(shell icp identity principal --identity test-bob))

	@# Fund Alice with token_a and Bob with token_b from the minting account
	@icp canister call token_a icrc1_transfer "(record { \
	  to = record { owner = principal \"$(ALICE)\"; subaccount = null }; \
	  amount = 100_020_000 : nat; fee = null; memo = null; \
	  from_subaccount = null; created_at_time = null \
	})"
	@icp canister call token_b icrc1_transfer "(record { \
	  to = record { owner = principal \"$(BOB)\"; subaccount = null }; \
	  amount = 100_020_000 : nat; fee = null; memo = null; \
	  from_subaccount = null; created_at_time = null \
	})"

	@echo "=== Test 1: balances returns empty lists initially ==="
	@result=$$(icp canister call --query backend balances '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec {}' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: Alice approves and deposits token_a ==="
	@icp canister call --identity test-alice token_a icrc2_approve "(record { \
	  amount = 100_010_000 : nat; \
	  spender = record { owner = principal \"$(SWAP)\"; subaccount = null }; \
	  expires_at = null; expected_allowance = null; fee = null; \
	  from_subaccount = null; memo = null; created_at_time = null \
	})"
	@result=$$(icp canister call --identity test-alice backend deposit "(record { \
	  token = principal \"$(TOKEN_A)\"; \
	  from = record { owner = principal \"$(ALICE)\"; subaccount = null }; \
	  amount = 100_000_000 : nat; \
	  spender_subaccount = null; fee = null; memo = null; created_at_time = null \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: Bob approves and deposits token_b ==="
	@icp canister call --identity test-bob token_b icrc2_approve "(record { \
	  amount = 100_010_000 : nat; \
	  spender = record { owner = principal \"$(SWAP)\"; subaccount = null }; \
	  expires_at = null; expected_allowance = null; fee = null; \
	  from_subaccount = null; memo = null; created_at_time = null \
	})"
	@result=$$(icp canister call --identity test-bob backend deposit "(record { \
	  token = principal \"$(TOKEN_B)\"; \
	  from = record { owner = principal \"$(BOB)\"; subaccount = null }; \
	  amount = 100_000_000 : nat; \
	  spender_subaccount = null; fee = null; memo = null; created_at_time = null \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: swap Alice's token_a for Bob's token_b ==="
	@result=$$(icp canister call backend swap "(record { \
	  user_a = principal \"$(ALICE)\"; \
	  user_b = principal \"$(BOB)\" \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: Alice withdraws token_b (received via swap) ==="
	@result=$$(icp canister call --identity test-alice backend withdraw "(record { \
	  token = principal \"$(TOKEN_B)\"; \
	  to = record { owner = principal \"$(ALICE)\"; subaccount = null }; \
	  amount = 100_000_000 : nat; \
	  fee = null; memo = null; created_at_time = null \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 6: Bob withdraws token_a (received via swap) ==="
	@result=$$(icp canister call --identity test-bob backend withdraw "(record { \
	  token = principal \"$(TOKEN_A)\"; \
	  to = record { owner = principal \"$(BOB)\"; subaccount = null }; \
	  amount = 100_000_000 : nat; \
	  fee = null; memo = null; created_at_time = null \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
