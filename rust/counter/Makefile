.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
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
	cargo test
	(  dfx canister call counter set '(7 : nat)' \
	&& dfx canister call counter inc \
	&& dfx canister call counter get \
	) | grep '(8 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
