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
	# Perform calls, ignore output.
	-dfx canister call CanisterLogs print 'print_message'
	-dfx canister call CanisterLogs trap 'trap_message'
	-dfx canister call CanisterLogs raw_rand
	sleep 5  # TODO: remove this after dfx gets new release.

	# Capture logs and check for expected strings.
	logs=$$(dfx canister logs CanisterLogs) && \
	echo "$$logs" && \
	echo "===" && \
	echo "$$logs" | grep 'print_message' && \
	echo "$$logs" | grep 'trap_message' && \
	echo "$$logs" | grep 'ic.raw_rand() call succeeded' && \
	echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
