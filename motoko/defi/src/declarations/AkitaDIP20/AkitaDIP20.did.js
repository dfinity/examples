export const idlFactory = ({ IDL }) => {
  const TxReceipt = IDL.Variant({
    'Ok' : IDL.Nat,
    'Err' : IDL.Variant({
      'InsufficientAllowance' : IDL.Null,
      'InsufficientBalance' : IDL.Null,
      'ErrorOperationStyle' : IDL.Null,
      'Unauthorized' : IDL.Null,
      'LedgerTrap' : IDL.Null,
      'ErrorTo' : IDL.Null,
      'Other' : IDL.Null,
      'BlockUsed' : IDL.Null,
      'AmountTooSmall' : IDL.Null,
    }),
  });
  const Metadata = IDL.Record({
    'fee' : IDL.Nat,
    'decimals' : IDL.Nat8,
    'owner' : IDL.Principal,
    'logo' : IDL.Text,
    'name' : IDL.Text,
    'totalSupply' : IDL.Nat,
    'symbol' : IDL.Text,
  });
  const Time = IDL.Int;
  const TokenInfo = IDL.Record({
    'holderNumber' : IDL.Nat,
    'deployTime' : Time,
    'metadata' : Metadata,
    'historySize' : IDL.Nat,
    'cycles' : IDL.Nat,
    'feeTo' : IDL.Principal,
  });
  const Operation = IDL.Variant({
    'transferFrom' : IDL.Null,
    'burn' : IDL.Null,
    'mint' : IDL.Null,
    'approve' : IDL.Null,
    'transfer' : IDL.Null,
  });
  const TransactionStatus = IDL.Variant({
    'inprogress' : IDL.Null,
    'failed' : IDL.Null,
    'succeeded' : IDL.Null,
  });
  const TxRecord = IDL.Record({
    'op' : Operation,
    'to' : IDL.Principal,
    'fee' : IDL.Nat,
    'status' : TransactionStatus,
    'from' : IDL.Principal,
    'timestamp' : Time,
    'caller' : IDL.Opt(IDL.Principal),
    'index' : IDL.Nat,
    'amount' : IDL.Nat,
  });
  const Token = IDL.Service({
    'allowance' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [IDL.Nat],
        ['query'],
      ),
    'approve' : IDL.Func([IDL.Principal, IDL.Nat], [TxReceipt], []),
    'balanceOf' : IDL.Func([IDL.Principal], [IDL.Nat], ['query']),
    'burn' : IDL.Func([IDL.Nat], [TxReceipt], []),
    'decimals' : IDL.Func([], [IDL.Nat8], ['query']),
    'getAllowanceSize' : IDL.Func([], [IDL.Nat], ['query']),
    'getHolders' : IDL.Func(
        [IDL.Nat, IDL.Nat],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat))],
        ['query'],
      ),
    'getMetadata' : IDL.Func([], [Metadata], ['query']),
    'getTokenFee' : IDL.Func([], [IDL.Nat], ['query']),
    'getTokenInfo' : IDL.Func([], [TokenInfo], ['query']),
    'getTransaction' : IDL.Func([IDL.Nat], [TxRecord], ['query']),
    'getTransactions' : IDL.Func(
        [IDL.Nat, IDL.Nat],
        [IDL.Vec(TxRecord)],
        ['query'],
      ),
    'getUserApprovals' : IDL.Func(
        [IDL.Principal],
        [IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Nat))],
        ['query'],
      ),
    'getUserTransactionAmount' : IDL.Func(
        [IDL.Principal],
        [IDL.Nat],
        ['query'],
      ),
    'getUserTransactions' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Nat],
        [IDL.Vec(TxRecord)],
        ['query'],
      ),
    'historySize' : IDL.Func([], [IDL.Nat], ['query']),
    'logo' : IDL.Func([], [IDL.Text], ['query']),
    'mint' : IDL.Func([IDL.Principal, IDL.Nat], [TxReceipt], []),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'setFee' : IDL.Func([IDL.Nat], [], ['oneway']),
    'setFeeTo' : IDL.Func([IDL.Principal], [], ['oneway']),
    'setLogo' : IDL.Func([IDL.Text], [], ['oneway']),
    'setName' : IDL.Func([IDL.Text], [], ['oneway']),
    'setOwner' : IDL.Func([IDL.Principal], [], ['oneway']),
    'symbol' : IDL.Func([], [IDL.Text], ['query']),
    'totalSupply' : IDL.Func([], [IDL.Nat], ['query']),
    'transfer' : IDL.Func([IDL.Principal, IDL.Nat], [TxReceipt], []),
    'transferFrom' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Nat],
        [TxReceipt],
        [],
      ),
  });
  return Token;
};
export const init = ({ IDL }) => {
  return [
    IDL.Text,
    IDL.Text,
    IDL.Text,
    IDL.Nat8,
    IDL.Nat,
    IDL.Principal,
    IDL.Nat,
  ];
};
