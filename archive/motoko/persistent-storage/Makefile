.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create persistent_storage
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install persistent_storage

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install persistent_storage --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call persistent_storage increment \
		| grep '(1 : nat)' && echo 'PASS'
	dfx canister call persistent_storage get \
		| grep '(1 : nat)' && echo 'PASS'
  dfx canister call persistent_storage reset \
		| grep '(0 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
