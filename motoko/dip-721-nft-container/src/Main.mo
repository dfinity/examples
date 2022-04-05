import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat8";
import Nat16 "mo:base/Nat16";
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import List "mo:base/List";
import Array "mo:base/Array";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Types "./Types";

shared actor class Dip721NFT() = Self {
    stable var nfts = List.fromArray<Types.TokenMetadata>([]);

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
            case null {
                #err(#InvalidTokenId);
            };
            case (?token) {
                #ok(token.principal);
            };
        };
    };
}