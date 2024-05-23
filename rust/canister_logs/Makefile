.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create canister_logs
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install canister_logs

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install canister_logs --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call canister_logs print 'hi'
	dfx canister logs canister_logs \
		| grep 'hi' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
