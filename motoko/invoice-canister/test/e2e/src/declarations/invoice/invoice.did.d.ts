import type { Principal } from '@dfinity/principal';
export type AccountIdentifier = { 'principal' : Principal } |
  { 'blob' : Array<number> } |
  { 'text' : string };
export interface AccountIdentifierToBlobErr {
  'kind' : { 'InvalidAccountIdentifier' : null } |
    { 'Other' : null },
  'message' : [] | [string],
}
export type AccountIdentifierToBlobResult = {
    'ok' : AccountIdentifierToBlobSuccess
  } |
  { 'err' : AccountIdentifierToBlobErr };
export type AccountIdentifierToBlobSuccess = Array<number>;
export type AccountIdentifier__1 = { 'principal' : Principal } |
  { 'blob' : Array<number> } |
  { 'text' : string };
export interface CreateInvoiceArgs {
  'permissions' : [] | [Permissions],
  'token' : Token,
  'details' : [] | [Details],
  'amount' : bigint,
}
export interface CreateInvoiceErr {
  'kind' : { 'InvalidDetails' : null } |
    { 'InvalidAmount' : null } |
    { 'InvalidDestination' : null } |
    { 'MaxInvoicesReached' : null } |
    { 'BadSize' : null } |
    { 'InvalidToken' : null } |
    { 'Other' : null },
  'message' : [] | [string],
}
export type CreateInvoiceResult = { 'ok' : CreateInvoiceSuccess } |
  { 'err' : CreateInvoiceErr };
export interface CreateInvoiceSuccess { 'invoice' : Invoice }
export interface Details { 'meta' : Array<number>, 'description' : string }
export interface GetAccountIdentifierArgs {
  'principal' : Principal,
  'token' : Token,
}
export interface GetAccountIdentifierErr {
  'kind' : { 'InvalidToken' : null } |
    { 'Other' : null },
  'message' : [] | [string],
}
export type GetAccountIdentifierResult = {
    'ok' : GetAccountIdentifierSuccess
  } |
  { 'err' : GetAccountIdentifierErr };
export interface GetAccountIdentifierSuccess {
  'accountIdentifier' : AccountIdentifier,
}
export interface GetBalanceArgs { 'token' : Token }
export interface GetBalanceErr {
  'kind' : { 'NotFound' : null } |
    { 'InvalidToken' : null } |
    { 'Other' : null },
  'message' : [] | [string],
}
export type GetBalanceResult = { 'ok' : GetBalanceSuccess } |
  { 'err' : GetBalanceErr };
export interface GetBalanceSuccess { 'balance' : bigint }
export interface GetInvoiceArgs { 'id' : bigint }
export interface GetInvoiceErr {
  'kind' : { 'NotFound' : null } |
    { 'NotAuthorized' : null } |
    { 'InvalidInvoiceId' : null } |
    { 'Other' : null },
  'message' : [] | [string],
}
export type GetInvoiceResult = { 'ok' : GetInvoiceSuccess } |
  { 'err' : GetInvoiceErr };
export interface GetInvoiceSuccess { 'invoice' : Invoice }
export interface Invoice {
  'id' : bigint,
  'permissions' : [] | [Permissions],
  'creator' : Principal,
  'destination' : AccountIdentifier,
  'token' : TokenVerbose,
  'paid' : boolean,
  'verifiedAtTime' : [] | [Time],
  'amountPaid' : bigint,
  'details' : [] | [Details],
  'amount' : bigint,
}
export interface Permissions {
  'canGet' : Array<Principal>,
  'canVerify' : Array<Principal>,
}
export type Time = bigint;
export interface Token { 'symbol' : string }
export interface TokenVerbose {
  'decimals' : bigint,
  'meta' : [] | [{ 'Issuer' : string }],
  'symbol' : string,
}
export interface TransferArgs {
  'destination' : AccountIdentifier,
  'token' : Token,
  'amount' : bigint,
}
export interface TransferError {
  'kind' : { 'InvalidDestination' : null } |
    { 'BadFee' : null } |
    { 'InvalidToken' : null } |
    { 'Other' : null } |
    { 'InsufficientFunds' : null },
  'message' : [] | [string],
}
export type TransferResult = { 'ok' : TransferSuccess } |
  { 'err' : TransferError };
export interface TransferSuccess { 'blockHeight' : bigint }
export interface VerifyInvoiceArgs { 'id' : bigint }
export interface VerifyInvoiceErr {
  'kind' : { 'InvalidAccount' : null } |
    { 'TransferError' : null } |
    { 'NotFound' : null } |
    { 'NotAuthorized' : null } |
    { 'InvalidToken' : null } |
    { 'InvalidInvoiceId' : null } |
    { 'Other' : null } |
    { 'NotYetPaid' : null } |
    { 'Expired' : null },
  'message' : [] | [string],
}
export type VerifyInvoiceResult = { 'ok' : VerifyInvoiceSuccess } |
  { 'err' : VerifyInvoiceErr };
export type VerifyInvoiceSuccess = { 'Paid' : { 'invoice' : Invoice } } |
  { 'AlreadyVerified' : { 'invoice' : Invoice } };
export interface _SERVICE {
  'accountIdentifierToBlob' : (arg_0: AccountIdentifier__1) => Promise<
      AccountIdentifierToBlobResult
    >,
  'create_invoice' : (arg_0: CreateInvoiceArgs) => Promise<CreateInvoiceResult>,
  'get_account_identifier' : (arg_0: GetAccountIdentifierArgs) => Promise<
      GetAccountIdentifierResult
    >,
  'get_balance' : (arg_0: GetBalanceArgs) => Promise<GetBalanceResult>,
  'get_invoice' : (arg_0: GetInvoiceArgs) => Promise<GetInvoiceResult>,
  'transfer' : (arg_0: TransferArgs) => Promise<TransferResult>,
  'verify_invoice' : (arg_0: VerifyInvoiceArgs) => Promise<VerifyInvoiceResult>,
}
