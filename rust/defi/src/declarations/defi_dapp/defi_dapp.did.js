export const idlFactory = ({ IDL }) => {
  const OrderId = IDL.Nat32;
  const CancelOrderErr = IDL.Variant({
    'NotAllowed' : IDL.Null,
    'NotExistingOrder' : IDL.Null,
  });
  const CancelOrderReceipt = IDL.Variant({
    'Ok' : IDL.Nat64,
    'Err' : CancelOrderErr,
  });
  const Token = IDL.Principal;
  const DepositErr = IDL.Variant({
    'TransferFailure' : IDL.Null,
    'BalanceLow' : IDL.Null,
  });
  const DepositReceipt = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : DepositErr });
  const Balance = IDL.Record({
    'token' : Token,
    'owner' : IDL.Principal,
    'amount' : IDL.Nat,
  });
  const Order = IDL.Record({
    'id' : OrderId,
    'to' : IDL.Principal,
    'fromAmount' : IDL.Nat,
    'owner' : IDL.Principal,
    'from' : IDL.Principal,
    'toAmount' : IDL.Nat,
  });
  const OrderPlacementErr = IDL.Variant({
    'InvalidOrder' : IDL.Null,
    'OrderBookFull' : IDL.Null,
  });
  const OrderPlacementReceipt = IDL.Variant({
    'Ok' : IDL.Opt(Order),
    'Err' : OrderPlacementErr,
  });
  const WithdrawErr = IDL.Variant({
    'TransferFailure' : IDL.Null,
    'BalanceLow' : IDL.Null,
  });
  const WithdrawReceipt = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : WithdrawErr });
  return IDL.Service({
    'cancelOrder' : IDL.Func([OrderId], [CancelOrderReceipt], []),
    'clear' : IDL.Func([], [], []),
    'credit' : IDL.Func([IDL.Principal, Token, IDL.Nat], [], []),
    'deposit' : IDL.Func([Token], [DepositReceipt], []),
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
