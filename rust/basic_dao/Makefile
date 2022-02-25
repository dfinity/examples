.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	cargo build --target wasm32-unknown-unknown --package  basic_dao --release
	ic-cdk-optimizer target/wasm32-unknown-unknown/release/basic_dao.wasm -o target/wasm32-unknown-unknown/release/basic_dao_opt.wasm

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
