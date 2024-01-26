import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Account {
  'owner' : Principal,
  'subaccount' : [] | [Subaccount],
}
export interface Allowance {
  'allowance' : Tokens,
  'expires_at' : [] | [Timestamp],
}
export interface AllowanceArgs { 'account' : Account, 'spender' : Account }
export interface ApproveArgs {
  'fee' : [] | [Tokens],
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Subaccount],
  'created_at_time' : [] | [Timestamp],
  'amount' : Tokens,
  'expected_allowance' : [] | [Tokens],
  'expires_at' : [] | [Timestamp],
  'spender' : Account,
}
export type ApproveError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'Duplicate' : { 'duplicate_of' : BlockIndex } } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'AllowanceChanged' : { 'current_allowance' : Tokens } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'Expired' : { 'ledger_time' : bigint } } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export type ApproveResult = { 'Ok' : BlockIndex } |
  { 'Err' : ApproveError };
export type Block = Value;
export type BlockIndex = bigint;
export interface BlockRange { 'blocks' : Array<Block> }
export type ChangeFeeCollector = { 'SetTo' : Account } |
  { 'Unset' : null };
export interface DataCertificate {
  'certificate' : [] | [Uint8Array | number[]],
  'hash_tree' : Uint8Array | number[],
}
export type Duration = bigint;
export interface FeatureFlags { 'icrc2' : boolean }
export interface GetBlocksArgs { 'start' : BlockIndex, 'length' : bigint }
export interface GetBlocksResponse {
  'certificate' : [] | [Uint8Array | number[]],
  'first_index' : BlockIndex,
  'blocks' : Array<Block>,
  'chain_length' : bigint,
  'archived_blocks' : Array<
    {
      'callback' : QueryBlockArchiveFn,
      'start' : BlockIndex,
      'length' : bigint,
    }
  >,
}
export interface GetTransactionsRequest { 'start' : TxIndex, 'length' : bigint }
export interface GetTransactionsResponse {
  'first_index' : TxIndex,
  'log_length' : bigint,
  'transactions' : Array<Transaction>,
  'archived_transactions' : Array<
    { 'callback' : QueryArchiveFn, 'start' : TxIndex, 'length' : bigint }
  >,
}
export interface InitArgs {
  'decimals' : [] | [number],
  'token_symbol' : string,
  'transfer_fee' : bigint,
  'metadata' : Array<[string, MetadataValue]>,
  'minting_account' : Account,
  'initial_balances' : Array<[Account, bigint]>,
  'fee_collector_account' : [] | [Account],
  'archive_options' : {
    'num_blocks_to_archive' : bigint,
    'trigger_threshold' : bigint,
    'max_message_size_bytes' : [] | [bigint],
    'cycles_for_archive_creation' : [] | [bigint],
    'node_max_memory_size_bytes' : [] | [bigint],
    'controller_id' : Principal,
  },
  'max_memo_length' : [] | [number],
  'token_name' : string,
  'feature_flags' : [] | [FeatureFlags],
}
export type LedgerArg = { 'Upgrade' : [] | [UpgradeArgs] } |
  { 'Init' : InitArgs };
export type Map = Array<[string, Value]>;
export type MetadataValue = { 'Int' : bigint } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string };
export type QueryArchiveFn = ActorMethod<
  [GetTransactionsRequest],
  TransactionRange
