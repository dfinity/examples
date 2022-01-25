export const idlFactory = ({ IDL }) => {
  const Token = IDL.Principal;
  const OrderId = IDL.Nat32;
  const CancelOrderResponse = IDL.Record({
    'status' : IDL.Text,
    'order_id' : OrderId,
  });
  const Order = IDL.Record({
    'id' : OrderId,
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
  const WithdrawError = IDL.Variant({
    'balanceLow' : IDL.Null,
    'transferFailure' : IDL.Null,
  });
  const Result = IDL.Variant({ 'ok' : IDL.Nat64, 'err' : WithdrawError });
  return IDL.Service({
    'balance' : IDL.Func([Token], [IDL.Nat64], ['query']),
    'cancel_order' : IDL.Func([OrderId], [CancelOrderResponse], []),
    'check_order' : IDL.Func([OrderId], [IDL.Opt(Order)], []),
    'deposit_address' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'deposit_dip' : IDL.Func([Token], [IDL.Opt(IDL.Text)], []),
    'deposit_icp' : IDL.Func([], [IDL.Opt(IDL.Text)], []),
    'list_order' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'place_order' : IDL.Func(
        [Token, IDL.Nat, Token, IDL.Nat],
        [OrderPlacementResponse],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
    'withdraw_dip' : IDL.Func([Token, IDL.Nat64], [Result], []),
    'withdraw_icp' : IDL.Func([IDL.Nat64], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
