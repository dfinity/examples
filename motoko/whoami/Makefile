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
	dfx canister install whoami \
		--argument='(principal "2mxjj-pyyts-rk2hl-2xyka-avylz-dfama-pqui5-pwrhx-wtq2x-xl5lj-qqe")'

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install whoami \
		--argument='(principal "2mxjj-pyyts-rk2hl-2xyka-avylz-dfama-pqui5-pwrhx-wtq2x-xl5lj-qqe")' \
		--mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call whoami whoami \
		| grep "$(dfx identity get-principal)" && echo 'PASS'
	dfx canister call whoami id \
		| grep "$(dfx canister id whoami)" && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
