import type { Principal } from '@dfinity/principal';
export interface AccountBalanceArgs { 'account' : Address }
export type Address = Array<number>;
export type BlockIndex = bigint;
export type Memo = bigint;
export type SubAccount = Array<number>;
export interface TimeStamp { 'timestamp_nanos' : bigint }
export interface Token { 'e8s' : bigint }
export interface TransferArgs {
  'to' : Address,
  'fee' : Token,
  'memo' : Memo,
  'from_subaccount' : [] | [SubAccount],
  'created_at_time' : [] | [TimeStamp],
  'amount' : Token,
}
export type TransferError = {
    'TxTooOld' : { 'allowed_window_nanos' : bigint }
  } |
  { 'BadFee' : { 'expected_fee' : Token } } |
  { 'TxDuplicate' : { 'duplicate_of' : BlockIndex } } |
  { 'TxCreatedInFuture' : null } |
  { 'InsufficientFunds' : { 'balance' : Token } };
export type TransferResult = { 'Ok' : BlockIndex } |
  { 'Err' : TransferError };
export interface _SERVICE {
  'account_balance' : (arg_0: AccountBalanceArgs) => Promise<Token>,
  'transfer' : (arg_0: TransferArgs) => Promise<TransferResult>,
}
