import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Blob "mo:base/Blob";
import Hex "./utils/Hex";
import Debug "mo:base/Debug";
import Cycles "mo:base/ExperimentalCycles";

actor {
    type VETKD_API = actor {
        vetkd_public_key : ({
            canister_id : ?Principal;
            context : Blob;
            key_id : { curve : { #bls12_381_g2 }; name : Text };
        }) -> async ({ public_key : Blob });
        vetkd_derive_key : ({
            input : Blob;
            context : Blob;
            key_id : { curve : { #bls12_381_g2 }; name : Text };
            transport_public_key : Blob;
        }) -> async ({ encrypted_key : Blob });
    };

    let management_canister : VETKD_API = actor ("aaaaa-aa");

    public shared ({ caller = _ }) func app_vetkd_public_key(context : Blob) : async Text {
        let { public_key } = await management_canister.vetkd_public_key({
            canister_id = null;
            context;
            key_id = { curve = #bls12_381_g2; name = "test_key_1" };
        });
        Hex.encode(Blob.toArray(public_key));
    };

    public shared ({ caller = _ }) func symmetric_key_verification_key() : async Text {
        let { public_key } = await management_canister.vetkd_public_key({
            canister_id = null;
            context = Text.encodeUtf8("symmetric_key");
            key_id = { curve = #bls12_381_g2; name = "test_key_1" };
        });
        Hex.encode(Blob.toArray(public_key));
    };

    public shared ({ caller }) func encrypted_symmetric_key_for_caller(transport_public_key : Blob) : async Text {
        Debug.print("encrypted_symmetric_key_for_caller: caller: " # debug_show (caller));

        let { encrypted_key } = await (with cycles = 26_153_846_153) management_canister.vetkd_derive_key({
            input = Principal.toBlob(caller);
            context = Text.encodeUtf8("symmetric_key");
            key_id = { curve = #bls12_381_g2; name = "test_key_1" };
            transport_public_key;
        });
        Hex.encode(Blob.toArray(encrypted_key));
    };

    public shared ({ caller = _ }) func ibe_encryption_key() : async Text {
        let { public_key } = await management_canister.vetkd_public_key({
            canister_id = null;
            context = Text.encodeUtf8("ibe_encryption");
            key_id = { curve = #bls12_381_g2; name = "test_key_1" };
        });
        Hex.encode(Blob.toArray(public_key));
    };

    public shared ({ caller }) func encrypted_ibe_decryption_key_for_caller(transport_public_key : Blob) : async Text {
        Debug.print("encrypted_ibe_decryption_key_for_caller: caller: " # debug_show (caller));

        let { encrypted_key } = await (with cycles = 26_153_846_153) management_canister.vetkd_derive_key({
            input = Principal.toBlob(caller);
            context = Text.encodeUtf8("ibe_encryption");
            key_id = { curve = #bls12_381_g2; name = "test_key_1" };
            transport_public_key;
        });
        Hex.encode(Blob.toArray(encrypted_key));
    };
};
