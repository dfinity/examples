export const idlFactory = ({ IDL }) => {
  const OrderId = IDL.Nat32;
  const CancelOrderReceipt = IDL.Variant({
    'Ok' : OrderId,
    'Err' : IDL.Variant({
      'NotAllowed' : IDL.Null,
      'NotExistingOrder' : IDL.Null,
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
  const Balance = IDL.Record({
    'token' : Token,
    'owner' : IDL.Principal,
    'amount' : IDL.Nat,
  });
  const Order = IDL.Record({
    'id' : OrderId,
    'to' : Token,
    'fromAmount' : IDL.Nat,
    'owner' : IDL.Principal,
    'from' : Token,
    'toAmount' : IDL.Nat,
  });
  const OrderPlacementReceipt = IDL.Variant({
    'Ok' : Order,
    'Err' : IDL.Variant({
      'InvalidOrder' : IDL.Null,
      'OrderBookFull' : IDL.Null,
    }),
    'Executed' : IDL.Null,
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
    'deposit' : IDL.Func([Token, IDL.Nat], [DepositReceipt], []),
    'getAllBalances' : IDL.Func([], [IDL.Vec(Balance)], ['query']),
    'getBalance' : IDL.Func([Token], [IDL.Nat], ['query']),
    'getBalances' : IDL.Func([], [IDL.Vec(Balance)], ['query']),
    'getDepositAddress' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'getOrder' : IDL.Func([OrderId], [IDL.Opt(Order)], []),
    'getOrders' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'getSymbol' : IDL.Func([Token], [IDL.Text], []),
    'placeOrder' : IDL.Func(
        [Token, IDL.Nat, Token, IDL.Nat],
        [OrderPlacementReceipt],
        [],
      ),
    'whoami' : IDL.Func([], [IDL.Principal], ['query']),
    'withdraw' : IDL.Func([Token, IDL.Nat], [WithdrawReceipt], []),
  });
};
export const init = ({ IDL }) => { return []; };
