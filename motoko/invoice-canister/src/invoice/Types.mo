import Result     "mo:base/Result";
import Time       "mo:base/Time";

module {
/**
* Base Types
*/
// #region Base Types
  public type Token = {
    symbol : Text;
  };
  public type TokenVerbose = {
    symbol : Text;
    decimals : Int;
    meta : ?{
      Issuer : Text;
    };
  };
  public type AccountIdentifier = {
    #text : Text;
    #principal : Principal;
    #blob : Blob;
  };
  public type Details = {
    description : Text;
    meta : Blob;
  };
  public type Permissions = {
      canGet : [Principal];
      canVerify : [Principal];
  };
  public type Invoice = {
    id : Nat;
    creator : Principal;
    details : ?Details;
    permissions : ?Permissions;
    amount : Nat;
    amountPaid : Nat;
    token : TokenVerbose;
    verifiedAtTime : ?Time.Time;
    paid : Bool;
    destination : AccountIdentifier;
  };
// #endregion

/**
* Service Args and Result Types
*/

// #region create_invoice
  public type CreateInvoiceArgs = {
    amount : Nat;
    token : Token;
    permissions: ?Permissions;
    details : ?Details;
  };
  public type CreateInvoiceResult = Result.Result<CreateInvoiceSuccess, CreateInvoiceErr>;
  public type CreateInvoiceSuccess = {
    invoice : Invoice;
  };
  public type CreateInvoiceErr = {
    message : ?Text;
    kind : {
      #BadSize;
      #InvalidToken;
      #InvalidAmount;
      #InvalidDestination;
      #InvalidDetails;
      #MaxInvoicesReached;
      #Other;
    };
  };
// #endregion

// #region Get Destination Account Identifier
  public type GetDestinationAccountIdentifierArgs = {
    token : Token;
    caller : Principal;
    invoiceId : Nat;
  };
  public type GetDestinationAccountIdentifierResult = Result.Result<GetDestinationAccountIdentifierSuccess, GetDestinationAccountIdentifierErr>;
  public type GetDestinationAccountIdentifierSuccess = {
    accountIdentifier : AccountIdentifier;
  };
  public type GetDestinationAccountIdentifierErr = {
    message : ?Text;
    kind : {
        #InvalidToken;
        #InvalidInvoiceId;
        #Other;
    };
  };
// #endregion

// #region get_invoice
  public type GetInvoiceArgs = {
    id : Nat;
  };
  public type GetInvoiceResult = Result.Result<GetInvoiceSuccess, GetInvoiceErr>;
  public type GetInvoiceSuccess = {
    invoice : Invoice;
  };
  public type GetInvoiceErr = {
    message : ?Text;
    kind : {
      #InvalidInvoiceId;
      #NotFound;
      #NotAuthorized;
      #Other;
    };
  };
// #endregion

// #region get_balance
  public type GetBalanceArgs = {
    token : Token;
  };
  public type GetBalanceResult = Result.Result<GetBalanceSuccess, GetBalanceErr>;
  public type GetBalanceSuccess = {
    balance : Nat;
  };
  public type GetBalanceErr = {
    message : ?Text;
    kind : {
      #InvalidToken;
      #NotFound;
      #Other;
    };
  };
// #endregion

// #region verify_invoice
  public type VerifyInvoiceArgs = {
    id : Nat;
  };
  public type VerifyInvoiceResult = Result.Result<VerifyInvoiceSuccess, VerifyInvoiceErr>;
  public type VerifyInvoiceSuccess = {
    #Paid : {
      invoice : Invoice;
    };
    #AlreadyVerified : {
      invoice : Invoice;
    };
  };
  type VerifyInvoiceErr = {
    message : ?Text;
    kind : {
      #InvalidInvoiceId;
      #NotFound;
      #NotYetPaid;
      #NotAuthorized;
      #Expired;
      #TransferError;
      #InvalidToken;
      #InvalidAccount;
      #Other;
    };
  };
// #endregion

// #region transfer
  public type TransferArgs = {
    amount : Nat;
    token : Token;
    destination : AccountIdentifier;
  };
  public type TransferResult = Result.Result<TransferSuccess, TransferError>;
  public type TransferSuccess = {
    blockHeight : Nat64;
  };
  public type TransferError = {
    message : ?Text;
    kind : {
      #BadFee;
      #InsufficientFunds;
      #InvalidToken;
      #InvalidDestination;
      #Other;
    };
  };
// #endregion

// #region get_caller_identifier
  public type GetAccountIdentifierArgs = {
    token : Token;
    principal : Principal;
  };
  public type GetAccountIdentifierResult = Result.Result<GetAccountIdentifierSuccess, GetAccountIdentifierErr>;
  public type GetAccountIdentifierSuccess = {
    accountIdentifier : AccountIdentifier;
  };
  public type GetAccountIdentifierErr = {
    message : ?Text;
    kind : {
      #InvalidToken;
      #Other;
    };
  };
// #endregion

// #region accountIdentifierToBlob
  public type AccountIdentifierToBlobArgs = {
    accountIdentifier : AccountIdentifier;
    canisterId : ?Principal;
  };
  public type AccountIdentifierToBlobResult = Result.Result<AccountIdentifierToBlobSuccess, AccountIdentifierToBlobErr>;
  public type AccountIdentifierToBlobSuccess = Blob;
  public type AccountIdentifierToBlobErr = {
    message : ?Text;
    kind : {
      #InvalidAccountIdentifier;
      #Other;
    };
  };
// #endregion

// #region accountIdentifierToText
  public type AccountIdentifierToTextArgs = {
    accountIdentifier : AccountIdentifier;
    canisterId : ?Principal;
  };
  public type AccountIdentifierToTextResult = Result.Result<AccountIdentifierToTextSuccess, AccountIdentifierToTextErr>;
  public type AccountIdentifierToTextSuccess = Text;
  public type AccountIdentifierToTextErr = {
    message : ?Text;
    kind : {
      #InvalidAccountIdentifier;
      #Other;
    };
  };
// #endregion

// #region ICP Transfer
  public type Memo = Nat64;
  public type SubAccount = Blob;
  public type TimeStamp = {
    timestamp_nanos : Nat64;
  };
  public type ICPTokens = {
    e8s : Nat64;
  };
  public type ICPTransferError = {
    message : ?Text;
    kind : {
      #BadFee : {
        expected_fee : ICPTokens;
      };
      #InsufficientFunds : {
        balance : ICPTokens;
      };
      #TxTooOld : {
        allowed_window_nanos : Nat64;
      };
      #TxCreatedInFuture;
      #TxDuplicate : {
        duplicate_of : Nat;
      };
      #Other;
    }
  };

  public type ICPTransferArgs = {
    memo : Memo;
    amount : ICPTokens;
    fee : ICPTokens;
    from_subaccount : ?SubAccount;
    to : AccountIdentifier;
    created_at_time : ?TimeStamp;
  };

  public type ICPTransferResult = Result.Result<TransferSuccess, ICPTransferError>;
// #endregion
};
