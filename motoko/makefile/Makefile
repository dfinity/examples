.PHONY: default canister build install call

# Usage:
# make call method={actorMethod} [arg]

src := $(wildcard src/*/*.mo) $(wildcard src/*.mo)

c ?= $(shell basename $(CURDIR))
canister ?= $(c)

m ?= ''
method ?= $(m)

a ?= ''
args ?= $(a)

install_mode ?= reinstall

# commands

default:
	$(MAKE) call $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
canister: .dfx/local/canister_ids.json

# If first target is 'call', rest of targets will be stored as $CALL_ARGS
# so they can be forwarded as positional args.
# via https://stackoverflow.com/a/14061796
ifeq (call,$(firstword $(MAKECMDGOALS)))
  # use the rest as arguments for "call"
  CALL_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  # ...and turn them into do-nothing targets
  $(eval $(CALL_ARGS):;@:)
endif
call: .dfx/installed/$(canister)
	@if [ -z $(method) ]; then\
		echo "provide a method=yourMethodName parameter";\
		exit 1;\
	fi
	dfx canister call $(canister) $(method) "$(CALL_ARGS)"

build: canister
	dfx build

# files

.dfx/local/canister_ids.json:
	dfx canister create --all

.dfx/local/canisters/$(canister)/$(canister).wasm: $(wildcard src/$(canister)/*.mo)
	$(MAKE) build
	touch $@

.dfx/installed:
	mkdir -p $@

.dfx/installed/$(canister): .dfx/installed .dfx/built .dfx/local/canisters/$(canister)/$(canister).wasm
	dfx canister install $(canister) --mode="$(install_mode)"
	touch $@

.dfx/built: $(src)
	$(MAKE) build
	touch $@
