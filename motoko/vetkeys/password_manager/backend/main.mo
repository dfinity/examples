import EncryptedMapsCanister "mo:ic-vetkeys/encrypted_maps/Canister";
import EncryptedMaps "mo:ic-vetkeys/encrypted_maps/EncryptedMaps";
import Types "mo:ic-vetkeys/Types";
import Runtime "mo:core/Runtime";

/// This canister is a thin wrapper around the `ic-vetkeys` Encrypted Maps
/// library. The `EncryptedMapsCanister` mixin contributes the whole endpoint set
/// — the vetKD key, access-control, map-name and value endpoints — so the Candid
/// this canister exposes is exactly what the `@icp-sdk/vetkeys` Encrypted Maps
/// client expects. Those endpoints are snake_case (not the usual Motoko
/// camelCase) because the client calls them by these exact names.
///
/// The mixin holds no stable state of its own: this actor declares the
/// `EncryptedMapsState` and passes it in, so the persistent state stays a plain,
/// visible stable variable this canister owns and can migrate.
actor PasswordManager {
    // The vetKD key name is only an install-time input — it is baked into the
    // stable state below and never read again, hence `transient`. Set the
    // `VETKD_KEY_NAME` canister environment variable (see `icp.yaml`) to pick a
    // different key; it defaults to `test_key_1` so a deploy can never leave the
    // canister half-initialized.
    //
    // The key name is immutable for the life of the canister's data: it feeds
    // vetKD key derivation, so changing it would make every already-encrypted
    // value undecryptable — and this canister only ever sees ciphertext, so it
    // cannot re-encrypt them either. Changing the environment variable on a later
    // upgrade is silently ignored (the stable state is not rebuilt); only a
    // `reinstall`, which drops all data, switches keys.
    transient let keyName = Runtime.envVar<system>("VETKD_KEY_NAME") ?? "test_key_1";

    // The second argument is the domain separator that isolates this
    // application's derived keys from other vetKeys deployments; like the key
    // name it must stay stable for the life of the canister.
    let encryptedMapsState = EncryptedMaps.newEncryptedMapsState<Types.AccessRights>(
        { curve = #bls12_381_g2; name = keyName },
        "password_manager_app",
    );

    include EncryptedMapsCanister(encryptedMapsState);
};
