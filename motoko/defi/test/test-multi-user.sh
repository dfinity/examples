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
let user1_balance=${AKITA_BALANCE_USER1};

let dip20_deposit_dex_balance=${DIP_DEPOSIT_DEX_BALANCE};
let dip20_deposit_dip_balance=${DIP_DEPOSIT_DIP_BALANCE};
let dip20_deposit_dex_balance1=${DIP_DEPOSIT_DEX_BALANCE};
let dip20_deposit_dip_balance1=${DIP_DEPOSIT_DIP_BALANCE1};


"Moving DIP20 tokens to DEX";
let _ = call AkitaDIP20.setFee(
  dip20_fee
);

let _ = call AkitaDIP20.transfer(
  user1,user1_balance
);

let _ = call AkitaDIP20.balanceOf(
  default
);

let _ = call AkitaDIP20.balanceOf(
  user1
);

// approve funds to DEX
identity user1;
let _ = call AkitaDIP20.approve(
  principal "${DEX_PRINCIPLE}", dip20_deposit
);
let _ = call DEX.deposit(
  principal "${AKITA_ID}"
);
let status = call DEX.balance(
  principal "${AKITA_ID}"
);
assert status ~= (dip20_deposit_dex_balance1 : nat);
let status = call AkitaDIP20.balanceOf(
  user1
);
assert status ~= (dip20_deposit_dip_balance1 : nat);


identity default "~/.config/dfx/identity/default/identity.pem";
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
