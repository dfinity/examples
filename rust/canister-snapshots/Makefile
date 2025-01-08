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
	dfx canister install chat

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install chat --mode=upgrade

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy chat

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call chat append "Hi there!" | grep -qw '()' && echo 'PASS'
	dfx canister call chat dump | grep -qw 'Hi' && echo 'PASS'
	dfx canister call chat remove_spam | grep -qw '0' && echo 'PASS'
	dfx canister call chat dump | grep -qwvw 'Hi' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
