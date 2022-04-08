import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat8";
import Nat16 "mo:base/Nat16";
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import List "mo:base/List";
import Blob "mo:base/Blob";
import Bool "mo:base/Bool";
import Principal "mo:base/Principal";

module {
  public type ApiError = {
    #Unauthorized;
    #InvalidTokenId;
    #ZeroAddress;
    #Other;
  };

  public type Result<S, E> = {
    #Ok : S;
    #Err : E;
  };

  public type OwnerResult = Result<Principal, ApiError>;
  public type TxReceipt = Result<Nat, ApiError>;

  public type InterfaceId = {
    #Approval;
    #TransactionHistory;
    #Mint;
    #Burn;
    #TransferNotification;
  };

  public type LogoResult = {
    logo_type: Text;
    data: Text;
  };

  public type ExtendedMetadataResult = {
    metadata_desc: MetadataDesc;
    token_id: TokenId;
  };

  public type TokenId = Nat64;

  public type MetadataResult = Result<MetadataDesc, ApiError>;

  public type MetadataDesc = List.List<MetadataPart>;

  public type MetadataPart = {
    purpose: MetadataPurpose;
    key_val_data: List.List<MetadataKeyVal>;
    data: Blob;
  };

  public type MetadataPurpose = {
    #Preview;
    #Rendered;
  };
  
  public type MetadataKeyVal = {
    key: Text;
    val: MetadataVal;
  };

  public type MetadataVal = {
    #TextContent : Text;
    #BlobContent : Blob;
    #NatContent : Nat;
    #Nat8Content: Nat8;
    #Nat16Content: Nat16;
    #Nat32Content: Nat32;
    #Nat64Content: Nat64;
  };

  public type TransactionResult = {
    fee: Nat;
    transaction_type: TransactionType;
  };

  public type TransactionType = {
    #Transfer: {
      token_id: Nat64;
      from: Principal;
      to: Principal;
    };
    #TransferFrom: {
      token_id: Nat64;
      from: Principal;
      to: Principal;
    };
    #Approve: {
      token_id: Nat64;
      from: Principal;
      to: Principal;
    };
    #SetApprovalForAll: {
      from: Principal;
      to: Principal;
    };
    #Mint: {
      from: Principal;
      to: Principal;
    };
    #Burn: {
      token_id: Nat64;
    }
  };

  public type MintReceipt = Result<MintReceiptPart, ApiError>;

  public type MintReceiptPart = {
    token_id: Nat64;
    id: Nat;
  };

  public type Balance = Nat;
  public type Memo = Blob;
  public type SubAccount = List.List<Nat8>;
  public type TokenIdentifier = Text;
  public type TokenIndex = Nat32;
  public type AccountIdentifier = Text;

  public type User = {
    #address: AccountIdentifier;
    #principal: Principal;
  };

  public type TransferRequest = {
    amount: Balance;
    from: User;
    memo: Memo;
    notify: Bool;
    subaccount: ?SubAccount;
    to: User;
    token: TokenIdentifier;
  };

  public type TransferResponseError = {
    #CannotNotify: AccountIdentifier;
    #InsufficientBalance;
    #InvalidToken: AccountIdentifier;
    #Other: Text;
    #Rejected;
    #Unauthorized: AccountIdentifier;
  };

  public type TransferResponse = Result<Balance, TransferResponseError>;

  public type MintRequest = {
    metadata: ?MetadataContainer;
    to: User;
  };

  public type CommonError = {
    #InvalidToken: TokenIdentifier;
    #Other: Text;
  };

  public type AccountIdentifierReturn = Result<AccountIdentifier, CommonError>;

  public type BalanceReturn = Result<Balance, CommonError>;

  public type MetadataReturn = Result<Metadata, CommonError>;

  public type TokenMetadata = {
    account_identifier: AccountIdentifier;
    metadata: Metadata;
    token_identifier: TokenIdentifier;
    principal: Principal;
  };

  public type Metadata = {
    #fungible: {
      name: Text;
      symbol: Text;
      decimals: Nat8;
      metadata: ?MetadataContainer;
    };
    #nonfungible: ?MetadataContainer;
  };

  public type MetadataContainer = {
    #data: List.List<MetadataValue>;
    #blob: Blob;
    #json: Text;
  };

  public type MetadataValue = {
    text: Value;
  };

  public type Value = {
    #text: Text;
    #blob: Blob;
    #nat: Nat;
    #nat8: Nat8;
  };

  public type TransactionId = Nat;
  public type Date = Nat64;

  public type Transaction = {
    txid: TransactionId;
    request: TransferRequest;
    date: Date;
  };

  public type TransactionRequestFilter = {
    #txid: TransactionId;
    #user: User;
    #date: Date;
    #page: Nat;
  };

  public type TransactionRequest = {
    transaction_query: TransactionRequestFilter;
    token: TokenIdentifier;
  };

  public type TransactionsResult = Result<List.List<Transaction>, CommonError>;
};
