export const idlFactory = ({ IDL }) => {
  const InitArgs = IDL.Record({ 'ledger_id' : IDL.Principal });
  const TxId = IDL.Nat;
  const Account = IDL.Record({
    'owner' : IDL.Principal,
    'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const GetAccountTransactionsArgs = IDL.Record({
    'max_results' : IDL.Nat,
    'start' : IDL.Opt(TxId),
    'account' : Account,
  });
  const Burn = IDL.Record({
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'spender' : IDL.Opt(Account),
  });
  const Mint = IDL.Record({
    'to' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
  });
  const Approve = IDL.Record({
    'fee' : IDL.Opt(IDL.Nat),
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'expected_allowance' : IDL.Opt(IDL.Nat),
    'expires_at' : IDL.Opt(IDL.Nat64),
    'spender' : Account,
  });
  const Transfer = IDL.Record({
    'to' : Account,
    'fee' : IDL.Opt(IDL.Nat),
    'from' : Account,
    'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'created_at_time' : IDL.Opt(IDL.Nat64),
    'amount' : IDL.Nat,
    'spender' : IDL.Opt(Account),
  });
  const Transaction = IDL.Record({
    'burn' : IDL.Opt(Burn),
    'kind' : IDL.Text,
    'mint' : IDL.Opt(Mint),
    'approve' : IDL.Opt(Approve),
    'timestamp' : IDL.Nat64,
    'transfer' : IDL.Opt(Transfer),
  });
  const TransactionWithId = IDL.Record({
    'id' : TxId,
    'transaction' : Transaction,
  });
  const GetTransactions = IDL.Record({
    'transactions' : IDL.Vec(TransactionWithId),
    'oldest_tx_id' : IDL.Opt(TxId),
  });
  const GetTransactionsErr = IDL.Record({ 'message' : IDL.Text });
  const GetTransactionsResult = IDL.Variant({
    'Ok' : GetTransactions,
    'Err' : GetTransactionsErr,
  });
  const SubAccount = IDL.Vec(IDL.Nat8);
  const ListSubaccountsArgs = IDL.Record({
    'owner' : IDL.Principal,
    'start' : IDL.Opt(SubAccount),
  });
  return IDL.Service({
    'get_account_transactions' : IDL.Func(
        [GetAccountTransactionsArgs],
        [GetTransactionsResult],
        [],
      ),
    'ledger_id' : IDL.Func([], [IDL.Principal], ['query']),
    'list_subaccounts' : IDL.Func(
        [ListSubaccountsArgs],
        [IDL.Vec(SubAccount)],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => {
  const InitArgs = IDL.Record({ 'ledger_id' : IDL.Principal });
  return [InitArgs];
};
