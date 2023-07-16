.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister create --all --no-wallet
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister install --all --mode=upgrade

.PHONY: test
.SILENT: test
test: install
  dfx canister call canister-info info "(principal\"$(dfx canister id test)\")" --output idl | grep "^(record.*controllers.*module_hash.*recent_changes.*creation.*code_deployment.*total_num_changes = 2.*)$"
  dfx canister call canister-info canister_controllers "(principal\"$(dfx canister id test)\",variant {at_timestamp=$(date +%s%N)})" --output idl | grep "(opt record.*vec.*$(dfx identity get-principal).*opt.*)"
  dfx canister call canister-info canister_controllers "(principal\"$(dfx canister id test)\",variant {at_version=2})" --output idl | grep "(opt record.*vec.*$(dfx identity get-principal).*null.*)"
  dfx canister call canister-info canister_module_hash "(principal\"$(dfx canister id test)\",variant {at_timestamp=$(date +%s%N)})" --output idl | grep "(opt record.*opt.*blob.*opt.*)"
  dfx canister call canister-info canister_module_hash "(principal\"$(dfx canister id test)\",variant {at_version=2})" --output idl | grep "(opt record.*opt.*blob.*null.*)"
  dfx canister call canister-info canister_deployment_chain "(principal\"$(dfx canister id test)\",variant {at_timestamp=$(date +%s%N)})" --output idl | grep "(vec.*record.*code_deployment.*,.*opt.*)"
  dfx canister call canister-info canister_deployment_chain "(principal\"$(dfx canister id test)\",variant {at_version=2})" --output idl | grep "(vec.*record.*code_deployment.*,.*null.*)"
  dfx canister call canister-info reflexive_transitive_controllers "(principal\"$(dfx canister id test)\")" --output idl | grep "^(vec.*$(dfx identity get-principal).*)$"

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
