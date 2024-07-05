.PHONY: all
all: deploy

.PHONY: deploy
.SILENT: deploy
deploy:
	dfx deploy schnorr_canister && dfx deploy basic_bitcoin --argument '(variant { regtest }, "dfx_test_key" : text)'

.PHONY: mock
.SILENT: mock
mock: deploy
	SCHNORR_MOCK_CANISTER_ID=$(shell dfx canister id schnorr_canister); \
	THIS_CANISTER_ID=$(shell dfx canister id basic_bitcoin); \
	echo "Changing to using mock canister instead of management canister for signing"; \
	CMD="dfx canister call "$${THIS_CANISTER_ID}" for_test_only_set_schnorr_canister_id '("\"$${SCHNORR_MOCK_CANISTER_ID}\"")'"; \
	echo "$${CMD}"; \
	eval "$${CMD}"

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
