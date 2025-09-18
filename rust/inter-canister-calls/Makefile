.PHONY: all
all: test

.PHONY: deploy
.SILENT: deploy
build:
	dfx deploy

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
	dfx canister call caller sign_message '("Some text to be signed")' | grep "Ok = \"" && echo PASS

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
