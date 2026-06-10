import ManagementCanister "mo:ic-vetkeys/ManagementCanister";
import Principal "mo:core/Principal";
import Text "mo:core/Text";

actor {

    let TEST_KEY : ManagementCanister.VetKdKeyid = {
        curve = #bls12_381_g2;
        name = "test_key_1";
    };

    public shared func symmetric_key_verification_key() : async Blob {
        await ManagementCanister.vetKdPublicKey(null, Text.encodeUtf8("symmetric_key"), TEST_KEY);
    };

    public shared ({ caller }) func encrypted_symmetric_key_for_caller(transport_public_key : Blob) : async Blob {
        await ManagementCanister.vetKdDeriveKey(Principal.toBlob(caller), Text.encodeUtf8("symmetric_key"), TEST_KEY, transport_public_key);
    };

    public shared func ibe_encryption_key() : async Blob {
        await ManagementCanister.vetKdPublicKey(null, Text.encodeUtf8("ibe_encryption"), TEST_KEY);
    };

    public shared ({ caller }) func encrypted_ibe_decryption_key_for_caller(transport_public_key : Blob) : async Blob {
        await ManagementCanister.vetKdDeriveKey(Principal.toBlob(caller), Text.encodeUtf8("ibe_encryption"), TEST_KEY, transport_public_key);
    };
};
