export const idlFactory = ({ IDL }) => {
  const Token = IDL.Text;
  const TokenBalance = IDL.Record({
    'principal' : IDL.Principal,
    'token' : Token,
    'amount' : IDL.Nat,
  });
  const Order = IDL.Record({
    'id' : IDL.Text,
    'to' : Token,
    'fromAmount' : IDL.Nat,
    'from' : Token,
    'toAmount' : IDL.Nat,
  });
  const OrderPlacementResult = IDL.Record({
    'status' : IDL.Text,
    'order' : Order,
  });
  return IDL.Service({
    'balances' : IDL.Func([], [IDL.Vec(TokenBalance)], ['query']),
    'cancel_order' : IDL.Func([IDL.Text], [], ['oneway']),
    'check_order' : IDL.Func([IDL.Text], [IDL.Opt(Order)], []),
    'deposit' : IDL.Func([], [], ['oneway']),
    'list_order' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'place_order' : IDL.Func(
        [Token, IDL.Nat, Token, IDL.Nat],
        [OrderPlacementResult],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
    'withdraw' : IDL.Func([], [], ['oneway']),
  });
};
export const init = ({ IDL }) => { return []; };
