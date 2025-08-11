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
	dfx canister install wotd

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install wotd --mode=upgrade

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy wotd

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call wotd setup | grep -qw '()' && echo 'PASS'
	dfx canister call wotd print | grep -qw 'Colourless green ideas sleep furiously.' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
