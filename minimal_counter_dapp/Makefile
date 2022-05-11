.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister --no-wallet create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister --no-wallet  install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister --no-wallet  install --all --mode=reinstall

.PHONY: test
.SILENT: test
test: upgrade
	dfx canister call minimal_counter_dapp get \
		| grep '(0 : nat)' && echo 'PASS'
	dfx canister call minimal_counter_dapp increment \
		| grep '(1 : nat)' && echo 'PASS'
	sleep 2 # Wait for update.
	dfx canister call minimal_counter_dapp set '(2)' \
		| grep '()' && echo 'PASS'
	sleep 2 # Wait for update.
	dfx canister call minimal_counter_dapp get \
		| grep '(2 : nat)' && echo 'PASS'
		
.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
