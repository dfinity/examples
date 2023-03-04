import Principal "mo:base/Principal";
import Time "mo:base/Time";
import Nat "mo:base/Nat";
import Nat64 "mo:base/Nat64";
import List "mo:base/List";

module {
  // User Profile
  public type Profile = {
    username : ?Text;
    avatar : ?Text;
    phone : ?Text;
    inviter : ?Principal;
  };

  // referral
  public type Referral = {
    uid : Principal;
    member : Principal;
  };

  // Transaction
  public type Operation = {
    #awardReferral;
  };
  public type TxRecord = {
    uuid : Text;
    caller : Principal;
    refType : Operation; // operation type
    refId : ?Text;
    blockIndex : Nat64; // transaction index
    fromPrincipal : Principal;
    toPrincipal : Principal;
    amount : Nat64;
    fee : Nat64;
    timestamp : Time.Time;
  };

  // Error codes
  public type Error = {
    #BalanceLow;
    #TransferFailure;
    #NotFound;
    #AlreadyExisting;
    #NotAuthorized;
    #SomethingWrong;
    #InvalidData;
  };
};
