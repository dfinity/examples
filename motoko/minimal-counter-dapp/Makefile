NAME = minimal_dapp

.PHONY: all
all: build

.PHONY: node_modules
.SILENT: node_modules
node_modules:
	npm install

.PHONY: build
.SILENT: build
build: node_modules
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --all --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call minimal_dapp_backend increment \
		| grep '(1 : nat)' && echo 'PASS'
	dfx canister call minimal_dapp_backend increment \
		| grep '(2 : nat)' && echo 'PASS'
	dfx canister call minimal_dapp_backend getCount \
		| grep '(2 : nat)' && echo 'PASS'
	dfx canister call minimal_dapp_backend decrement \
		| grep '(1 : nat)' && echo 'PASS'
	dfx canister call minimal_dapp_backend reset \
		| grep '(0 : nat)' && echo 'PASS'
	dfx canister call minimal_dapp_backend decrement \
		| grep '(0 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm -fr node_modules
