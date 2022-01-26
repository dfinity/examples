#!/usr/local/bin/ic-repl

identity user1;
identity default "~/.config/dfx/identity/default/identity.pem";
import AkitaDIP20="${AKITA_ID}";
import DEX="${DEX_PRINCIPLE}";
import LEDGER="${LEDGER}" as "../src/ledger/ledger.public.did";

// configurable values
// need to change assert values based on defined config values
// ic-repl does not support arithemtics
let dip20_fee=${DIP20_FEE};
let dip20_deposit=${DIP_DEPOSIT};
let icp_fee=${ICP_FEE};
let icp_deposit=${ICP_DEPOSIT};

let dip20_deposit_dex_balance=${DIP_DEPOSIT_DEX_BALANCE};
let dip20_deposit_dip_balance=${DIP_DEPOSIT_DIP_BALANCE};

let dip20_deposit2_dex_balance=${DIP_DEPOSIT2_DEX_BALANCE};
let dip20_deposit2_dip_balance=${DIP_DEPOSIT2_DIP_BALANCE};

"Moving DIP20 tokens to DEX";
let _ = call AkitaDIP20.setFee(
  dip20_fee
);

let _ = call AkitaDIP20.approve(
  principal "${DEX_PRINCIPLE}", dip20_deposit
);
let _ = call DEX.deposit(
  principal "${AKITA_ID}"
);
let status = call DEX.balance(
  principal "${AKITA_ID}"
);
assert status ~= (dip20_deposit_dex_balance : nat);
let status = call AkitaDIP20.balanceOf(
  default
);
assert status ~= (dip20_deposit_dip_balance : nat);

"Moving DIP20 tokens to DEX Ok!";
"Moving DIP20 tokens to DEX second time";

// double deposit
let _ = call AkitaDIP20.approve(
  principal "${DEX_PRINCIPLE}", dip20_deposit
);
let _ = call DEX.deposit(
  principal "${AKITA_ID}"
);
let status = call DEX.balance(
  principal "${AKITA_ID}"
);
assert status ~= (dip20_deposit2_dex_balance : nat);
let status = call AkitaDIP20.balanceOf(
  default
);
assert status ~= (dip20_deposit2_dip_balance : nat);

"Moving DIP20 tokens to DEX second time Ok!";
"Withddraw too much";

// withdraw more than user has available
let status = call DEX.withdraw(
  principal "${AKITA_ID}",
  10000000000
);
assert status.Err ~=  variant { BalanceLow };
"Withddraw too much Ok!";
