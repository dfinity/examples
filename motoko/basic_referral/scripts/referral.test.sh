#!ic-repl -r http://localhost:55863

import BasicReferral = "${BASIC_REFERRAL}";

identity alice;
let _ = call BasicReferral.register(null, opt "alice", null, null);
let alice_principal = call BasicReferral.getPrincipalText();
call BasicReferral.getBalance();
assert _.e8s == (0 : nat64);
call BasicReferral.getReferralCount();
assert _.ok == (0 : int);

// Bob register with alice principal
identity bob;
let _ = call BasicReferral.register(opt alice_principal, opt "bob", null, null);
call BasicReferral.getBalance();
assert _.e8s == (0 : nat64);
call BasicReferral.getReferralCount();
assert _.ok == (0 : int);

identity alice;
call BasicReferral.getBalance();
assert _.e8s == (40_000 : nat64);
call BasicReferral.getReferralCount();
assert _.ok == (1 : int);