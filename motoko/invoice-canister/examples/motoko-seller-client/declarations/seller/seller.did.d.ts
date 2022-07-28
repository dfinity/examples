import type { Principal } from '@dfinity/principal';
export type AccountIdentifier = { 'principal' : Principal } |
  { 'blob' : Array<number> } |
  { 'text' : string };
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
export interface Invoice {
  'id' : bigint,
  'permissions' : [] | [Permissions],
  'creator' : Principal,
  'destination' : AccountIdentifier,
  'token' : TokenVerbose,
  'paid' : boolean,
  'verifiedAtTime' : [] | [bigint],
  'amountPaid' : bigint,
  'details' : [] | [Details],
  'amount' : bigint,
}
export interface Permissions {
  'canGet' : Array<Principal>,
  'canVerify' : Array<Principal>,
}
export interface TokenVerbose {
  'decimals' : bigint,
  'meta' : [] | [{ 'Issuer' : string }],
  'symbol' : string,
}
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
  'check_license_status' : () => Promise<boolean>,
  'create_invoice' : () => Promise<CreateInvoiceResult>,
  'get_invoice' : (arg_0: bigint) => Promise<[] | [Invoice]>,
  'reset_license' : () => Promise<undefined>,
  'verify_invoice' : (arg_0: bigint) => Promise<VerifyInvoiceResult>,
}
