.PHONY: all
all: test

.PHONY: deploy
.SILENT: deploy
build:
	dfx deploy

.PHONY: test
.SILENT: test
test: build
	dfx canister call berc_backend demonstrate_deadlines true | grep -E '\([1-9][0-9]* : nat64\)' && echo 'PASS'
	dfx canister call berc_backend demonstrate_deadlines false | grep '(0 : nat64)' && echo 'PASS'
	dfx canister call berc_backend demonstrate_timeouts false | grep '(true : bool)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx