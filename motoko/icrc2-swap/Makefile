.PHONY: deploy test

# Deploy all canisters in the correct order:
# 1. token_a and token_b (ICRC-1 ledger canisters) with init args
# 2. backend (swap) with the token canister IDs as init args
deploy:
	$(eval MINTER := $(shell icp identity principal))
	icp deploy token_a --argument "(variant { Init = record { \
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
	icp deploy token_b --argument "(variant { Init = record { \
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
	$(eval TOKEN_A := $(shell icp canister id token_a))
	$(eval TOKEN_B := $(shell icp canister id token_b))
	icp deploy backend --argument "(record { token_a = principal \"$(TOKEN_A)\"; token_b = principal \"$(TOKEN_B)\" })"

test:
	$(eval TOKEN_A := $(shell icp canister id token_a))
	$(eval TOKEN_B := $(shell icp canister id token_b))
	$(eval MINTER := $(shell icp identity principal))
	$(eval SWAP := $(shell icp canister id backend))

	@echo "=== Test 1: balances returns empty lists initially ==="
	@result=$$(icp canister call --query backend balances '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'vec {}' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 2: approve and deposit token_a succeeds ==="
	@icp canister call token_a icrc2_approve "(record { \
	  amount = 100_010_000 : nat; \
	  spender = record { owner = principal \"$(SWAP)\"; subaccount = null }; \
	  expires_at = null; \
	  expected_allowance = null; \
	  fee = null; \
	  from_subaccount = null; \
	  memo = null; \
	  created_at_time = null \
	})"
	@result=$$(icp canister call backend deposit "(record { \
	  token = principal \"$(TOKEN_A)\"; \
	  from = record { owner = principal \"$(MINTER)\"; subaccount = null }; \
	  amount = 100_000_000 : nat; \
	  spender_subaccount = null; \
	  fee = null; \
	  memo = null; \
	  created_at_time = null \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 3: balances shows deposited token_a ==="
	@result=$$(icp canister call --query backend balances '()') && \
	  echo "$$result" && \
	  echo "$$result" | grep -qE '100[_,]?000[_,]?000' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 4: swap with same user swaps all balances ==="
	@result=$$(icp canister call backend swap "(record { \
	  user_a = principal \"$(MINTER)\"; \
	  user_b = principal \"$(MINTER)\" \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'ok' && \
	  echo "PASS" || (echo "FAIL" && exit 1)

	@echo "=== Test 5: withdraw with insufficient deposited balance returns InsufficientFunds ==="
	@result=$$(icp canister call backend withdraw "(record { \
	  token = principal \"$(TOKEN_A)\"; \
	  to = record { owner = principal \"$(MINTER)\"; subaccount = null }; \
	  amount = 100_000_000 : nat; \
	  fee = null; \
	  memo = null; \
	  created_at_time = null \
	})") && \
	  echo "$$result" && \
	  echo "$$result" | grep -q 'InsufficientFunds' && \
	  echo "PASS" || (echo "FAIL" && exit 1)
