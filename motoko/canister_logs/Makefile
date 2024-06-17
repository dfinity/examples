.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create CanisterLogs
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install CanisterLogs

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install CanisterLogs --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call CanisterLogs print 'hi'
	dfx canister logs CanisterLogs \
		| grep 'hi' && echo 'PASS'

	dfx canister call CanisterLogs trap 'oops'
	dfx canister logs CanisterLogs \
		| grep 'oops' && echo 'PASS'

	dfx canister call CanisterLogs panic 'aaa!'
	dfx canister logs CanisterLogs \
		| grep 'aaa!' && echo 'PASS'

	dfx canister call CanisterLogs raw_rand
	dfx canister logs CanisterLogs \
		| grep 'ic00_raw_rand call succeeded' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
