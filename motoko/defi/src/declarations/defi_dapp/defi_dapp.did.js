export const idlFactory = ({ IDL }) => {
  const Token = IDL.Principal;
  const OrderId = IDL.Nat32;
  const CancelOrderReceipt = IDL.Variant({
    'Ok' : OrderId,
    'Err' : IDL.Variant({
      'NotAllowed' : IDL.Null,
      'NotExistingOrder' : IDL.Null,
    }),
  });
  const Order = IDL.Record({
    'id' : OrderId,
    'to' : Token,
    'fromAmount' : IDL.Nat,
    'owner' : IDL.Principal,
    'from' : Token,
    'toAmount' : IDL.Nat,
  });
  const DepositReceipt = IDL.Variant({
    'Ok' : IDL.Nat,
    'Err' : IDL.Variant({
      'TransferFailure' : IDL.Null,
      'BalanceLow' : IDL.Null,
    }),
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
    'balance' : IDL.Func([Token], [IDL.Nat], ['query']),
    'cancel_order' : IDL.Func([OrderId], [CancelOrderReceipt], []),
    'check_order' : IDL.Func([OrderId], [IDL.Opt(Order)], []),
    'deposit' : IDL.Func([Token], [DepositReceipt], []),
    'deposit_address' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'list_order' : IDL.Func([], [IDL.Vec(Order)], ['query']),
    'place_order' : IDL.Func(
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
