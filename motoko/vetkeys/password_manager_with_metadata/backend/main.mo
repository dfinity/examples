import Principal "mo:core/Principal";
import Blob "mo:core/Blob";
import List "mo:core/List";
import OrderedMap "mo:core/pure/Map";
import Time "mo:core/Time";
import Nat "mo:core/Nat";
import Int "mo:core/Int";
import Runtime "mo:core/Runtime";
import EncryptedMapsControlPlaneCanister "mo:ic-vetkeys/encrypted_maps/ControlPlaneCanister";
import EncryptedMaps "mo:ic-vetkeys/encrypted_maps/EncryptedMaps";
import Types "mo:ic-vetkeys/Types";

/// This canister layers a second, backend-authoritative store on top of the
/// `ic-vetkeys` Encrypted Maps library: every encrypted password has a matching
/// metadata row (creation date, modification counter, tags, url) that the
/// canister — not the client — maintains.
///
/// It includes the `EncryptedMapsControlPlaneCanister` mixin rather than the full
/// `EncryptedMapsCanister`: the control-plane mixin contributes the vetKD key,
/// access-control and map-name endpoints plus the in-scope `encryptedMaps`
/// object, but none of the endpoints that read or write encrypted values. That is
/// what this canister needs — the plain `insert_encrypted_value` /
/// `remove_encrypted_value` mutators are never exposed, so nothing can write a
/// value without its metadata row, and the `*_with_metadata` endpoints below
/// update both stores in a single call.
///
/// The public methods are snake_case (not the usual Motoko camelCase): the
/// standard Encrypted Maps methods are called by these exact names by the
/// `@icp-sdk/vetkeys` client, and the custom metadata methods follow the same
/// convention for a consistent interface.
actor PasswordManagerWithMetadata {
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
  // application's derived keys from other vetKeys deployments; like the key name
  // it must stay stable for the life of the canister. This actor — not the mixin
  // — owns the state, so it stays a plain, visible stable variable.
  let encryptedMapsState = EncryptedMaps.newEncryptedMapsState<Types.AccessRights>(
    { curve = #bls12_381_g2; name = keyName },
    "password_manager_with_metadata_app",
  );

  // Brings the control-plane endpoints and the `encryptedMaps`, `ByteBuf` and
  // `Result` names into scope, over the state declared above.
  include EncryptedMapsControlPlaneCanister(encryptedMapsState);

  public type PasswordMetadata = {
    creation_date : Nat64;
    last_modification_date : Nat64;
    number_of_modifications : Nat64;
    last_modified_principal : Principal;
    tags : [Text];
    url : Text;
  };

  type MapOwner = Principal;
  type MapName = Blob;
  type MapKey = Blob;
  type MetadataKey = (MapOwner, MapName, MapKey);

  func compareMetadataKeys(a : MetadataKey, b : MetadataKey) : {
    #less;
    #greater;
    #equal;
  } {
    let ownerCompare = Principal.compare(a.0, b.0);
    if (ownerCompare == #equal) {
      let nameCompare = Blob.compare(a.1, b.1);
      if (nameCompare == #equal) {
        Blob.compare(a.2, b.2);
      } else {
        nameCompare;
      };
    } else {
      ownerCompare;
    };
  };

  // The metadata store kept in lockstep with the encrypted values above.
  var metadata : OrderedMap.Map<MetadataKey, PasswordMetadata> = OrderedMap.empty<MetadataKey, PasswordMetadata>();

  func newPasswordMetadata(caller : Principal, tags : [Text], url : Text) : PasswordMetadata {
    let timeNow = Int.abs(Time.now()).toNat64();
    {
      creation_date = timeNow;
      last_modification_date = timeNow;
      number_of_modifications = 0;
      last_modified_principal = caller;
      tags = tags;
      url = url;
    };
  };

  func updatePasswordMetadata(metadata : PasswordMetadata, caller : Principal, tags : [Text], url : Text) : PasswordMetadata {
    let timeNow = Int.abs(Time.now()).toNat64();
    {
      creation_date = metadata.creation_date;
      last_modification_date = timeNow;
      number_of_modifications = metadata.number_of_modifications + 1;
      last_modified_principal = caller;
      tags = tags;
      url = url;
    };
  };

  public query (msg) func get_encrypted_values_for_map_with_metadata(
    map_owner : Principal,
    map_name : ByteBuf,
  ) : async Result<[(ByteBuf, ByteBuf, PasswordMetadata)], Text> {
    switch (encryptedMaps.getEncryptedValuesForMap(msg.caller, (map_owner, map_name.inner))) {
      case (#err(msg)) { #Err(msg) };
      case (#ok(mapValues)) {
        let results = List.empty<(ByteBuf, ByteBuf, PasswordMetadata)>();

        for ((key, encryptedValue) in mapValues.values()) {
          let metadataKey = (map_owner, map_name.inner, key);
          switch (metadata.get(compareMetadataKeys, metadataKey)) {
            case (null) {
              Runtime.trap("bug: inconsistent state: no metadata for key");
            };
            case (?metadataValue) {
              results.add(({ inner = key }, { inner = encryptedValue }, metadataValue));
            };
          };
        };

        #Ok(results.toArray());
      };
    };
  };

  public shared (msg) func insert_encrypted_value_with_metadata(
    map_owner : Principal,
    map_name : ByteBuf,
    map_key : ByteBuf,
    value : ByteBuf,
    tags : [Text],
    url : Text,
  ) : async Result<?(ByteBuf, PasswordMetadata), Text> {
    // The library call performs the access-control check, so it comes first: if
    // the caller may not write, the metadata store is left untouched.
    switch (encryptedMaps.insertEncryptedValue(msg.caller, (map_owner, map_name.inner), map_key.inner, value.inner)) {
      case (#err(msg)) { #Err(msg) };
      case (#ok(optPrevValue)) {
        let metadataKey = (map_owner, map_name.inner, map_key.inner);
        let prevMetadata = metadata.get(compareMetadataKeys, metadataKey);

        let metadataValue = switch (prevMetadata) {
          case (null) {
            newPasswordMetadata(msg.caller, tags, url);
          };
          case (?existingMetadata) {
            updatePasswordMetadata(existingMetadata, msg.caller, tags, url);
          };
        };

        metadata := metadata.add(compareMetadataKeys, metadataKey, metadataValue);

        switch (optPrevValue, prevMetadata) {
          case (null, null) { #Ok(null) };
          case (null, ?_) {
            Runtime.trap("bug: inconsistent state: no previous value but some metadata");
          };
          case (?_, null) {
            Runtime.trap("bug: inconsistent state: some previous value but no metadata");
          };
          case (?prevValue, ?m) { #Ok(?({ inner = prevValue }, m)) };
        };
      };
    };
  };

  public shared (msg) func remove_encrypted_value_with_metadata(
    map_owner : Principal,
    map_name : ByteBuf,
    map_key : ByteBuf,
  ) : async Result<?(ByteBuf, PasswordMetadata), Text> {
    switch (encryptedMaps.removeEncryptedValue(msg.caller, (map_owner, map_name.inner), map_key.inner)) {
      case (#err(msg)) { #Err(msg) };
      case (#ok(optPrevValue)) {
        let metadataKey = (map_owner, map_name.inner, map_key.inner);
        let prevMetadata = metadata.get(compareMetadataKeys, metadataKey);

        metadata := metadata.remove(compareMetadataKeys, metadataKey);

        switch (optPrevValue, prevMetadata) {
          case (null, null) { #Ok(null) };
          case (null, ?_) {
            Runtime.trap("bug: inconsistent state: no previous value but some metadata");
          };
          case (?_, null) {
            Runtime.trap("bug: inconsistent state: some previous value but no metadata");
          };
          case (?prevValue, ?m) { #Ok(?({ inner = prevValue }, m)) };
        };
      };
    };
  };
};
