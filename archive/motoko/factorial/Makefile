.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create factorial
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install factorial

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install factorial --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call factorial fac '(20)' \
		| grep '(2_432_902_008_176_640_000 : nat)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
