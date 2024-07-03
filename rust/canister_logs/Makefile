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
	# Test print via update call.
	dfx canister call canister_logs print 'print via update'
	dfx canister logs canister_logs \
		| grep 'print via update' && echo 'PASS'

	# Test print via replicated query call.
	dfx canister call --update canister_logs print_query 'print via replicated query'
	dfx canister logs canister_logs \
		| grep 'print via replicated query' && echo 'PASS'

	# Test print via non-replicated query call should NOT record the message.
	dfx canister call --query canister_logs print_query 'print via non-replicated query'
	! dfx canister logs canister_logs \
		| grep 'print via non-replicated query' && echo 'PASS'

	# Test trapped call is recorded in logs. 
	# Ignore failed call output (so the test continues) and check the logs to contain the message.
	-dfx canister call canister_logs trap 'trap via update'
	dfx canister logs canister_logs \
		| grep 'trap via update' && echo 'PASS'

	# Test the call with panic is recorded in logs. 
	# Ignore failed call output (so the test continues) and check the logs to contain the message.
	-dfx canister call canister_logs panic 'panic via update'
	dfx canister logs canister_logs \
		| grep 'panic via update' && echo 'PASS'

	# Test call to fail with memory out of bounds.
	# Ignore failed call output (so the test continues) and check the logs to contain the message.
	-dfx canister call canister_logs memory_oob
	dfx canister logs canister_logs \
		| grep 'stable memory out of bounds' && echo 'PASS'

	# Test call to fail with failed unwrap.
	# Ignore failed call output (so the test continues) and check the logs to contain the message.
	-dfx canister call canister_logs failed_unwrap
	dfx canister logs canister_logs \
		| grep 'Result::unwrap()' && echo 'PASS'

	# Test timer trap, assume it's been 5 seconds since the start.
	dfx canister logs canister_logs \
		| grep 'timer trap' && echo 'PASS'

	# TODO: make sure DFX has the fix.
	# Test raw_rand.
	dfx canister call canister_logs raw_rand
	dfx canister logs canister_logs \
		| grep 'ic.raw_rand() call succeeded' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
