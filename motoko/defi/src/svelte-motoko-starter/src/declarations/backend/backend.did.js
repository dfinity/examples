export const idlFactory = ({ IDL }) => {
  const Token = IDL.Text;
  const TokenBalance = IDL.Record({
    'principal' : IDL.Principal,
    'token' : Token,
    'amount' : IDL.Nat,
  });
  const CancelOrderResponse = IDL.Record({
    'status' : IDL.Text,
    'order_id' : IDL.Text,
  });
  const Order = IDL.Record({
    'id' : IDL.Text,
    'to' : Token,
    'fromAmount' : IDL.Nat,
    'owner' : IDL.Principal,
    'from' : Token,
    'toAmount' : IDL.Nat,
  });
  const OrderPlacementResponse = IDL.Record({
    'status' : IDL.Text,
    'order' : Order,
  });
  return IDL.Service({
    'balances' : IDL.Func([], [IDL.Vec(TokenBalance)], ['query']),
    'cancel_order' : IDL.Func([IDL.Text], [CancelOrderResponse], []),
    'check_order' : IDL.Func([IDL.Text], [IDL.Opt(Order)], []),
    'convert_icp' : IDL.Func([], [IDL.Opt(IDL.Text)], []),
    'convert_token' : IDL.Func([], [IDL.Opt(IDL.Text)], []),
    'deposit' : IDL.Func([], [], ['oneway']),
    'deposit_address' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'init' : IDL.Func([], [IDL.Opt(IDL.Text)], []),
    'list_order' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'place_order' : IDL.Func(
        [Token, IDL.Nat, Token, IDL.Nat],
        [OrderPlacementResponse],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
    'withdraw' : IDL.Func([], [], ['oneway']),
  });
};
export const init = ({ IDL }) => { return []; };
