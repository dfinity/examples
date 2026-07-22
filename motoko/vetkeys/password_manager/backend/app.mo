import IcVetkeys "mo:ic-vetkeys";
import Types "mo:ic-vetkeys/Types";
import Principal "mo:core/Principal";
import Text "mo:core/Text";
import Blob "mo:core/Blob";
import Result "mo:core/Result";
import Array "mo:core/Array";

// This canister exposes the Encrypted Maps interface. Its public methods are
// intentionally snake_case (not the usual Motoko camelCase) because the
// `@icp-sdk/vetkeys` Encrypted Maps client calls the canister by these exact
// names — renaming them to camelCase would break the frontend. The delegation
// methods below are hand-written today; an upstream Motoko actor mixin that
// generates this endpoint set automatically is in progress
// (https://github.com/dfinity/vetkeys/pull/405).
actor class (keyName : Text) {
    let encryptedMapsState = IcVetkeys.EncryptedMaps.newEncryptedMapsState<Types.AccessRights>({ curve = #bls12_381_g2; name = keyName }, "password_manager_example_app");
    transient let encryptedMaps = IcVetkeys.EncryptedMaps.EncryptedMaps<Types.AccessRights>(encryptedMapsState, Types.accessRightsOperations());

    /// In this canister, we use the `ByteBuf` type to represent blobs. The reason is that we want to be consistent with the Rust canister implementation.
    /// Unfortunately, the `Blob` type cannot be serialized/deserialized in the current Rust implementation efficiently without nesting it in another type.
    public type ByteBuf = { inner : Blob };

    public type EncryptedMapData = {
        map_owner : Principal;
        map_name : ByteBuf;
        keyvals : [(ByteBuf, ByteBuf)];
        access_control : [(Principal, Types.AccessRights)];
    };

    /// The result type compatible with Rust's `Result`.
    public type Result<Ok, Err> = {
        #Ok : Ok;
        #Err : Err;
    };

    public query (msg) func get_accessible_shared_map_names() : async [(Principal, ByteBuf)] {
        Array.map<(Principal, Blob), (Principal, ByteBuf)>(
            encryptedMaps.getAccessibleSharedMapNames(msg.caller),

            func((principal, blob) : (Principal, Blob)) {
                (principal, { inner = blob });
            },
        );
    };

    public query (msg) func get_shared_user_access_for_map(
        map_owner : Principal,
        map_name : ByteBuf,
    ) : async Result<[(Principal, Types.AccessRights)], Text> {
        convertResult(encryptedMaps.getSharedUserAccessForMap(msg.caller, (map_owner, map_name.inner)));
    };

    public query (msg) func get_encrypted_values_for_map(
        map_owner : Principal,
        map_name : ByteBuf,
    ) : async Result<[(ByteBuf, ByteBuf)], Text> {
        let result = encryptedMaps.getEncryptedValuesForMap(msg.caller, (map_owner, map_name.inner));
        switch (result) {
            case (#err(e)) { #Err(e) };
            case (#ok(values)) {
                #Ok(
                    Array.map<(Blob, Blob), (ByteBuf, ByteBuf)>(
                        values,
                        func((blob1, blob2) : (Blob, Blob)) {
                            ({ inner = blob1 }, { inner = blob2 });
                        },
                    )
                );
            };
        };
    };

    public query (msg) func get_all_accessible_encrypted_values() : async [((Principal, ByteBuf), [(ByteBuf, ByteBuf)])] {
        Array.map<((Principal, Blob), [(Blob, Blob)]), ((Principal, ByteBuf), [(ByteBuf, ByteBuf)])>(
            encryptedMaps.getAllAccessibleEncryptedValues(msg.caller),
            func(((owner, map_name), values) : ((Principal, Blob), [(Blob, Blob)])) {
                (
                    (owner, { inner = map_name }),
                    Array.map<(Blob, Blob), (ByteBuf, ByteBuf)>(
                        values,
                        func((blob1, blob2) : (Blob, Blob)) {
                            ({ inner = blob1 }, { inner = blob2 });
                        },
                    ),
                );
            },
        );
    };

    public query (msg) func get_all_accessible_encrypted_maps() : async [EncryptedMapData] {
        Array.map<IcVetkeys.EncryptedMaps.EncryptedMapData<Types.AccessRights>, EncryptedMapData>(
            encryptedMaps.getAllAccessibleEncryptedMaps(msg.caller),
            func(map : IcVetkeys.EncryptedMaps.EncryptedMapData<Types.AccessRights>) : EncryptedMapData {
                {
                    map_owner = map.map_owner;
                    map_name = { inner = map.map_name };
                    keyvals = Array.map<(Blob, Blob), (ByteBuf, ByteBuf)>(
                        map.keyvals,
                        func((blob1, blob2) : (Blob, Blob)) {
                            ({ inner = blob1 }, { inner = blob2 });
                        },
                    );
                    access_control = map.access_control;
                };
            },
        );
    };

    public query (msg) func get_encrypted_value(
        map_owner : Principal,
        map_name : ByteBuf,
        map_key : ByteBuf,
    ) : async Result<?ByteBuf, Text> {
        let result = encryptedMaps.getEncryptedValue(msg.caller, (map_owner, map_name.inner), map_key.inner);
        switch (result) {
            case (#err(e)) { #Err(e) };
            case (#ok(null)) { #Ok(null) };
            case (#ok(?blob)) { #Ok(?{ inner = blob }) };
        };
    };

    public shared (msg) func remove_map_values(
        map_owner : Principal,
        map_name : ByteBuf,
    ) : async Result<[ByteBuf], Text> {
        let result = encryptedMaps.removeMapValues(msg.caller, (map_owner, map_name.inner));
        switch (result) {
            case (#err(e)) { #Err(e) };
            case (#ok(values)) {
                #Ok(
                    Array.map<Blob, ByteBuf>(
                        values,
                        func(blob : Blob) : ByteBuf {
                            { inner = blob };
                        },
                    )
                );
            };
        };
    };

    public query (msg) func get_owned_non_empty_map_names() : async [ByteBuf] {
        Array.map<Blob, ByteBuf>(
            encryptedMaps.getOwnedNonEmptyMapNames(msg.caller),
            func(blob : Blob) : ByteBuf {
                { inner = blob };
            },
        );
    };

    public shared (msg) func insert_encrypted_value(
        map_owner : Principal,
        map_name : ByteBuf,
        map_key : ByteBuf,
        value : ByteBuf,
    ) : async Result<?ByteBuf, Text> {
        let result = encryptedMaps.insertEncryptedValue(msg.caller, (map_owner, map_name.inner), map_key.inner, value.inner);
        switch (result) {
            case (#err(e)) { #Err(e) };
            case (#ok(null)) { #Ok(null) };
            case (#ok(?blob)) { #Ok(?{ inner = blob }) };
        };
    };

    public shared (msg) func remove_encrypted_value(
        map_owner : Principal,
        map_name : ByteBuf,
        map_key : ByteBuf,
    ) : async Result<?ByteBuf, Text> {
        let result = encryptedMaps.removeEncryptedValue(msg.caller, (map_owner, map_name.inner), map_key.inner);
        switch (result) {
            case (#err(e)) { #Err(e) };
            case (#ok(null)) { #Ok(null) };
            case (#ok(?blob)) { #Ok(?{ inner = blob }) };
        };
    };

    public shared func get_vetkey_verification_key() : async ByteBuf {
        let inner = await encryptedMaps.getVetkeyVerificationKey();
        { inner };
    };

    public shared (msg) func get_encrypted_vetkey(
        map_owner : Principal,
        map_name : ByteBuf,
        transport_key : ByteBuf,
    ) : async Result<ByteBuf, Text> {
        let result = await encryptedMaps.getEncryptedVetkey(msg.caller, (map_owner, map_name.inner), transport_key.inner);
        switch (result) {
            case (#err(e)) { #Err(e) };
            case (#ok(vetkey)) { #Ok({ inner = vetkey }) };
        };
    };

    public query (msg) func get_user_rights(
        map_owner : Principal,
        map_name : ByteBuf,
        user : Principal,
    ) : async Result<?Types.AccessRights, Text> {
        convertResult(encryptedMaps.getUserRights(msg.caller, (map_owner, map_name.inner), user));
    };

    public shared (msg) func set_user_rights(
        map_owner : Principal,
        map_name : ByteBuf,
        user : Principal,
        access_rights : Types.AccessRights,
    ) : async Result<?Types.AccessRights, Text> {
        convertResult(encryptedMaps.setUserRights(msg.caller, (map_owner, map_name.inner), user, access_rights));
    };

    public shared (msg) func remove_user(
        map_owner : Principal,
        map_name : ByteBuf,
        user : Principal,
    ) : async Result<?Types.AccessRights, Text> {
        convertResult(encryptedMaps.removeUser(msg.caller, (map_owner, map_name.inner), user));
    };

    /// Convert to the result type compatible with Rust's `Result`
    private func convertResult<Ok, Err>(result : Result.Result<Ok, Err>) : Result<Ok, Err> {
        switch (result) {
            case (#err(e)) { #Err(e) };
            case (#ok(o)) { #Ok(o) };
        };
    };
};
