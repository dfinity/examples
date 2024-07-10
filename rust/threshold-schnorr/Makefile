.PHONY: all
all: build

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy

.PHONY: test
.SILENT: test
test:
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