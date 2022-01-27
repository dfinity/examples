export const idlFactory = ({ IDL }) => {
  const OrderId = IDL.Nat32;
  const CancelOrderReceipt = IDL.Variant({
    'Ok' : OrderId,
    'Err' : IDL.Variant({
      'NotAllowed' : IDL.Null,
      'NotExistingOrder' : IDL.Null,
      'InternalError' : IDL.Null,
    }),
  });
  const Token = IDL.Principal;
  const DepositReceipt = IDL.Variant({
    'Ok' : IDL.Nat,
    'Err' : IDL.Variant({
      'TransferFailure' : IDL.Null,
      'BalanceLow' : IDL.Null,
    }),
  });
  const Symbol = IDL.Text;
  const Time = IDL.Int;
  const Order = IDL.Record({
    'id' : OrderId,
    'to' : Token,
    'dip_symbol' : Symbol,
    'fromAmount' : IDL.Nat,
    'submitted' : Time,
    'owner' : IDL.Principal,
    'from' : Token,
    'price' : IDL.Float64,
    'toAmount' : IDL.Nat,
  });
  const OrderPlacementReceipt = IDL.Variant({
    'Ok' : Order,
    'Err' : IDL.Variant({
      'InvalidOrder' : IDL.Null,
      'OrderBookFull' : IDL.Null,
    }),
  });
  const WithdrawReceipt = IDL.Variant({
    'Ok' : IDL.Nat,
    'Err' : IDL.Variant({
      'TransferFailure' : IDL.Null,
      'BalanceLow' : IDL.Null,
    }),
  });
  return IDL.Service({
    'cancelOrder' : IDL.Func([OrderId], [CancelOrderReceipt], []),
    'deposit' : IDL.Func([Token], [DepositReceipt], []),
    'depositAddress' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'getBalance' : IDL.Func([Token], [IDL.Nat], ['query']),
    'getOrder' : IDL.Func([OrderId], [IDL.Opt(Order)], []),
    'listOrders' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'placeOrder' : IDL.Func(
        [Token, IDL.Nat, Token, IDL.Nat],
        [OrderPlacementReceipt],
        [],
      ),
    'symbol' : IDL.Func([Token], [IDL.Text], []),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
    'withdraw' : IDL.Func([Token, IDL.Nat], [WithdrawReceipt], []),
  });
};
export const init = ({ IDL }) => { return []; };
