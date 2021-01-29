NAME = calc

.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create $(NAME)
	dfx build $(NAME)

.PHONY: install
.SILENT: install
install: build
	dfx canister install $(NAME)

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --mode=upgrade $(NAME)

.PHONY: test
.SILENT: test
test: install
	dfx canister call $(NAME) add '(9)'
	dfx canister call $(NAME) sub '(1)'
	dfx canister call $(NAME) mul '(3)'
	dfx canister call $(NAME) div '(6)' \
		| grep '(opt 4)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
