#!ic-repl
load "prelude.sh";

import fake = "2vxsx-fae" as "../src/basic_dao/src/basic_dao.did";
let wasm = file "../.dfx/local/canisters/basic_dao/basic_dao.wasm";


// Setup initial account
identity alice;
let args = encode fake.__init_args(
  record {
    accounts = vec { record { owner = alice; tokens = record { amount_e8s = 1_000_000_000_000 } } };
    proposals = vec {};
    system_params = record {
      transfer_fee = record { amount_e8s = 10_000 };
      proposal_vote_threshold = record { amount_e8s = 1_000_000_000 };
      proposal_submission_deposit = record { amount_e8s = 10_000 };
    };
  }
);
let DAO = install(wasm, args, null);
call DAO.account_balance();
assert _.amount_e8s == (1_000_000_000_000 : nat64);

// destination address needs to exist
identity bob;
call DAO.account_balance();
assert _.amount_e8s == (0 : nat64);
call DAO.transfer(
  record {
    to = alice;
    amount = record {
      amount_e8s = 500_000_000_000;
    };
  }
);
assert _ == variant { Err = "Caller needs an account to transfer funds" };

// transfer from alice to bob
identity alice;
call DAO.transfer(
  record {
    to = bob;
    amount = record {
      amount_e8s = 500_000_000_000;
    };
  },
);
call DAO.account_balance();
assert _.amount_e8s == (499_999_990_000 : nat64);
identity bob;
call DAO.account_balance();
assert _.amount_e8s == (500_000_000_000 : nat64);

// not enough funds considering transfer fee
call DAO.transfer(
  record {
    to = alice;
    amount = record {
      amount_e8s = 600_000_000_000;
    };
  }
);
assert _ == variant {Err = "Caller's account has insufficient funds to transfer Tokens { amount_e8s: 600000000000 }" };

// transfer to self
call DAO.transfer(
  record {
    to = bob;
    amount = record {
      amount_e8s = 10;
    };
  }
);
call DAO.account_balance();
assert _.amount_e8s == (499_999_990_000 : nat64);
