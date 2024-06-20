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
	# Test print.
	dfx canister call canister_logs print 'print_message'
	dfx canister logs canister_logs \
		| grep 'print_message' && echo 'PASS'

	# Test trap, ignore failed call output.
	-dfx canister call canister_logs trap 'trap_message'
	dfx canister logs canister_logs \
		| grep 'trap_message' && echo 'PASS'

	# Test panic, ignore failed call output.
	-dfx canister call canister_logs panic 'panic_message'
	dfx canister logs canister_logs \
		| grep 'panic_message' && echo 'PASS'

	# Test memory_oob, ignore failed call output.
	-dfx canister call canister_logs memory_oob
	dfx canister logs canister_logs \
		| grep 'stable memory out of bounds' && echo 'PASS'

	# Test failed_unwrap, ignore failed call output.
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
