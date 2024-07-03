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
	dfx canister call CanisterLogs print 'print via update'
	dfx canister logs CanisterLogs \
		| grep 'print via update' && echo 'PASS'

	# Test print via replicated query call.
	dfx canister call --update CanisterLogs print_query 'print via replicated query'
	dfx canister logs CanisterLogs \
		| grep 'print via replicated query' && echo 'PASS'

	# Test print via non-replicated query call should NOT record the message.
	dfx canister call --query CanisterLogs print_query 'print via non-replicated query'
	! dfx canister logs CanisterLogs \
		| grep 'print via non-replicated query' && echo 'PASS'

	# Test trapped call is recorded in logs. 
	# Ignore failed call output (so the test continues) and check the logs to contain the message.
	-dfx canister call CanisterLogs trap 'trap via update'
	dfx canister logs CanisterLogs \
		| grep 'trap via update' && echo 'PASS'

	# Test call to fail with memory out of bounds.
	# Ignore failed call output (so the test continues) and check the logs to contain the message.
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
