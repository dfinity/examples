.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create minimal_dapp
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install minimal_dapp

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install minimal_dapp --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call minimal_dapp count \
		| grep '(1 : nat)' && echo 'PASS'
	dfx canister call minimal_dapp getCount \
		| grep '(1 : nat)' && echo 'PASS'
  dfx canister call minimal_dapp reset \
		| grep '(0 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
