.PHONY: all
all: test

.PHONY: deploy
.SILENT: deploy
build:
	dfx deploy
SHELL := /bin/bash
.SHELLFLAGS := -euo pipefail -c
.PHONY: test
.SILENT: test
test: build
	dfx canister call counter set "( 0 : nat )" | grep '()' && echo 'PASS'
	dfx canister call caller call_get_and_set "( principal \"`dfx canister id counter`\", 42 : nat )" | grep '(0 : nat)' && echo 'PASS'
	dfx canister call caller set_then_get "( principal \"`dfx canister id counter`\", 7 : nat )" | grep '(7 : nat)' && echo 'PASS'
	dfx canister call counter get | grep '(7 : nat)' && echo 'PASS'
	dfx canister call caller call_increment "( principal \"`dfx canister id counter`\" )" | grep '(variant { Ok })' && echo 'PASS'
	dfx canister call counter get | grep '(8 : nat)' && echo 'PASS'
	dfx canister call caller call_get "( principal \"`dfx canister id counter`\" )" | grep '(variant { Ok = 8 : nat })' && echo 'PASS'
	dfx canister call caller stubborn_set "( principal \"`dfx canister id counter`\", 42 : nat )" | grep '(variant { Ok })' && echo 'PASS'
	dfx canister call caller call_get "( principal \"`dfx canister id counter`\" )" | grep '(variant { Ok = 42 : nat })' && echo 'PASS'
	# We'll test that the counter's balance increases when calling send_cycles to it.
	# Storing the original balance in a file is the easiest with make.
	echo $$(dfx canister status counter | awk '/Balance/ { gsub(/_/, "", $$2); print $$2 }') > .counter_before
	# Send 10% of the caller's available cycles
	CYCLES_TO_SEND=$$(dfx canister status caller | awk '/Balance/ { gsub(/_/, "", $$2); print int($$2/10) }'); \
	echo "Sending $$CYCLES_TO_SEND cycles to counter canister"; \
	dfx canister call caller send_cycles "( principal \"`dfx canister id counter`\", $$CYCLES_TO_SEND: nat64 )" | grep '(variant { Ok })' && echo PASS
	# Check that the counter's balance increased
	AFTER=$$(dfx canister status counter | awk '/Balance/ { gsub(/_/, "", $$2); print $$2 }'); \
	BEFORE=$$(cat .counter_before); \
	test $$AFTER -gt $$BEFORE && echo "PASS (cycles increased: $$BEFORE -> $$AFTER)"

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
