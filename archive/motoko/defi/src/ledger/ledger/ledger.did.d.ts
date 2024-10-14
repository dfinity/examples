import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AccountBalanceArgs { 'account' : AccountIdentifier }
export type AccountIdentifier = Uint8Array | number[];
export type BlockIndex = bigint;
export type Memo = bigint;
export type SubAccount = Uint8Array | number[];
export interface TimeStamp { 'timestamp_nanos' : bigint }
export interface Tokens { 'e8s' : bigint }
export interface TransferArgs {
  'to' : AccountIdentifier,
  'fee' : Tokens,
  'memo' : Memo,
  'from_subaccount' : [] | [SubAccount],
  'created_at_time' : [] | [TimeStamp],
  'amount' : Tokens,
}
export type TransferError = {
    'TxTooOld' : { 'allowed_window_nanos' : bigint }
  } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'TxDuplicate' : { 'duplicate_of' : BlockIndex } } |
  { 'TxCreatedInFuture' : null } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export type TransferResult = { 'Ok' : BlockIndex } |
  { 'Err' : TransferError };
export interface _SERVICE {
  'account_balance' : ActorMethod<[AccountBalanceArgs], Tokens>,
  'transfer' : ActorMethod<[TransferArgs], TransferResult>,
}
