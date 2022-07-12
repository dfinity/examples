export const idlFactory = ({ IDL }) => {
  const Address = IDL.Vec(IDL.Nat8);
  const AccountBalanceArgs = IDL.Record({ 'account' : Address });
  const Token = IDL.Record({ 'e8s' : IDL.Nat64 });
  const Memo = IDL.Nat64;
  const SubAccount = IDL.Vec(IDL.Nat8);
  const TimeStamp = IDL.Record({ 'timestamp_nanos' : IDL.Nat64 });
  const TransferArgs = IDL.Record({
    'to' : Address,
    'fee' : Token,
    'memo' : Memo,
    'from_subaccount' : IDL.Opt(SubAccount),
    'created_at_time' : IDL.Opt(TimeStamp),
    'amount' : Token,
  });
  const BlockIndex = IDL.Nat64;
  const TransferError = IDL.Variant({
    'TxTooOld' : IDL.Record({ 'allowed_window_nanos' : IDL.Nat64 }),
    'BadFee' : IDL.Record({ 'expected_fee' : Token }),
    'TxDuplicate' : IDL.Record({ 'duplicate_of' : BlockIndex }),
    'TxCreatedInFuture' : IDL.Null,
    'InsufficientFunds' : IDL.Record({ 'balance' : Token }),
  });
  const TransferResult = IDL.Variant({
    'Ok' : BlockIndex,
    'Err' : TransferError,
  });
  return IDL.Service({
    'account_balance' : IDL.Func([AccountBalanceArgs], [Token], ['query']),
    'transfer' : IDL.Func([TransferArgs], [TransferResult], []),
  });
};
export const init = ({ IDL }) => { return []; };
