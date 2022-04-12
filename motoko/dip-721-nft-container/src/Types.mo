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
  public type Dip721NonFungibleToken = {
    logo: LogoResult;
    name: Text;
    symbol: Text;
  };

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
  
  public type TransactionId = Nat;
  public type TokenId = Nat64;

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

  public type Nft = {
    owner: Principal;
    id: Nat64;
    metadata: MetadataDesc;
    content: List.List<Nat8>;
  };

  public type ExtendedMetadataResult = {
    metadata_desc: MetadataDesc;
    token_id: TokenId;
  };

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
      token_id: TokenId;
      from: Principal;
      to: Principal;
    };
    #TransferFrom: {
      token_id: TokenId;
      from: Principal;
      to: Principal;
    };
    #Approve: {
      token_id: TokenId;
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
      token_id: TokenId;
    }
  };

  public type MintReceipt = Result<MintReceiptPart, ApiError>;

  public type MintReceiptPart = {
    token_id: TokenId;
    id: Nat;
  };
};