>;
export type QueryBlockArchiveFn = ActorMethod<[GetBlocksArgs], BlockRange>;
export type Subaccount = Uint8Array | number[];
export type Timestamp = bigint;
export type Tokens = bigint;
export interface Transaction {
  'burn' : [] | [
    {
      'from' : Account,
      'memo' : [] | [Uint8Array | number[]],
      'created_at_time' : [] | [bigint],
      'amount' : bigint,
    }
  ],
  'kind' : string,
  'mint' : [] | [
    {
      'to' : Account,
      'memo' : [] | [Uint8Array | number[]],
      'created_at_time' : [] | [bigint],
      'amount' : bigint,
    }
  ],
  'timestamp' : bigint,
  'transfer' : [] | [
    {
      'to' : Account,
      'fee' : [] | [bigint],
      'from' : Account,
      'memo' : [] | [Uint8Array | number[]],
      'created_at_time' : [] | [bigint],
      'amount' : bigint,
    }
  ],
}
export interface TransactionRange { 'transactions' : Array<Transaction> }
export interface TransferArg {
  'to' : Account,
  'fee' : [] | [Tokens],
  'memo' : [] | [Uint8Array | number[]],
  'from_subaccount' : [] | [Subaccount],
  'created_at_time' : [] | [Timestamp],
  'amount' : Tokens,
}
export type TransferError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'BadBurn' : { 'min_burn_amount' : Tokens } } |
  { 'Duplicate' : { 'duplicate_of' : BlockIndex } } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export interface TransferFromArgs {
  'to' : Account,
  'fee' : [] | [Tokens],
  'spender_subaccount' : [] | [Subaccount],
  'from' : Account,
  'memo' : [] | [Uint8Array | number[]],
  'created_at_time' : [] | [Timestamp],
  'amount' : Tokens,
}
export type TransferFromError = {
    'GenericError' : { 'message' : string, 'error_code' : bigint }
  } |
  { 'TemporarilyUnavailable' : null } |
  { 'InsufficientAllowance' : { 'allowance' : Tokens } } |
  { 'BadBurn' : { 'min_burn_amount' : Tokens } } |
  { 'Duplicate' : { 'duplicate_of' : BlockIndex } } |
  { 'BadFee' : { 'expected_fee' : Tokens } } |
  { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
  { 'TooOld' : null } |
  { 'InsufficientFunds' : { 'balance' : Tokens } };
export type TransferFromResult = { 'Ok' : BlockIndex } |
  { 'Err' : TransferFromError };
export type TransferResult = { 'Ok' : BlockIndex } |
  { 'Err' : TransferError };
export type TxIndex = bigint;
export interface UpgradeArgs {
  'token_symbol' : [] | [string],
  'transfer_fee' : [] | [bigint],
  'metadata' : [] | [Array<[string, MetadataValue]>],
  'change_fee_collector' : [] | [ChangeFeeCollector],
  'max_memo_length' : [] | [number],
  'token_name' : [] | [string],
  'feature_flags' : [] | [FeatureFlags],
}
export type Value = { 'Int' : bigint } |
  { 'Map' : Map } |
  { 'Nat' : bigint } |
  { 'Nat64' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<Value> };
export interface _SERVICE {
  'get_blocks' : ActorMethod<[GetBlocksArgs], GetBlocksResponse>,
  'get_data_certificate' : ActorMethod<[], DataCertificate>,
  'get_transactions' : ActorMethod<
    [GetTransactionsRequest],
    GetTransactionsResponse
  >,
  'icrc1_balance_of' : ActorMethod<[Account], Tokens>,
  'icrc1_decimals' : ActorMethod<[], number>,
  'icrc1_fee' : ActorMethod<[], Tokens>,
  'icrc1_metadata' : ActorMethod<[], Array<[string, MetadataValue]>>,
  'icrc1_minting_account' : ActorMethod<[], [] | [Account]>,
  'icrc1_name' : ActorMethod<[], string>,
  'icrc1_supported_standards' : ActorMethod<
    [],
    Array<{ 'url' : string, 'name' : string }>
  >,
  'icrc1_symbol' : ActorMethod<[], string>,
  'icrc1_total_supply' : ActorMethod<[], Tokens>,
  'icrc1_transfer' : ActorMethod<[TransferArg], TransferResult>,
  'icrc2_allowance' : ActorMethod<[AllowanceArgs], Allowance>,
  'icrc2_approve' : ActorMethod<[ApproveArgs], ApproveResult>,
  'icrc2_transfer_from' : ActorMethod<[TransferFromArgs], TransferFromResult>,
}
