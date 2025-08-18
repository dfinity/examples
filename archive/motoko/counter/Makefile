.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create counter
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install counter

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install counter --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call counter set '(7)'
	dfx canister call counter inc
	dfx canister call counter get \
		| grep '(8 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
