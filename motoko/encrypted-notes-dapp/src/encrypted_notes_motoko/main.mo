import Map "mo:base/HashMap";
import Text "mo:base/Text";
import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import List "mo:base/List";
import Iter "mo:base/Iter";
import Time "mo:base/Time";
import Int "mo:base/Int";
import Nat "mo:base/Nat";
import Bool "mo:base/Bool";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Option "mo:base/Option";
import Debug "mo:base/Debug";
import Order "mo:base/Order";

import En "types";
import UserStore "user_store";


// Declare a shared actor class
// Bind the caller and the initializer
shared({ caller = initializer }) actor class() {

    // Currently, a single canister smart contract is limited to 4 GB of storage due to WebAssembly limitations.
    // To ensure that our canister does not exceed this limit, we restrict memory usage to at most 2 GB because 
    // up to 2x memory may be needed for data serialization during canister upgrades. Therefore, we aim to support
    // up to 1,000 users, each storing up to 2 MB of data. 
    // 1) One half of this data is reserved for device management: 
    //     DEVICES_PER_USER = (MAX_CYPHERTEXT_LENGTH + MAX_PUBLIC_KEY_LENGTH + MAX_DEVICE_ALIAS_LENGTH) x (4 bytes per char) x MAX_DEVICES_PER_USER
    //     1 MB = 40,700 x 4 x 6 = 976,800
    // 2) Another half is reserved for storing the notes:
    //     NOTES_PER_USER = MAX_NOTES_PER_USER x MAX_NOTE_CHARS x (4 bytes per char)
    //     1 MB = 500 x 500 x 4 = 1,000,000

    // Define dapp limits - important for security assurance
    private let MAX_USERS = 1_000;
    private let MAX_NOTES_PER_USER = 500;
    private let MAX_DEVICES_PER_USER = 6;
    private let MAX_NOTE_CHARS = 500;
    private let MAX_DEVICE_ALIAS_LENGTH = 200;
    private let MAX_PUBLIC_KEY_LENGTH = 500;
    private let MAX_CYPHERTEXT_LENGTH = 40_000;

    // Define private types
    private type PrincipalName = Text;

    // Define public types
    // Type of an encrypted note
    // Attention: This canister does *not* perform any encryption. 
    //            Here we assume that the notes are encrypted end-
    //            to-end by the front-end (at client side). 
    public type EncryptedNote = {
        encrypted_text: Text;
        id: Nat;
    };

    // Define private fields
    // Stable actor fields are automatically retained across canister upgrades. 
    // See https://internetcomputer.org/docs/current/motoko/main/upgrades/

    // Design choice: Use globally unique note identifiers for all users.
    //
    // The keyword `stable` makes this (scalar) variable keep its value across canister upgrades.
    //
    // See https://internetcomputer.org/docs/current/developer-docs/setup/manage-canisters#upgrade-a-canister
    private stable var nextNoteId: Nat = 1;
    
    // Internal representation: store each user's notes in a separate List. 
    private var notesByUser = Map.HashMap<PrincipalName, List.List<EncryptedNote>>(0, Text.equal, Text.hash);
    
    // While accessing data via [notesByUser] is more efficient, we use the following stable array
    // as a buffer to preserve user notes across canister upgrades.
    // See also: [preupgrade], [postupgrade]
    private stable var stable_notesByUser: [(PrincipalName, List.List<EncryptedNote>)] = [];

    // Internal representation: associate each user with a UserStore
    private var users = Map.HashMap<Principal, UserStore.UserStore>(10, Principal.equal, Principal.hash);

    // While accessing data via hashed structures (e.g., [users]) may be more efficient, we use 
    // the following stable array as a buffer to preserve registered users and user devices across 
    // canister upgrades. 
    // See also: [pre_upgrade], [post_upgrade]
    // TODO: replace with
    // private stable var stable_users: [UserStore.StableUserStoreEntry] = [];
    // once https://github.com/dfinity/motoko/issues/3128 is resolved.
    private stable var stable_users: [(Principal, En.PublicKey, En.DeviceAlias, ?En.Ciphertext)] = [];

    // The following invariant is preserved by [register_device].
    //
    // All the functions of this canister's public API are available only to 
    // registered users, with the exception of [register_device] and [whoami].
    //
    // See also: [is_user_registered]
    private func users_invariant(): Bool {
        notesByUser.size() == users.size()
    };

    // Check if this user has been registered 
    // Note: [register_device] must be each user's very first update call.
    // See also: [users_invariant]
    private func is_user_registered(principal: Principal): Bool {
        Option.isSome(users.get(principal));
    };

    // Returns the current number of users.
    // Traps if [users_invariant] is violated
    private func user_count(): Nat {
        assert users_invariant();
        notesByUser.size()
    };

    // Check that a note identifier is sane. This is needed since Motoko integers 
    // are infinite-precision. 
    // Note: avoid extraneous usage of async functions, hence [user_count]
    private func is_id_sane(id: Int): Bool {
        0 <= id and id < MAX_NOTES_PER_USER * user_count()
    };

    // Returns `true` iff [store.device_list] contains the provided public key [pk].
    //
    // Traps:
    //      [store.device_list] exceeds [MAX_DEVICES_PER_USER]
    //      [pk] exceeds [MAX_PUBLIC_KEY_LENGTH]
    private func is_known_public_key(store: UserStore.UserStore, pk: En.PublicKey): Bool {
        assert store.device_list.size() <= MAX_DEVICES_PER_USER;
        assert pk.size() <= MAX_PUBLIC_KEY_LENGTH;

        var found = false;
        for (x in store.device_list.entries()) {
            if (x.1 == pk) {
                return true;
            }
        };
        false
    };

    // Utility function that helps writing assertion-driven code more concisely.
    private func expect<T>(opt: ?T, violation_msg: Text): T {
        switch (opt) {
            case (null) {
                Debug.trap(violation_msg);
            };
            case (?x) {
                x
            };
        };
    };

    // Reflects the [caller]'s identity by returning (a future of) its principal. 
    // Useful for debugging. 
    public shared({ caller }) func whoami(): async Text {
        return Principal.toText(caller);
    };

    // Shared functions, i.e., those specified with [shared], are 
    // accessible to remote callers. 
    // The extra parameter [caller] is the caller's principal
    // See https://internetcomputer.org/docs/current/motoko/main/actors-async

    // Add new note for this [caller]. Note: this function may be called only by 
    // those users that have at least one device registered via [register_device].
    //      [encrypted_text]: (encrypted) content of this note
    //
    // Returns: 
    //      Future of unit
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    //      [encrypted_text] exceeds [MAX_NOTE_CHARS]
    //      User already has [MAX_NOTES_PER_USER] notes
    public shared({ caller }) func add_note(encrypted_text: Text): async () {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);
        assert encrypted_text.size() <= MAX_NOTE_CHARS;

        Debug.print("Adding note...");

        let principalName = Principal.toText(caller);
        let userNotes : List.List<EncryptedNote> = Option.get(notesByUser.get(principalName), List.nil<EncryptedNote>());

        // check that user is not going to exceed limits
        assert List.size(userNotes) < MAX_NOTES_PER_USER;
        
        let newNote: EncryptedNote = {
            id = nextNoteId; 
            encrypted_text = encrypted_text
        };
        nextNoteId += 1;
        notesByUser.put(principalName, List.push(newNote, userNotes));
    };

    // Returns (a future of) this [caller]'s notes.
    // 
    // --- Queries vs. Updates ---
    // Note that this method is declared as an *update* call (see `shared`) rather than *query*.
    //
    // While queries are significantly faster than updates, they are not certified by the IC. 
    // Thus, we avoid using queries throughout this dapp, ensuring that the result of our 
    // functions gets through consensus. Otherwise, this function could e.g. omit some notes 
    // if it got executed by a malicious node. (To make the dapp more efficient, one could 
    // use an approach in which both queries and updates are combined.)
    // See https://internetcomputer.org/docs/current/concepts/canisters-code#query-and-update-methods
    //
    // Returns: 
    //      Future of array of EncryptedNote
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    public shared({ caller }) func get_notes(): async [EncryptedNote] {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);

        let principalName = Principal.toText(caller);
        let userNotes = Option.get(notesByUser.get(principalName), List.nil());
        return List.toArray(userNotes);
    };

    // Update this [caller]'s note (by replacing an existing with 
    // the same id). If none of the existing notes have this id, 
    // do nothing. 
    // [encrypted_note]: the note to be updated
    //
    // Returns: 
    //      Future of unit
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    //      [encrypted_note.encrypted_text] exceeds [MAX_NOTE_CHARS]
    //      [encrypted_note.id] is unreasonable; see [is_id_sane]
    public shared({ caller }) func update_note(encrypted_note: EncryptedNote): async () {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);
        assert encrypted_note.encrypted_text.size() <= MAX_NOTE_CHARS;
        assert is_id_sane(encrypted_note.id);

        let principalName = Principal.toText(caller);
        var existingNotes = expect(notesByUser.get(principalName), 
            "registered user (principal " # principalName # ") w/o allocated notes");

        var updatedNotes = List.map(existingNotes, func (note: EncryptedNote): EncryptedNote {
            if (note.id == encrypted_note.id) {
                encrypted_note
            } else {
                note
            }
        });
        notesByUser.put(principalName, updatedNotes);
    };

    // Delete this [caller]'s note with given id. If none of the 
    // existing notes have this id, do nothing. 
    // [id]: the id of the note to be deleted
    //
    // Returns: 
    //      Future of unit
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    //      [id] is unreasonable; see [is_id_sane]
    public shared({ caller }) func delete_note(id: Int): async () {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);
        assert is_id_sane(id);

        let principalName = Principal.toText(caller);
        var notesOfUser = Option.get(notesByUser.get(principalName), List.nil());

        notesByUser.put(
            principalName,
            List.filter(notesOfUser, func(note: EncryptedNote): Bool { note.id != id })
        )
    };

    // Below, we implement a decentralized key-value store. 
    // The purpose of this code is to support and synchronize 
    // multiple devices that a single user may have. 

    // Associate a public key with a device ID.
    // Returns: 
    //      `true` iff device is *newly* registered, ie. [alias] has not been 
    //      registered for this user before. 
    // Traps:
    //      [caller] is the anonymous identity
    //      [alias] exceeds [MAX_DEVICE_ALIAS_LENGTH]
    //      [pk] exceeds [MAX_PUBLIC_KEY_LENGTH]
    //      While registering new user's device:
    //          There are already [MAX_USERS] users while we need to register a new user
    //          This user already has notes despite not having any registered devices
    //      This user already has [MAX_DEVICES_PER_USER] registered devices.
    public shared({ caller }) func register_device(
        alias: En.DeviceAlias, pk: En.PublicKey
    ): async Bool {
        
        assert not Principal.isAnonymous(caller);
        assert alias.size() <= MAX_DEVICE_ALIAS_LENGTH;
        assert pk.size() <= MAX_PUBLIC_KEY_LENGTH;

        // get caller's device list and add
        switch (users.get(caller)) {
            case null {
                // caller unknown ==> check invariants
                // A. can we add a new user?
                assert user_count() < MAX_USERS;
                // B. this caller does not have notes
                let principalName = Principal.toText(caller);
                assert notesByUser.get(principalName) == null;

                // ... then initialize the following:
                // 1) a new [UserStore] instance in [users]
                let new_store = UserStore.UserStore(caller, 10);
                new_store.device_list.put(alias, pk);
                users.put(caller, new_store);
                // 2) a new [[EncryptedNote]] list in [notesByUser]
                notesByUser.put(principalName, List.nil());
                
                // finally, indicate accept
                true
            };
            case (?store) {
                if (Option.isSome(store.device_list.get(alias))) {
                    // device alias already registered ==> indicate reject
                    false
                } else {
                    // device not yet registered ==> check that user did not exceed limits
                    assert store.device_list.size() < MAX_DEVICES_PER_USER;
                    // all good ==> register device
                    store.device_list.put(alias, pk);
                    // indicate accept
                    true
                }
            };
        }
    };

    // Remove this user's device with given [alias]
    //
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    //      [alias] exceeds [MAX_DEVICE_ALIAS_LENGTH]
    //      [caller] has only one registered device (which we refuse to remove)
    public shared({ caller }) func remove_device(alias: En.DeviceAlias): () {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);
        assert alias.size() <= MAX_DEVICE_ALIAS_LENGTH;

        let store = expect(users.get(caller), 
            "registered user (principal " # Principal.toText(caller) # ") w/o allocated notes");

        assert store.device_list.size() > 1;

        Option.iterate(store.device_list.get(alias), func (k: En.PublicKey) {
            store.ciphertext_list.delete(k);
        });
        store.device_list.delete(alias);
    };

    // Returns:
    //      Future array of all (device, public key) pairs for this user's registered devices.
    //
    //      See also [get_notes], in particular, "Queries vs. Updates"
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    public shared({ caller }) func get_devices(): async [(En.DeviceAlias, En.PublicKey)] {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);

        let store = switch (users.get(caller)) {
            case (?s) { s };
            case null { return [] }
        };
        Iter.toArray(store.device_list.entries())
    };

    // Returns:
    //      Future array of all public keys that are not already associated with a device.
    //
    //      See also [get_notes], in particular, "Queries vs. Updates"
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    public shared({ caller }) func get_unsynced_pubkeys(): async [En.PublicKey] {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);

        let store = switch (users.get(caller)) {
            case (?s) { s };
            case null { return [] }
        };
        let entries = Iter.toArray(store.device_list.entries());

        Array.mapFilter(entries, func((alias, key): (En.DeviceAlias, En.PublicKey)): ?En.PublicKey {
            if (Option.isNull(store.ciphertext_list.get(key))) {
                ?key
            } else {
                null
            }
        })
    };

    // Returns: 
    //      `true` iff the user has at least one public key.
    //
    //      See also [get_notes], in particular, "Queries vs. Updates"
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    public shared({ caller }) func is_seeded(): async Bool {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);

        switch (users.get(caller)) {
            case null { false };
            case (?store) { store.ciphertext_list.size() > 0 }
        }
    };

    // Fetch the private key associated with this public key.
    // See also [get_notes], in particular, "Queries vs. Updates"
    // Returns:
    //      Future of an [En.Ciphertext] result
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    //      [pk] exceeds [MAX_PUBLIC_KEY_LENGTH]
    public shared({ caller }) func get_ciphertext(
        pk: En.PublicKey
    ): async Result.Result<En.Ciphertext, En.GetCiphertextError> {
        
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);        
        assert pk.size() <= MAX_PUBLIC_KEY_LENGTH;
                
        let store = switch (users.get(caller)) {
            case null { return #err(#notFound) };
            case (?s) { s }
        };
        if (not is_known_public_key(store, pk)) {
            return #err(#notFound) // pk unknown
        };
        switch (store.ciphertext_list.get(pk)) {
            case null { #err(#notSynced) };
            case (?ciphertext) { #ok(ciphertext) }
        };
    };

    // Store a list of public keys and associated private keys. 
    // Considers only public keys matching those of a registered device.
    // Does not overwrite key-value pairs that already exist.
    //
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    //      Length of [ciphertexts] exceeds [MAX_DEVICES_PER_USER]
    //      User is trying to save a known device's ciphertext exceeding [MAX_CYPHERTEXT_LENGTH]
    public shared({ caller }) func submit_ciphertexts(ciphertexts: [(En.PublicKey, En.Ciphertext)]): () {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);
        assert ciphertexts.size() <= MAX_DEVICES_PER_USER;
        
        let store = switch (users.get(caller)) {
            case null { return };
            case (?s) { s }
        };
        for ((pk, text) in ciphertexts.vals()) {
            if (is_known_public_key(store, pk) 
                and Option.isNull(store.ciphertext_list.get(pk))) {

                assert text.size() <= MAX_CYPHERTEXT_LENGTH;
                store.ciphertext_list.put(pk, text);
            }
        }
    };

    // Store a public key and associated private key in an empty user store. 
    // This function is a no-op if the user already has at least one public key stored.
    //
    // Traps: 
    //      [caller] is the anonymous identity
    //      [caller] is not a registered user
    //      [pk] exceeds [MAX_PUBLIC_KEY_LENGTH]
    //      [ctext] exceeding [MAX_CYPHERTEXT_LENGTH]
    public shared({ caller }) func seed(pk: En.PublicKey, ctext: En.Ciphertext): () {
        assert not Principal.isAnonymous(caller);
        assert is_user_registered(caller);
        assert pk.size() <= MAX_PUBLIC_KEY_LENGTH;
        assert ctext.size() <= MAX_CYPHERTEXT_LENGTH;

        let store = switch (users.get(caller)) {
            case null { return };
            case (?s) { s }
        };
        if (is_known_public_key(store, pk) and store.ciphertext_list.size() == 0) {
            store.ciphertext_list.put(pk, ctext)
        }
    };

    // Below, we implement the upgrade hooks for our canister.
    // See https://internetcomputer.org/docs/current/motoko/main/upgrades/

    // The work required before a canister upgrade begins.
    // See [nextNoteId], [stable_notesByUser], [stable_users]
    system func preupgrade() {
        Debug.print("Starting pre-upgrade hook...");
        stable_notesByUser := Iter.toArray(notesByUser.entries());
        stable_users := UserStore.serializeAll(users);
        Debug.print("pre-upgrade finished.");
    };

    // The work required after a canister upgrade ends.
    // See [nextNoteId], [stable_notesByUser], [stable_users]
    system func postupgrade() {
        Debug.print("Starting post-upgrade hook...");
        notesByUser := Map.fromIter<PrincipalName, List.List<EncryptedNote>>(
            stable_notesByUser.vals(), stable_notesByUser.size(), Text.equal, Text.hash);

        users := UserStore.deserialize(stable_users, stable_notesByUser.size());
        stable_notesByUser := [];
        Debug.print("post-upgrade finished.");
    };
};
