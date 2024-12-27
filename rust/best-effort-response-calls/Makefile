.PHONY: all
all: test

.PHONY: deploy
.SILENT: deploy
build:
	dfx deploy

.PHONY: test
.SILENT: test
test: build
	dfx canister call berc_backend demonstrate_deadlines true | grep -E '\([1-9][0-9_]* : nat64\)' && echo 'PASS'
	dfx canister call berc_backend demonstrate_deadlines false | grep '(0 : nat64)' && echo 'PASS'
	dfx canister call berc_backend demonstrate_timeouts | grep '(true)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx