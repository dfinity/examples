
# Payments - Invoice Canister

As we look to refine the developer experience around payments, we concluded that in some instances the ledger canister interface may be too “low level”. For example, a canister that would like to access/implement a payment system would need to implement from scratch things like protection against double spending against the ledger interface. For that reason, we propose to design an interface that will make it easier for a typical canister to add payment functionality.

## Goals

Goals for this project are as follows:
1. Solution should be simple to include and develop against locally
2. Canister can easily check its balance
3. Canister can verify that a payment has been satisfied
4. User can submit payment from a wallet
5. Design should be compatible with BTC, ETH, and SNS ledgers as they become available
  

## Non-goals

* We do not intend to change the ICP ledger
* This interface won't specifically handle minting cycles or other secondary ledger features
* Handling escrow payments
* Automating recurring payments

## Open Questions

* Should this be a new canister type in `dfx`, a single centralized canister on the NNS subnet, or both?

## The Interface
```
// invoice.did
type VerifyInvoiceSuccess = 
 variant {
   AlreadyVerified: record {invoice: Invoice;};
   Paid: record {invoice: Invoice;};
 };
type VerifyInvoiceResult = 
 variant {
   err: VerifyInvoiceErr;
   ok: VerifyInvoiceSuccess;
 };
type VerifyInvoiceErr = 
 record {
   kind:
    variant {
      Expired;
      InvalidAccount;
      InvalidInvoiceId;
      InvalidToken;
      NotAuthorized;
      NotFound;
      NotYetPaid;
      Other;
      TransferError;
    };
   message: opt text;
 };
type VerifyInvoiceArgs = record {id: nat;};
type TransferSuccess = record {blockHeight: nat64;};
type TransferResult = 
 variant {
   err: TransferError;
   ok: TransferSuccess;
 };
type TransferError = 
 record {
   kind:
    variant {
      BadFee;
      InsufficientFunds;
      InvalidDestination;
      InvalidToken;
      Other;
    };
   message: opt text;
 };
type TransferArgs = 
 record {
   amount: nat;
   destination: AccountIdentifier;
   token: Token;
 };
type TokenVerbose = 
 record {
   decimals: int;
   meta: opt record {Issuer: text;};
   symbol: text;
 };
type Token = record {symbol: text;};
type Time = int;
type Permissions = 
 record {
   canGet: vec principal;
   canVerify: vec principal;
 };
type Invoice = 
 record {
   amount: nat;
   amountPaid: nat;
   creator: principal;
   destination: AccountIdentifier;
   details: opt Details;
   id: nat;
   paid: bool;
   permissions: opt Permissions;
   token: TokenVerbose;
   verifiedAtTime: opt Time;
 };
type GetInvoiceSuccess = record {invoice: Invoice;};
type GetInvoiceResult = 
 variant {
   err: GetInvoiceErr;
   ok: GetInvoiceSuccess;
 };
type GetInvoiceErr = 
 record {
   kind: variant {
           InvalidInvoiceId;
           NotAuthorized;
           NotFound;
           Other;
         };
   message: opt text;
 };
type GetInvoiceArgs = record {id: nat;};
type GetBalanceSuccess = record {balance: nat;};
type GetBalanceResult = 
 variant {
   err: GetBalanceErr;
   ok: GetBalanceSuccess;
 };
type GetBalanceErr = 
 record {
   kind: variant {
           InvalidToken;
           NotFound;
           Other;
         };
   message: opt text;
 };
type GetBalanceArgs = record {token: Token;};
type GetAccountIdentifierSuccess = record {
                                     accountIdentifier: AccountIdentifier;};
type GetAccountIdentifierResult = 
 variant {
   err: GetAccountIdentifierErr;
   ok: GetAccountIdentifierSuccess;
 };
type GetAccountIdentifierErr = 
 record {
   kind: variant {
           InvalidToken;
           Other;
         };
   message: opt text;
 };
type GetAccountIdentifierArgs = 
 record {
   "principal": principal;
   token: Token;
 };
type Details = 
 record {
   description: text;
   meta: blob;
 };
type CreateInvoiceSuccess = record {invoice: Invoice;};
type CreateInvoiceResult = 
 variant {
   err: CreateInvoiceErr;
   ok: CreateInvoiceSuccess;
 };
type CreateInvoiceErr = 
 record {
   kind:
    variant {
      InvalidAmount;
      InvalidDestination;
      InvalidDetails;
      InvalidToken;
      Other;
    };
   message: opt text;
 };
type CreateInvoiceArgs = 
 record {
   amount: nat;
   details: opt Details;
   permissions: opt Permissions;
   token: Token;
 };
type AccountIdentifier__1 = 
 variant {
   "blob": blob;
   "principal": principal;
   "text": text;
 };
type AccountIdentifierToBlobSuccess = blob;
type AccountIdentifierToBlobResult = 
 variant {
   err: AccountIdentifierToBlobErr;
   ok: AccountIdentifierToBlobSuccess;
 };
type AccountIdentifierToBlobErr = 
 record {
   kind: variant {
           InvalidAccountIdentifier;
           Other;
         };
   message: opt text;
 };
type AccountIdentifier = 
 variant {
   "blob": blob;
   "principal": principal;
   "text": text;
 };
service : {
  accountIdentifierToBlob: (AccountIdentifier__1) ->
   (AccountIdentifierToBlobResult);
  create_invoice: (CreateInvoiceArgs) -> (CreateInvoiceResult);
  get_account_identifier: (GetAccountIdentifierArgs) ->
   (GetAccountIdentifierResult) query;
  get_balance: (GetBalanceArgs) -> (GetBalanceResult);
  get_invoice: (GetInvoiceArgs) -> (GetInvoiceResult) query;
  remaining_cycles: () -> (nat) query;
  transfer: (TransferArgs) -> (TransferResult);
  verify_invoice: (VerifyInvoiceArgs) -> (VerifyInvoiceResult);
}


```


## Design Choices

The goal here was to design a flow where a client application such as a webpage, could initiate a payment flow that could be used to gate services or transfer ownership of assets.

The Invoice Canister will consolidate payments into a single balance per token, which will be the location that you can then transfer from and check your balance. The implementation may differ slightly for Bitcoin versus ICP, but the Invoice Canister will handle the implementation and abstract those differences into a single API.

### Basic Payment Flow ( hypothetical )

A canister smart contract can receive a request to purchase, create an invoice, and store the Principal of the caller and the UUID of the invoice.

Once the payment has been satisfied, the canister can check the status of the payment with `validate_payment`, while the Invoice canister checks the ledger. The canister can then present the status to the client, and satisfy the payment flow.
