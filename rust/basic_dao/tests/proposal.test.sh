#!ic-repl
load "prelude.sh";

// This will not be needed once replica supports custom section
import fake = "2vxsx-fae" as "../src/basic_dao/src/basic_dao.did";

let wasm = file "../.dfx/local/canisters/basic_dao/basic_dao.wasm";

identity alice;
identity bob;
identity cathy;
identity dory;
identity eve;
identity genesis;

// init args
let init = encode fake.__init_args(
  record {
    accounts = vec { record { owner = genesis; tokens = record { amount_e8s = 1_000_000_000_000 } } };
    proposals = vec {};
    system_params = record {
      transfer_fee = record { amount_e8s = 0 };
      proposal_vote_threshold = record { amount_e8s = 500 };
      proposal_submission_deposit = record { amount_e8s = 100 };
    };
  }
);
let DAO = install(wasm, init, null);

// cannot update system params without proposal
let update_transfer_fee = record { transfer_fee = opt record { amount_e8s = 10_000 : nat64 } };
call DAO.update_system_params(update_transfer_fee);
call DAO.get_system_params();
assert _.transfer_fee.amount_e8s == (0 : nat64);

// distribute tokens
let _ = call DAO.transfer(record { to = alice; amount = record { amount_e8s = 100 } });
let _ = call DAO.transfer(record { to = bob; amount = record { amount_e8s = 200 } });
let _ = call DAO.transfer(record { to = cathy; amount = record { amount_e8s = 300 } });
let _ = call DAO.transfer(record { to = dory; amount = record { amount_e8s = 400 } });
call DAO.account_balance();
// verify no transfer fee
assert _.amount_e8s == (999_999_999_000 : nat64);

// alice makes a proposal
identity alice;
call DAO.account_balance();
assert _.amount_e8s == (100 : nat64);
call DAO.submit_proposal(
  record {
    canister_id = DAO;
    method = "update_system_params";
    message = encode DAO.update_system_params(update_transfer_fee);
  },
);
let alice_id = _.Ok;
call DAO.account_balance();
assert _.amount_e8s == (0 : nat64);

// voting
call DAO.vote(record { proposal_id = alice_id; vote = variant { Yes } });
assert _.Ok == variant { Open };
identity eve;
call DAO.vote(record { proposal_id = alice_id; vote = variant { Yes } });
assert _.Err ~= "Caller does not have any tokens to vote with";
identity bob;
call DAO.get_proposal(alice_id);
assert _? ~= record {
  id = alice_id;
  proposer = alice;
  votes_yes = record { amount_e8s = 0 : nat64 };
  votes_no = record { amount_e8s = 0 : nat64 };
  state = variant { Open };
  payload = record {
    canister_id = DAO;
    method = "update_system_params";
  };
};
call DAO.vote(record { proposal_id = alice_id; vote = variant { Yes } });
assert _.Ok == variant { Open };
call DAO.vote(record { proposal_id = alice_id; vote = variant { No } });
assert _.Err ~= "Already voted";
identity dory;
call DAO.vote(record { proposal_id = alice_id; vote = variant { No } });
assert _.Ok == variant { Open };
identity cathy;
call DAO.vote(record { proposal_id = alice_id; vote = variant { Yes } });
assert _.Ok == variant { Accepted };
identity genesis;
call DAO.vote(record { proposal_id = alice_id; vote = variant { No } });
assert _.Err ~= "is not open for voting";

// refunded
identity alice;
call DAO.account_balance();
assert _.amount_e8s == (100 : nat64);

call DAO.get_proposal(alice_id);
assert _? ~= record {
  votes_yes = record { amount_e8s = 500 : nat64 };
  votes_no = record { amount_e8s = 400 : nat64 };
};

// check proposal is executed
call DAO.get_system_params();
assert _.transfer_fee.amount_e8s == (10_000 : nat64);

// bob makes proposals
identity bob;
call DAO.submit_proposal(
  record {
    canister_id = DAO;
    method = "transfer2";
    message = encode DAO.transfer(record { to = alice; amount = record { amount_e8s = 100 } });
  },
);
let bob1 = _.Ok;
call DAO.submit_proposal(
  record {
    canister_id = DAO;
    method = "transfer2";
    message = encode DAO.transfer(record { to = alice; amount = record { amount_e8s = 100 } });
  },
);
let bob2 = _.Ok;
call DAO.submit_proposal(
  record {
    canister_id = DAO;
    method = "transfer";
    message = encode DAO.transfer(record { to = alice; amount = record { amount_e8s = 100 } });
  },
);
assert _.Err ~= "Caller's account must have at least";

// reject bob1, accept bob2
identity cathy;
call DAO.vote(record { proposal_id = bob1; vote = variant { No } });
call DAO.vote(record { proposal_id = bob2; vote = variant { Yes } });
identity dory;
call DAO.vote(record { proposal_id = bob1; vote = variant { No } });
assert _.Ok == variant { Rejected };
call DAO.vote(record { proposal_id = bob2; vote = variant { Yes } });
assert _.Ok == variant { Accepted };

// bob gets only one refund
identity bob;
call DAO.account_balance();
assert _.amount_e8s == (100 : nat64);
