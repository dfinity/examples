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
	dfx canister install heartbeat --argument 1
	dfx canister install timer --argument 1

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install heartbeat --mode=upgrade --argument 1
	dfx canister install timer --mode=upgrade --argument 1

.PHONY: deploy
.SILENT: deploy
deploy:
	# Deploy the canisters and run periodic tasks with 1s interval.
	dfx deploy heartbeat --argument 1
	dfx deploy timer --argument 1

.PHONY: test
.SILENT: test
test: deploy
	# Wait at least 2 seconds.
	sleep 2
	# Validate the counters are non-zero.
	dfx canister call heartbeat counter | grep -vw '0' && echo 'PASS'
	dfx canister call timer counter | grep -vw '0' && echo 'PASS'
	# Validate the cycles used for periodic tasks are non-zero.
	dfx canister call heartbeat cycles_used | grep -vw '0' && echo 'PASS'
	dfx canister call timer cycles_used | grep -vw '0' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
