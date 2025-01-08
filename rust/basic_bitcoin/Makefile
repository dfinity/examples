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
	P2TR_SCRIPT_SPEND_ADDR=$(shell dfx canister call basic_bitcoin get_p2tr_script_spend_address | tr -d '()') && \
	P2TR_SCRIPT_KEY_SPEND_ADDR=$(shell dfx canister call basic_bitcoin get_p2tr_raw_key_spend_address | tr -d '()') && \
	TOPUP_CMD_P2PKH="bitcoin-cli -rpcport=8333 sendtoaddress $${P2PKH_ADDR} 1" && \
	TOPUP_CMD_P2TR_SCRIPT_SPEND="bitcoin-cli -rpcport=8333 sendtoaddress $${P2TR_SCRIPT_SPEND_ADDR} 1" && \
	TOPUP_CMD_P2TR_RAW_KEY_SPEND="bitcoin-cli -rpcport=8333 sendtoaddress $${P2TR_SCRIPT_KEY_SPEND_ADDR} 1" && \
	eval "$${TOPUP_CMD_P2PKH}" && \
	eval "$${TOPUP_CMD_P2PKH}" && \
	eval "$${TOPUP_CMD_P2PKH}" && \
	eval "$${TOPUP_CMD_P2TR_SCRIPT_SPEND}" && \
	eval "$${TOPUP_CMD_P2TR_SCRIPT_SPEND}" && \
	eval "$${TOPUP_CMD_P2TR_SCRIPT_SPEND}" && \
	eval "$${TOPUP_CMD_P2TR_RAW_KEY_SPEND}" && \
	eval "$${TOPUP_CMD_P2TR_RAW_KEY_SPEND}" && \
	eval "$${TOPUP_CMD_P2TR_RAW_KEY_SPEND}" && \
	bitcoin-cli -rpcport=8333 -generate 6

.PHONY: clean
.SILENT: clean
clean:
	rm -rf .dfx
	rm -rf dist
	rm -rf node_modules
	rm -rf src/declarations
	rm -f .env
	cargo clean
