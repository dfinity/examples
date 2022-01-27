#!ic-repl

function install(wasm, args, cycle) {
  let id = call ic.provisional_create_canister_with_cycles(record { settings = null; amount = cycle });
  let S = id.canister_id;
  call ic.install_code(
    record {
      arg = args;
      wasm_module = wasm;
      mode = variant { install };
      canister_id = S;
    }
  );
  S
};

function upgrade(cid, wasm, args) {
  call ic.install_code(
    record {
      arg = args;
      wasm_module = wasm;
      mode = variant { upgrade };
      canister_id = cid;
    }
  );
};
