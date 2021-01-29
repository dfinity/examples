NAME = factorial

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
	dfx canister call $(NAME) fac '(20)' \
		| grep '(2_432_902_008_176_640_000)' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
