.PHONY: all
all: test

.PHONY: deploy
.SILENT: deploy
build:
	dfx deploy --no-wallet

.PHONY: test
.SILENT: test
test: build
	dfx canister call caller setup_callee "(principal \"`dfx canister id callee`\")"
	dfx canister call caller sequential_calls 100 | grep '(100 : nat)' && echo 'PASS'
	dfx canister call caller parallel_calls 100 | grep '(100 : nat)' && echo 'PASS'
	dfx canister call caller sequential_calls 2000 | grep '(2_000 : nat)' && echo 'PASS' 
	dfx canister call caller parallel_calls 2000 | grep '(500 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
