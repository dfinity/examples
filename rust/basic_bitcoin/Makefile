.PHONY: all
all: deploy

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy basic_bitcoin --argument '(variant { regtest })'

.PHONY: regtest_topup
.SILENT: regtest_topup
regtest_topup:
	P2PKH_ADDR=$(shell dfx canister call basic_bitcoin get_p2pkh_address | tr -d '()') && \
	P2TR_ADDR=$(shell dfx canister call basic_bitcoin get_p2tr_address | tr -d '()') && \
	P2TR_KEY_ONLY_ADDR=$(shell dfx canister call basic_bitcoin get_p2tr_key_only_address | tr -d '()') && \
	TOPUP_CMD_P2PKH_ADDR="bitcoin-cli -regtest -rpcport=8333 sendtoaddress $${P2PKH_ADDR} 1" && \
	TOPUP_CMD_P2TR_ADDR="bitcoin-cli -regtest -rpcport=8333 sendtoaddress $${P2TR_ADDR} 1" && \
	TOPUP_CMD_P2TR_KEY_ONLY_ADDR="bitcoin-cli -regtest -rpcport=8333 sendtoaddress $${P2TR_KEY_ONLY_ADDR} 1" && \
	eval "$${TOPUP_CMD_P2PKH_ADDR}" && \
	eval "$${TOPUP_CMD_P2PKH_ADDR}" && \
	eval "$${TOPUP_CMD_P2PKH_ADDR}" && \
	eval "$${TOPUP_CMD_P2TR_ADDR}" && \
	eval "$${TOPUP_CMD_P2TR_ADDR}" && \
	eval "$${TOPUP_CMD_P2TR_ADDR}" && \
	eval "$${TOPUP_CMD_P2TR_KEY_ONLY_ADDR}" && \
	eval "$${TOPUP_CMD_P2TR_KEY_ONLY_ADDR}" && \
	eval "$${TOPUP_CMD_P2TR_KEY_ONLY_ADDR}" && \
	bitcoin-cli -regtest -rpcport=8333 -generate 6

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
	rm -rf dist
	rm -rf node_modules
	rm -rf src/declarations
	rm -f .env
	cargo clean
