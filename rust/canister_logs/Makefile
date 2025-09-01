.PHONY: all
all: deploy

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy canister_logs

.PHONY: test
.SILENT: test
test: deploy
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
	# Ignore failed dfx command output (so the test continues) and check the logs to contain the message.
	- dfx canister call canister_logs trap 'trap via update'
	dfx canister logs canister_logs \
		| grep 'trap via update' && echo 'PASS'

	# Test trap via replicated query call.
	# Ignore failed dfx command output (so the test continues) and check the logs to contain the message.
	- dfx canister call --update canister_logs trap_query 'trap via replicated query'
	dfx canister logs canister_logs \
		| grep 'trap via replicated query' && echo 'PASS'

	# Test trap via non-replicated query call should NOT record the message.
	# Ignore failed dfx command output (so the test continues) and check the logs to contain the message.
	- dfx canister call --query canister_logs trap_query 'trap via non-replicated query'
	! dfx canister logs canister_logs \
		| grep 'trap via non-replicated query' && echo 'PASS'

	# Test the call with panic is recorded in logs. 
	# Ignore failed dfx command output (so the test continues) and check the logs to contain the message.
	- dfx canister call canister_logs panic 'panic via update'
	dfx canister logs canister_logs \
		| grep 'panic via update' && echo 'PASS'

	# Test call to fail with memory out of bounds.
	# Ignore failed dfx command output (so the test continues) and check the logs to contain the message.
	- dfx canister call canister_logs memory_oob
	dfx canister logs canister_logs \
		| grep 'stable memory out of bounds' && echo 'PASS'

	# Test call to fail with failed unwrap.
	# Ignore failed dfx command output (so the test continues) and check the logs to contain the message.
	- dfx canister call canister_logs failed_unwrap
	dfx canister logs canister_logs \
		| grep 'Result::unwrap()' && echo 'PASS'

	# Test timer trap.
	# The timer is setup to trap every 5 seconds, so this test has to be called 
	# at least 5 seconds after the start to record the trap log message.
	sleep 5
	dfx canister logs canister_logs \
		| grep 'timer trap' && echo 'PASS'

	# Test raw_rand.
	dfx canister call canister_logs raw_rand
	dfx canister logs canister_logs \
		| grep 'ic.raw_rand() call succeeded' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
