.PHONY: build
.SILENT: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: test
.SILENT: test
test: build
	cargo test

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
	cargo clean
