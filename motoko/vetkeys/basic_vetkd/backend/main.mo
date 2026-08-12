import ManagementCanister "mo:ic-vetkeys/ManagementCanister";
import Principal "mo:core/Principal";
import Text "mo:core/Text";
import Runtime "mo:core/Runtime";

actor {

    // The vetKD key name this canister derives from. Set the `VETKD_KEY_NAME`
    // canister environment variable (see `icp.yaml`) to pick a different key; it
    // defaults to `test_key_1` so a deploy can never leave the canister
    // half-initialized. It is re-read on every upgrade, so changing the variable
    // does take effect — and orphans everything encrypted under the old key, which
    // no other key can decrypt. Treat it as fixed once the canister holds data.
    transient let keyName = Runtime.envVar<system>("VETKD_KEY_NAME") ?? "test_key_1";

    let TEST_KEY : ManagementCanister.VetKdKeyid = {
        curve = #bls12_381_g2;
        name = keyName;
    };

    public shared func symmetric_key_verification_key() : async Blob {
        await ManagementCanister.vetKdPublicKey(null, Text.encodeUtf8("symmetric_key"), TEST_KEY);
    };

    public shared ({ caller }) func encrypted_symmetric_key_for_caller(transport_public_key : Blob) : async Blob {
        await ManagementCanister.vetKdDeriveKey(caller.toBlob(), Text.encodeUtf8("symmetric_key"), TEST_KEY, transport_public_key);
    };

    public shared func ibe_encryption_key() : async Blob {
        await ManagementCanister.vetKdPublicKey(null, Text.encodeUtf8("ibe_encryption"), TEST_KEY);
    };

    public shared ({ caller }) func encrypted_ibe_decryption_key_for_caller(transport_public_key : Blob) : async Blob {
        await ManagementCanister.vetKdDeriveKey(caller.toBlob(), Text.encodeUtf8("ibe_encryption"), TEST_KEY, transport_public_key);
    };
};
