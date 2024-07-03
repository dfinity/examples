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
	# Test print via update call.
	dfx canister call CanisterLogs print 'print via update call'
	dfx canister logs CanisterLogs \
		| grep 'print via update call' && echo 'PASS'

	# Test print via replicated query call.
	dfx canister call CanisterLogs print_query 'print via replicated query'
	dfx canister logs CanisterLogs \
		| grep 'print via replicated query' && echo 'PASS'

	# Test trap, ignore failed call output.
	-dfx canister call CanisterLogs trap 'trap_message'
	dfx canister logs CanisterLogs \
		| grep 'trap_message' && echo 'PASS'

	# Test memory_oob, ignore failed call output.
	-dfx canister call CanisterLogs memory_oob
	dfx canister logs CanisterLogs \
		| grep 'StableMemory range out of bounds' && echo 'PASS'

	# Test timer trap, assume it's been 5 seconds since the start.
	sleep 2
	dfx canister logs CanisterLogs \
		| grep 'timer trap' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
