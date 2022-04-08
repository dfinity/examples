import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat8";
import Nat16 "mo:base/Nat16";
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import List "mo:base/List";
import Array "mo:base/Array";
import Option "mo:base/Option";
import Bool "mo:base/Bool";
import Principal "mo:base/Principal";
import Types "./Types";

shared actor class Dip721NFT() = Self {
  stable var nfts = List.nil<Types.TokenMetadata>();
  stable var custodians = List.nil<Principal>();

  // https://forum.dfinity.org/t/is-there-any-address-0-equivalent-at-dfinity-motoko/5445/3
  let null_address : Principal = Principal.fromText("aaaaa-aa");

  public query func balanceOfDip721(user: Principal) : async Nat64 {
    return Nat64.fromNat(
      List.size(
        List.filter(nfts, func(token: Types.TokenMetadata) : Bool { token.principal == user })
      )
    );
  };

  public query func ownerOfDip721(token_id: Nat64) : async Types.OwnerResult {
    let item = List.get(nfts, Nat64.toNat(token_id));
    switch (item) {
      case (null) {
        #err(#InvalidTokenId);
      };
      case (?token) {
          #ok(token.principal);
      };
    };
  };

  public shared({ caller }) func safeTransferFromDip721(from: Principal, to: Principal, token_id: Nat64) : async Types.TxReceipt {  
    if (to == null_address) {
      return #err(#ZeroAddress);
    } else {
      return transferFrom(from, to, token_id, caller);
    };
  };

  public shared({ caller }) func transferFromDip721(from: Principal, to: Principal, token_id: Nat64) : async Types.TxReceipt {
    return transferFrom(from, to, token_id, caller);
  };

  func transferFrom(from: Principal, to: Principal, token_id: Nat64, caller: Principal) : Types.TxReceipt {
    let item = List.get(nfts, Nat64.toNat(token_id));
    switch (item) {
      case null {
        return #err(#InvalidTokenId);
      };
      case (?token) {
        if (
          caller != token.principal and
          not List.some(custodians, func (custodian : Principal) : Bool { custodian == caller })
        ) {
          return #err(#Unauthorized);
        } else if (Principal.notEqual(from, token.principal)) {
          return #err(#Other);
        } else {
          nfts := List.map(nfts, func (item : Types.TokenMetadata) : Types.TokenMetadata { 
            if (item.token_identifier == token.token_identifier) {
              let update : Types.TokenMetadata = {
                account_identifier = token.account_identifier;
                metadata = token.metadata;
                token_identifier = token.token_identifier;
                principal = to;
              };
              return update;
            } else {
              return item;
            };
           });
          return #ok(Nat64.toNat(token_id));   
        };
      };
    };
  }
}