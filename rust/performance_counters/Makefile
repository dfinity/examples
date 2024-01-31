.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install performance_counters

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install performance_counters --mode=upgrade

.PHONY: deploy
.SILENT: deploy
deploy:
	# Deploy the canisters and run periodic tasks with 1s interval.
	dfx deploy performance_counters

.PHONY: test
.SILENT: test
test: deploy
	# Wait at least 2 seconds.
	sleep 2
	# Validate the counters are non-zero.
	dfx canister call performance_counters for_update | grep -vw '0' && echo 'PASS'
	dfx canister call performance_counters for_composite_query | grep -vw '0' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
