import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Subaccount],
}
export interface Account__1 {
  'owner' : Principal,
  'subaccount' : [] | [Uint8Array | number[]],
}
export type BalanceOfRequest = Array<Account__1>;
export type BalanceOfResponse = Array<bigint>;
export interface NftCanister {
  'claimCollection' : ActorMethod<[], undefined>,
  'collectionHasBeenClaimed' : ActorMethod<[], boolean>,
  'getCollectionOwner' : ActorMethod<[], Principal>,
  'icrc10_supported_standards' : ActorMethod<[], SupportedStandards>,
  'icrc7_atomic_batch_transfers' : ActorMethod<[], [] | [boolean]>,
  'icrc7_balance_of' : ActorMethod<[BalanceOfRequest], BalanceOfResponse>,
  'icrc7_collection_metadata' : ActorMethod<[], Array<[string, Value__1]>>,
  'icrc7_default_take_value' : ActorMethod<[], [] | [bigint]>,
  'icrc7_description' : ActorMethod<[], [] | [string]>,
  'icrc7_logo' : ActorMethod<[], [] | [string]>,
  'icrc7_max_memo_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_query_batch_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_take_value' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_update_batch_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_name' : ActorMethod<[], string>,
  'icrc7_owner_of' : ActorMethod<[OwnerOfRequest], OwnerOfResponse>,
  'icrc7_permitted_drift' : ActorMethod<[], [] | [bigint]>,
  'icrc7_supply_cap' : ActorMethod<[], [] | [bigint]>,
  'icrc7_symbol' : ActorMethod<[], string>,
  'icrc7_token_metadata' : ActorMethod<
    [Array<bigint>],
    Array<[] | [Array<[string, Value__1]>]>
  >,
  'icrc7_tokens' : ActorMethod<[[] | [bigint], [] | [bigint]], Array<bigint>>,
  'icrc7_tokens_of' : ActorMethod<
    [Account, [] | [bigint], [] | [bigint]],
    Array<bigint>
  >,
  'icrc7_total_supply' : ActorMethod<[], bigint>,
  'icrc7_transfer' : ActorMethod<
    [Array<TransferArg>],
    Array<[] | [TransferResult]>
  >,
  'icrc7_tx_window' : ActorMethod<[], [] | [bigint]>,
  'mint' : ActorMethod<[Account], Array<SetNFTResult>>,
}
export type OwnerOfRequest = Array<bigint>;
export type OwnerOfResponse = Array<[] | [Account__1]>;
export type SetNFTError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TokenExists' : null } |
  { 'NonExistingTokenId' : null } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null };
export type SetNFTResult = { 'Ok' : [] | [bigint] } |
  { 'Err' : SetNFTError } |
  { 'GenericError' : { 'message' : string, 'error_code' : bigint } };
export type Subaccount = Uint8Array | number[];
export type SupportedStandards = Array<{ 'url' : string, 'name' : string }>;
export interface TransferArg {
  'to' : Account__1,
  'token_id' : bigint,
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [bigint],
}
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'Duplicate' : { 'duplicate_of' : bigint } } |
  { 'NonExistingTokenId' : null } |
  { 'Unauthorized' : null } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'InvalidRecipient' : null } |
  { 'GenericBatchError' : { 'message' : string, 'error_code' : bigint } } |
  { 'TooOld' : null };
export type TransferResult = { 'Ok' : bigint } |
  { 'Err' : TransferError };
export type Value__1 = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value__1]> } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<Value__1> };
export interface _SERVICE extends NftCanister {}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
