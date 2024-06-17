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

	dfx canister call canister_logs trap 'oops'
	dfx canister logs canister_logs \
		| grep 'oops' && echo 'PASS'

	dfx canister call canister_logs panic 'aaa!'
	dfx canister logs canister_logs \
		| grep 'aaa!' && echo 'PASS'

	dfx canister call canister_logs raw_rand
	dfx canister logs canister_logs \
		| grep 'ic00_raw_rand call succeeded' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
