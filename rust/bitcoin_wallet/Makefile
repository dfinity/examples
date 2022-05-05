.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	cargo build --target wasm32-unknown-unknown --package bitcoin_wallet --release
	ic-cdk-optimizer target/wasm32-unknown-unknown/release/bitcoin_wallet.wasm -o target/wasm32-unknown-unknown/release/bitcoin_wallet_opt.wasm

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
