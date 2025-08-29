.PHONY: all
all: deploy

.PHONY: build
.SILENT: build
deploy:
	dfx deploy

.PHONY: test
.SILENT: test
test: deploy
	dfx canister call canister-info info "(principal\"`dfx canister id test`\")" --output idl | grep -x "(record.*controllers.*module_hash.*recent_changes.*creation.*code_deployment.*total_num_changes = 2.*)"
	dfx canister call canister-info canister_controllers "(principal\"`dfx canister id test`\",variant {at_timestamp=`perl -MTime::HiRes=time -e 'printf "%d\n", 1e9*time'`})" --output idl | grep -x "(opt record.*vec.*`dfx identity get-principal`.*opt.*)"
	dfx canister call canister-info canister_controllers "(principal\"`dfx canister id test`\",variant {at_version=2})" --output idl | grep -x "(opt record.*vec.*`dfx identity get-principal`.*null.*)"
	dfx canister call canister-info canister_module_hash "(principal\"`dfx canister id test`\",variant {at_timestamp=`perl -MTime::HiRes=time -e 'printf "%d\n", 1e9*time'`})" --output idl | grep -x "(opt record.*opt.*blob.*opt.*)"
	dfx canister call canister-info canister_module_hash "(principal\"`dfx canister id test`\",variant {at_version=2})" --output idl | grep -x "(opt record.*opt.*blob.*null.*)"
	dfx canister call canister-info canister_deployment_chain "(principal\"`dfx canister id test`\",variant {at_timestamp=`perl -MTime::HiRes=time -e 'printf "%d\n", 1e9*time'`})" --output idl | grep -x "(record.*vec.*record.*code_deployment.*max_clock_skew.*opt.*)"
	dfx canister call canister-info canister_deployment_chain "(principal\"`dfx canister id test`\",variant {at_version=2})" --output idl | grep -x "(record.*vec.*record.*code_deployment.*max_clock_skew.*null.*)"
	dfx canister call canister-info reflexive_transitive_controllers "(principal\"`dfx canister id test`\")" --output idl | grep -x "(vec.*`dfx identity get-principal`.*)"

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
