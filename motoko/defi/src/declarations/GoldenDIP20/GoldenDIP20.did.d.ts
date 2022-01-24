import type { Principal } from '@dfinity/principal';
export interface Metadata {
  'fee' : bigint,
  'decimals' : number,
  'owner' : Principal,
  'logo' : string,
  'name' : string,
  'totalSupply' : bigint,
  'symbol' : string,
}
export type Operation = { 'transferFrom' : null } |
  { 'burn' : null } |
  { 'mint' : null } |
  { 'approve' : null } |
  { 'transfer' : null };
export type Time = bigint;
export interface Token {
  'allowance' : (arg_0: Principal, arg_1: Principal) => Promise<bigint>,
  'approve' : (arg_0: Principal, arg_1: bigint) => Promise<TxReceipt>,
  'balanceOf' : (arg_0: Principal) => Promise<bigint>,
  'burn' : (arg_0: bigint) => Promise<TxReceipt>,
  'decimals' : () => Promise<number>,
  'getAllowanceSize' : () => Promise<bigint>,
  'getHolders' : (arg_0: bigint, arg_1: bigint) => Promise<
      Array<[Principal, bigint]>
    >,
  'getMetadata' : () => Promise<Metadata>,
  'getTokenFee' : () => Promise<bigint>,
  'getTokenInfo' : () => Promise<TokenInfo>,
  'getTransaction' : (arg_0: bigint) => Promise<TxRecord>,
  'getTransactions' : (arg_0: bigint, arg_1: bigint) => Promise<
      Array<TxRecord>
    >,
  'getUserApprovals' : (arg_0: Principal) => Promise<
      Array<[Principal, bigint]>
    >,
  'getUserTransactionAmount' : (arg_0: Principal) => Promise<bigint>,
  'getUserTransactions' : (
      arg_0: Principal,
      arg_1: bigint,
      arg_2: bigint,
    ) => Promise<Array<TxRecord>>,
  'historySize' : () => Promise<bigint>,
  'logo' : () => Promise<string>,
  'mint' : (arg_0: Principal, arg_1: bigint) => Promise<TxReceipt>,
  'name' : () => Promise<string>,
  'setFee' : (arg_0: bigint) => Promise<undefined>,
  'setFeeTo' : (arg_0: Principal) => Promise<undefined>,
  'setLogo' : (arg_0: string) => Promise<undefined>,
  'setName' : (arg_0: string) => Promise<undefined>,
  'setOwner' : (arg_0: Principal) => Promise<undefined>,
  'symbol' : () => Promise<string>,
  'totalSupply' : () => Promise<bigint>,
  'transfer' : (arg_0: Principal, arg_1: bigint) => Promise<TxReceipt>,
  'transferFrom' : (
      arg_0: Principal,
      arg_1: Principal,
      arg_2: bigint,
    ) => Promise<TxReceipt>,
}
export interface TokenInfo {
  'holderNumber' : bigint,
  'deployTime' : Time,
  'metadata' : Metadata,
  'historySize' : bigint,
  'cycles' : bigint,
  'feeTo' : Principal,
}
export type TransactionStatus = { 'inprogress' : null } |
  { 'failed' : null } |
  { 'succeeded' : null };
export type TxReceipt = { 'Ok' : bigint } |
  {
    'Err' : { 'InsufficientAllowance' : null } |
      { 'InsufficientBalance' : null } |
      { 'ErrorOperationStyle' : null } |
      { 'Unauthorized' : null } |
      { 'LedgerTrap' : null } |
      { 'ErrorTo' : null } |
      { 'Other' : null } |
      { 'BlockUsed' : null } |
      { 'AmountTooSmall' : null }
  };
export interface TxRecord {
  'op' : Operation,
  'to' : Principal,
  'fee' : bigint,
  'status' : TransactionStatus,
  'from' : Principal,
  'timestamp' : Time,
  'caller' : [] | [Principal],
  'index' : bigint,
  'amount' : bigint,
}
export interface _SERVICE extends Token {}
