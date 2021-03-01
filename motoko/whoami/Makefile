.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create whoami
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install whoami

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install whoami --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call whoami whoami \
		| grep "$(dfx identity get-principal)" && echo 'PASS'
	dfx canister call whoami whoareyou \
		| grep "$(dfx canister id whoami)" && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
