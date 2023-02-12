.PHONY: all
all: deploy

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy basic_bitcoin --argument '(variant { Regtest })'

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
	cargo clean
