import Map "mo:base/HashMap";
import Text "mo:base/Text";
import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import List "mo:base/List";
import Iter "mo:base/Iter";
import Int "mo:base/Int";
import Nat "mo:base/Nat";
import Bool "mo:base/Bool";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Option "mo:base/Option";
import Debug "mo:base/Debug";
import Order "mo:base/Order";
import Blob "mo:base/Blob";
import Hex "./utils/Hex";

// Declare a shared actor class
// Bind the caller and the initializer
shared({ caller = initializer }) actor class() {

    // Currently, a single canister smart contract is limited to 4 GB of storage due to WebAssembly limitations.
    // To ensure that our canister does not exceed this limit, we restrict memory usage to at most 2 GB because 
    // up to 2x memory may be needed for data serialization during canister upgrades. Therefore, we aim to support
    // up to 1,000 users, each storing up to 2 MB of data. 
    // The data is reserved for storing the notes:
    //     NOTES_PER_USER = MAX_NOTES_PER_USER x MAX_NOTE_CHARS x (4 bytes per char)
    //     2 MB = 500 x 1000 x 4 = 2,000,000

    // Define dapp limits - important for security assurance
    private let MAX_USERS = 1_000;
    private let MAX_NOTES_PER_USER = 500;
    private let MAX_NOTE_CHARS = 1000;

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

    // Returns the current number of users.
    // Traps if [users_invariant] is violated
    private func user_count(): Nat {
        notesByUser.size()
    };

    // Check that a note identifier is sane. This is needed since Motoko integers 
    // are infinite-precision. 
    // Note: avoid extraneous usage of async functions, hence [user_count]
    private func is_id_sane(id: Nat): Bool {
        0 <= id and id < MAX_NOTES_PER_USER * user_count()
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
    //      [encrypted_text] exceeds [MAX_NOTE_CHARS]
    //      User already has [MAX_NOTES_PER_USER] notes
    //      [encrypted_text] would be for a new user and [MAX_USERS] is exceeded
    public shared({ caller }) func add_note(encrypted_text: Text): async () {
        assert not Principal.isAnonymous(caller);
        assert encrypted_text.size() <= MAX_NOTE_CHARS;

        Debug.print("Adding note...");

        let principalName = Principal.toText(caller);
        let userNotes : List.List<EncryptedNote> = Option.get(notesByUser.get(principalName), List.nil<EncryptedNote>());

        if (List.isNil(userNotes)) {
            // user didn't have notes yet, so this is a new user: check that user is not going to exceed limits
            Debug.print("new user: #" # Nat.toText(user_count()));
            assert user_count() < MAX_USERS;
        };

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
    public shared({ caller }) func get_notes(): async [EncryptedNote] {
        assert not Principal.isAnonymous(caller);

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
    //      [encrypted_note.encrypted_text] exceeds [MAX_NOTE_CHARS]
    //      [encrypted_note.id] is unreasonable; see [is_id_sane]
    public shared({ caller }) func update_note(encrypted_note: EncryptedNote): async () {
        assert not Principal.isAnonymous(caller);
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
    //      [id] is unreasonable; see [is_id_sane]
    public shared({ caller }) func delete_note(id: Nat): async () {
        assert not Principal.isAnonymous(caller);
        assert is_id_sane(id);

        let principalName = Principal.toText(caller);
        var notesOfUser = Option.get(notesByUser.get(principalName), List.nil());

        notesByUser.put(
            principalName,
            List.filter(notesOfUser, func(note: EncryptedNote): Bool { note.id != id })
        )
    };

    // Only the ecdsa methods in the IC management canister is required here.
    type VETKD_SYSTEM_API = actor {
        vetkd_public_key : ({
            canister_id : ?Principal;
            derivation_path : [Blob];
            key_id : { curve: { #bls12_381; } ; name: Text };
        }) -> async ({ public_key : Blob; });
        vetkd_encrypted_key : ({
            public_key_derivation_path : [Blob];
            derivation_id : Blob;
            key_id : { curve: { #bls12_381; } ; name: Text };
            encryption_public_key : Blob;
        }) -> async ({ encrypted_key : Blob });
    };

    let vetkd_system_api : VETKD_SYSTEM_API = actor("s55qq-oqaaa-aaaaa-aaakq-cai");

    public shared({ caller }) func app_vetkd_public_key(derivation_path: [Blob]): async Text {
        let { public_key } = await vetkd_system_api.vetkd_public_key({
            canister_id = null;
            derivation_path;
            key_id = { curve = #bls12_381; name = "test_key_1" };
        });
        Hex.encode(Blob.toArray(public_key))
    };

    public shared({ caller }) func symmetric_key_verification_key(): async Text {
        let { public_key } = await vetkd_system_api.vetkd_public_key({
            canister_id = null;
            derivation_path = Array.make(Text.encodeUtf8("symmetric_key"));
            key_id = { curve = #bls12_381; name = "test_key_1" };
        });
        Hex.encode(Blob.toArray(public_key))
    };

    public shared ({ caller }) func encrypted_symmetric_key_for_caller(encryption_public_key : Blob) : async Text {
        let caller_blob = Principal.toBlob(caller);
        let { encrypted_key } = await vetkd_system_api.vetkd_encrypted_key({
            derivation_id = Principal.toBlob(caller);
            public_key_derivation_path = Array.make(Text.encodeUtf8("symmetric_key"));
            key_id = { curve = #bls12_381; name = "test_key_1" };
            encryption_public_key;
        });
        Hex.encode(Blob.toArray(encrypted_key));
    };

    // Below, we implement the upgrade hooks for our canister.
    // See https://internetcomputer.org/docs/current/motoko/main/upgrades/

    // The work required before a canister upgrade begins.
    // See [nextNoteId], [stable_notesByUser]
    system func preupgrade() {
        Debug.print("Starting pre-upgrade hook...");
        stable_notesByUser := Iter.toArray(notesByUser.entries());
        Debug.print("pre-upgrade finished.");
    };

    // The work required after a canister upgrade ends.
    // See [nextNoteId], [stable_notesByUser]
    system func postupgrade() {
        Debug.print("Starting post-upgrade hook...");
        notesByUser := Map.fromIter<PrincipalName, List.List<EncryptedNote>>(
            stable_notesByUser.vals(), stable_notesByUser.size(), Text.equal, Text.hash);

        stable_notesByUser := [];
        Debug.print("post-upgrade finished.");
    };
};
