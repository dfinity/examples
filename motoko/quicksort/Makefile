.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create quicksort
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install quicksort

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install quicksort --mode=upgrade

.PHONY: test
.SILENT: test
test: install
	dfx canister call quicksort sort '(vec { 5; 3; 0; 9; 8; 2; 1; 4; 7; 6 })' > tmp.txt
	tr '\n' ' ' < tmp.txt | grep '(   vec {     0 : int;     1 : int;     2 : int;     3 : int;     4 : int;     5 : int;     6 : int;     7 : int;     8 : int;     9 : int;   }, )' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
	rm tmp.txt
