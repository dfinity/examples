.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx build --check

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy

.PHONY: test
.SILENT: test
test: build
	cargo test

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
	rm -rf dist
	rm -rf node_modules
	rm -rf src/declarations
	rm -f .env
	cargo clean