import Principal "mo:core/Principal";
import Text "mo:core/Text";
import Blob "mo:core/Blob";
import Nat64 "mo:core/Nat64";
import Time "mo:core/Time";
import Map "mo:core/Map";
import Array "mo:core/Array";
import List "mo:core/List";
import Nat "mo:core/Nat";
import VetKeys "mo:ic-vetkeys";
import Order "mo:core/Order";
import Runtime "mo:core/Runtime";

actor {
    // The vetKD key name this canister derives from. Set the `VETKD_KEY_NAME`
    // canister environment variable (see `icp.yaml`) to pick a different key; it
    // defaults to `test_key_1` so a deploy can never leave the canister
    // half-initialized. It is re-read on every upgrade, so changing the variable
    // does take effect — and orphans everything encrypted under the old key, which
    // no other key can decrypt. Treat it as fixed once the canister holds data.
    transient let keyName = Runtime.envVar<system>("VETKD_KEY_NAME") ?? "test_key_1";

    // Types
    type Signature = {
        message : Text;
        signature : Blob;
        timestamp : Nat64;
    };

    type SignatureKey = {
        signer : Principal;
        timestamp : Nat64;
    };

    type VetKdKeyid = {
        curve : { #bls12_381_g2 };
        name : Text;
    };

    // Compare function for SignatureKey
    private func signatureKeyCompare(a : SignatureKey, b : SignatureKey) : Order.Order {
        switch (Principal.compare(a.signer, b.signer)) {
            case (#equal) { Nat64.compare(a.timestamp, b.timestamp) };
            case (other) { other };
        }
    };

    // Signatures are retained across upgrades: this actor field is not declared `transient`.
    private let signatures = Map.empty<SignatureKey, Signature>();

    // Helper function to get current timestamp
    private func getTimestamp() : Nat64 {
        Nat64.fromIntWrap(Time.now());
    };

    // Helper function to create context for vetKD
    private func context(signer : Principal) : Blob {
        // Domain separator for this app
        let domainSeparator : [Nat8] = Text.encodeUtf8("basic_bls_signing_app").toArray();
        let domainSeparatorLength : [Nat8] = [domainSeparator.size().toNat8()]; // Length of domain separator

        // Combine domain separator length, domain separator, and signer principal
        let signerBytes = signer.toBlob();
        let signerArray = signerBytes.toArray();

        let contextArray = domainSeparatorLength.concat(domainSeparator).concat(
            signerArray,
        );

        contextArray.toBlob();
    };

    // Helper function to get key ID
    private func keyId() : VetKdKeyid {
        {
            curve = #bls12_381_g2;
            name = keyName;
        };
    };

    // Sign a message using BLS
    public shared ({ caller }) func signMessage(message : Text) : async Blob {
        let signatureBytes = await VetKeys.ManagementCanister.signWithBls(
            message.encodeUtf8(),
            context(caller),
            keyId(),
        );

        let timestamp = getTimestamp();
        let signature : Signature = {
            message = message;
            signature = signatureBytes;
            timestamp = timestamp;
        };

        // Handle potential timestamp collisions by incrementing until we find a free slot
        var timestampForMapKey = timestamp;
        while (signatures.get(signatureKeyCompare, { signer = caller; timestamp = timestampForMapKey }) != null) {
            timestampForMapKey += 1;
        };

        ignore signatures.insert(signatureKeyCompare, { signer = caller; timestamp = timestampForMapKey }, signature);

        signatureBytes;
    };

    // Get all signatures for the current caller
    public shared query ({ caller }) func getMySignatures() : async [Signature] {
        let callerSignatures = List.empty<Signature>();

        for ((key, value) in signatures.entries()) {
            if (Principal.equal(key.signer, caller)) {
                callerSignatures.add(value);
            };
        };

        callerSignatures.toArray();
    };

    // Get verification key for the current caller
    public shared ({ caller }) func getMyVerificationKey() : async Blob {
        await VetKeys.ManagementCanister.blsPublicKey(
            null,
            context(caller),
            keyId(),
        );
    };
};
