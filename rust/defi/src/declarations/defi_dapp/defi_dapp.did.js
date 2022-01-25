export const idlFactory = ({ IDL }) => {
  const OwnerBalance = IDL.Record({
    'owner' : IDL.Principal,
    'token_canister_id' : IDL.Principal,
    'amount' : IDL.Nat,
  });
  const Order = IDL.Record({
    'id' : IDL.Nat64,
    'from_amount' : IDL.Nat,
    'from_token_canister_id' : IDL.Principal,
    'owner' : IDL.Principal,
    'to_amount' : IDL.Nat,
    'to_token_canister_id' : IDL.Principal,
  });
  const Balance = IDL.Record({
    'token_canister_id' : IDL.Principal,
    'amount' : IDL.Nat,
  });
  return IDL.Service({
    'cancel_order' : IDL.Func([IDL.Nat64], [IDL.Text], []),
    'clear' : IDL.Func([], [IDL.Text], []),
    'deposit' : IDL.Func([IDL.Principal, IDL.Nat], [IDL.Text], []),
    'get_all_balances' : IDL.Func([], [IDL.Vec(OwnerBalance)], ['query']),
    'get_all_orders' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'get_balance' : IDL.Func([IDL.Principal], [IDL.Opt(Balance)], ['query']),
    'get_balances' : IDL.Func([], [IDL.Vec(Balance)], ['query']),
    'get_from_orders' : IDL.Func([IDL.Principal], [IDL.Vec(Order)], ['query']),
    'get_order' : IDL.Func([IDL.Nat64], [IDL.Opt(Order)], ['query']),
    'get_orders' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'get_to_orders' : IDL.Func([IDL.Principal], [IDL.Vec(Order)], ['query']),
    'icp_deposit_account' : IDL.Func([], [IDL.Text], ['query']),
    'place_order' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Principal, IDL.Nat],
        [IDL.Text],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
    'withdraw' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Principal],
        [],
        ['oneway'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
