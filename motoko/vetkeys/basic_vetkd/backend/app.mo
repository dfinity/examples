import Blob "mo:core/Blob";
import Principal "mo:core/Principal";
import Text "mo:core/Text";

actor {

    type VetKdKeyId = {
        curve : { #bls12_381_g2 };
        name : Text;
    };

    type VetKdSystemApi = actor {
        vetkd_public_key : ({
            canister_id : ?Principal;
            context : Blob;
            key_id : VetKdKeyId;
        }) -> async { public_key : Blob };
        vetkd_derive_key : ({
            input : Blob;
            context : Blob;
            key_id : VetKdKeyId;
            transport_public_key : Blob;
        }) -> async { encrypted_key : Blob };
    };

    let management_canister : VetKdSystemApi = actor ("aaaaa-aa");

    let TEST_KEY : VetKdKeyId = {
        curve = #bls12_381_g2;
        name = "test_key_1";
    };

    public shared func symmetric_key_verification_key() : async Blob {
        let { public_key } = await management_canister.vetkd_public_key({
            canister_id = null;
            context = Text.encodeUtf8("symmetric_key");
            key_id = TEST_KEY;
        });
        public_key
    };

    public shared ({ caller }) func encrypted_symmetric_key_for_caller(transport_public_key : Blob) : async Blob {
        let { encrypted_key } = await (with cycles = 26_153_846_153) management_canister.vetkd_derive_key({
            input = Principal.toBlob(caller);
            context = Text.encodeUtf8("symmetric_key");
            key_id = TEST_KEY;
            transport_public_key;
        });
        encrypted_key
    };

    public shared func ibe_encryption_key() : async Blob {
        let { public_key } = await management_canister.vetkd_public_key({
            canister_id = null;
            context = Text.encodeUtf8("ibe_encryption");
            key_id = TEST_KEY;
        });
        public_key
    };

    public shared ({ caller }) func encrypted_ibe_decryption_key_for_caller(transport_public_key : Blob) : async Blob {
        let { encrypted_key } = await (with cycles = 26_153_846_153) management_canister.vetkd_derive_key({
            input = Principal.toBlob(caller);
            context = Text.encodeUtf8("ibe_encryption");
            key_id = TEST_KEY;
            transport_public_key;
        });
        encrypted_key
    };
};
