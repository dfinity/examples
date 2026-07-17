import Map "mo:core/Map";
import Text "mo:core/Text";
import Array "mo:core/Array";
import List "mo:core/List";
import PureList "mo:core/pure/List";
import Iter "mo:core/Iter";
import Nat "mo:core/Nat";
import Nat8 "mo:core/Nat8";
import Bool "mo:core/Bool";
import Principal "mo:core/Principal";
import Option "mo:core/Option";
import Debug "mo:core/Debug";
import Runtime "mo:core/Runtime";
import Blob "mo:core/Blob";
import Hex "./utils/Hex";

// Declare a shared actor class
// Bind the caller and the initializer
shared ({ caller = initializer }) actor class (keyName: Text) {

    // Currently, a single canister is limited to 4 GB of heap size.
    // For the current limits see https://docs.internetcomputer.org/references/resource-limits.
    // To ensure that our canister does not exceed the limit, we put various restrictions (e.g., max number of users) in place.
    // This should keep us well below a memory usage of 2 GB because
    // up to 2x memory may be needed for data serialization during canister upgrades.
    // This is sufficient for this proof-of-concept, but in a production environment the actual
    // memory usage must be calculated or monitored and the various restrictions adapted accordingly.

    // Define app limits - important for security assurance
    private transient let MAX_USERS = 500;
    private transient let MAX_NOTES_PER_USER = 200;
    private transient let MAX_NOTE_CHARS = 1000;
    private transient let MAX_SHARES_PER_NOTE = 50;

    private type PrincipalName = Text;
    private type NoteId = Nat;

    // Define public types
    // Type of an encrypted note
    // Attention: This canister does *not* perform any encryption.
    //            Here we assume that the notes are encrypted end-
    //            to-end by the front-end (at client side).
    public type EncryptedNote = {
        encryptedText : Text;
        id : Nat;
        owner : PrincipalName;
        // Principals with whom this note is shared. Does not include the owner.
        // Needed to be able to efficiently show in the UI with whom this note is shared.
        users : [PrincipalName];
    };

    // Define private fields.
    // Non-`transient` actor fields are automatically retained across canister upgrades.

    // Design choice: Use globally unique note identifiers for all users.
    //
    // This variable is retained across upgrades because it is not declared `transient`.
    //
    // See https://docs.internetcomputer.org/guides/canister-management/lifecycle/#upgrade-a-canister
    private var nextNoteId : Nat = 1;

    // Store notes by their ID, so that note-specific encryption keys can be derived.
    private transient var notesById = Map.empty<NoteId, EncryptedNote>();
    // Store which note IDs are owned by a particular principal
    private transient var noteIdsByOwner = Map.empty<PrincipalName, PureList.List<NoteId>>();
    // Store which notes are shared with a particular principal. Does not include the owner, as this is tracked by `noteIdsByOwner`.
    private transient var noteIdsByUser = Map.empty<PrincipalName, PureList.List<NoteId>>();

    // The maps above are `transient` (not retained across upgrades), so we snapshot
    // them into the retained buffer arrays below during `preupgrade` and restore
    // them in `postupgrade`.
    private var notesByIdBackup : [(NoteId, EncryptedNote)] = [];
    private var noteIdsByOwnerBackup : [(PrincipalName, PureList.List<NoteId>)] = [];
    private var noteIdsByUserBackup : [(PrincipalName, PureList.List<NoteId>)] = [];

    // Utility function that helps writing assertion-driven code more concisely.
    private func expect<T>(opt : ?T, violation_msg : Text) : T {
        switch (opt) {
            case (null) {
                Runtime.trap(violation_msg);
            };
            case (?x) {
                x;
            };
        };
    };

    private func is_authorized(user : PrincipalName, note : EncryptedNote) : Bool {
        user == note.owner or Option.isSome(Array.find(note.users, func(x : PrincipalName) : Bool { x == user }));
    };

    public shared ({ caller }) func whoami() : async Text {
        return Principal.toText(caller);
    };

    // Shared functions, i.e., those specified with [shared], are
    // accessible to remote callers.
    // The extra parameter [caller] is the caller's principal
    // See https://docs.internetcomputer.org/languages/motoko/fundamentals/actors/actors-async

    // Add new empty note for this [caller].
    //
    // Returns:
    //      Future of ID of new empty note
    // Traps:
    //      [caller] is the anonymous identity
    //      [caller] already has [MAX_NOTES_PER_USER] notes
    //      This is the first note for [caller] and [MAX_USERS] is exceeded
    public shared ({ caller }) func createNote() : async NoteId {
        assert not Principal.isAnonymous(caller);
        let owner = Principal.toText(caller);

        let newNote : EncryptedNote = {
            id = nextNoteId;
            encryptedText = "";
            owner = owner;
            users = [];
        };

        switch (Map.get(noteIdsByOwner, Text.compare, owner)) {
            case (?owner_nids) {
                assert PureList.size(owner_nids) < MAX_NOTES_PER_USER;
                ignore Map.insert(noteIdsByOwner, Text.compare, owner, PureList.pushFront(owner_nids, newNote.id));
            };
            case null {
                assert Map.size(noteIdsByOwner) < MAX_USERS;
                ignore Map.insert(noteIdsByOwner, Text.compare, owner, PureList.singleton(newNote.id));
            };
        };

        ignore Map.insert(notesById, Nat.compare, newNote.id, newNote);
        nextNoteId += 1;
        newNote.id;
    };

    // Returns (a future of) this [caller]'s notes.
    //
    // --- Queries vs. Updates ---
    // Note that this method is declared as an *update* call (see `shared`) rather than *query*.
    //
    // While queries are significantly faster than updates, they are not certified by the IC.
    // Thus, we avoid using queries throughout this app, ensuring that the result of our
    // functions gets through consensus. Otherwise, this function could e.g. omit some notes
    // if it got executed by a malicious node. (To make the app more efficient, one could
    // use an approach in which both queries and updates are combined.)
    // See https://docs.internetcomputer.org/guides/canister-calls/calling-from-clients/#query-vs-update-calls
    //
    // Returns:
    //      Future of array of EncryptedNote
    // Traps:
    //      [caller] is the anonymous identity
    public shared ({ caller }) func getNotes() : async [EncryptedNote] {
        assert not Principal.isAnonymous(caller);
        let user = Principal.toText(caller);

        let owned_notes = PureList.map(
            Option.get(Map.get(noteIdsByOwner, Text.compare, user), PureList.empty()),
            func(nid : NoteId) : EncryptedNote {
                expect(Map.get(notesById, Nat.compare, nid), "missing note with ID " # Nat.toText(nid));
            },
        );
        let shared_notes = PureList.map(
            Option.get(Map.get(noteIdsByUser, Text.compare, user), PureList.empty()),
            func(nid : NoteId) : EncryptedNote {
                expect(Map.get(notesById, Nat.compare, nid), "missing note with ID " # Nat.toText(nid));
            },
        );

        let buf = List.empty<EncryptedNote>();
        List.append(buf, List.fromArray<EncryptedNote>(PureList.toArray(owned_notes)));
        List.append(buf, List.fromArray<EncryptedNote>(PureList.toArray(shared_notes)));
        List.toArray(buf);
    };

    // Replaces the encrypted text of note with ID [id] with [encryptedText].
    //
    // Returns:
    //      Future of unit
    // Traps:
    //     [caller] is the anonymous identity
    //     note with ID [id] does not exist
    //     [caller] is not the note's owner and not a user with whom the note is shared
    //     [encryptedText] exceeds [MAX_NOTE_CHARS]
    public shared ({ caller }) func updateNote(id : NoteId, encryptedText : Text) : async () {
        assert not Principal.isAnonymous(caller);
        let caller_text = Principal.toText(caller);
        let (?note_to_update) = Map.get(notesById, Nat.compare, id) else Runtime.trap("note with id " # Nat.toText(id) # "not found");
        if (not is_authorized(caller_text, note_to_update)) {
            Runtime.trap("unauthorized");
        };
        assert note_to_update.encryptedText.size() <= MAX_NOTE_CHARS;
        ignore Map.insert(notesById, Nat.compare, id, { note_to_update with encryptedText });
    };

    // Shares the note with ID [note_id] with the [user].
    // Has no effect if the note is already shared with that user.
    //
    // Returns:
    //      Future of unit
    // Traps:
    //     [caller] is the anonymous identity
    //     note with ID [id] does not exist
    //     [caller] is not the note's owner
    public shared ({ caller }) func addUser(note_id : NoteId, user : PrincipalName) : async () {
        assert not Principal.isAnonymous(caller);
        let caller_text = Principal.toText(caller);
        let (?note) = Map.get(notesById, Nat.compare, note_id) else Runtime.trap("note with id " # Nat.toText(note_id) # "not found");
        if (caller_text != note.owner) {
            Runtime.trap("unauthorized");
        };
        assert note.users.size() < MAX_SHARES_PER_NOTE;
        if (not Option.isSome(Array.find(note.users, func(u : PrincipalName) : Bool { u == user }))) {
            let users_buf = List.fromArray<PrincipalName>(note.users);
            List.add(users_buf, user);
            let updated_note = { note with users = List.toArray(users_buf) };
            ignore Map.insert(notesById, Nat.compare, note_id, updated_note);
        };
        switch (Map.get(noteIdsByUser, Text.compare, user)) {
            case (?user_nids) {
                if (not PureList.any(user_nids, func(nid : NoteId) : Bool { nid == note_id })) {
                    ignore Map.insert(noteIdsByUser, Text.compare, user, PureList.pushFront(user_nids, note_id));
                };
            };
            case null {
                ignore Map.insert(noteIdsByUser, Text.compare, user, PureList.singleton(note_id));
            };
        };
    };

    // Unshares the note with ID [note_id] with the [user].
    // Has no effect if the note is already shared with that user.
    //
    // Returns:
    //      Future of unit
    // Traps:
    //     [caller] is the anonymous identity
    //     note with ID [id] does not exist
    //     [caller] is not the note's owner
    public shared ({ caller }) func removeUser(note_id : NoteId, user : PrincipalName) : async () {
        assert not Principal.isAnonymous(caller);
        let caller_text = Principal.toText(caller);
        let (?note) = Map.get(notesById, Nat.compare, note_id) else Runtime.trap("note with id " # Nat.toText(note_id) # "not found");
        if (caller_text != note.owner) {
            Runtime.trap("unauthorized");
        };
        let updated_note = { note with users = Array.filter(note.users, func(u : PrincipalName) : Bool { u != user }) };
        ignore Map.insert(notesById, Nat.compare, note_id, updated_note);

        switch (Map.get(noteIdsByUser, Text.compare, user)) {
            case (?user_nids) {
                let updated_nids = PureList.filter(user_nids, func(nid : NoteId) : Bool { nid != note_id });
                if (not PureList.isEmpty(updated_nids)) {
                    ignore Map.insert(noteIdsByUser, Text.compare, user, updated_nids);
                } else {
                    ignore Map.remove(noteIdsByUser, Text.compare, user);
                };
            };
            case null {};
        };
    };

    // Delete the note with ID [id].
    //
    // Returns:
    //      Future of unit
    // Traps:
    //     [caller] is the anonymous identity
    //     note with ID [id] does not exist
    //     [caller] is not the note's owner
    public shared ({ caller }) func deleteNote(note_id : NoteId) : async () {
        assert not Principal.isAnonymous(caller);
        let caller_text = Principal.toText(caller);
        let (?note_to_delete) = Map.get(notesById, Nat.compare, note_id) else Runtime.trap("note with id " # Nat.toText(note_id) # "not found");
        let owner = note_to_delete.owner;
        if (owner != caller_text) {
            Runtime.trap("unauthorized");
        };
        switch (Map.get(noteIdsByOwner, Text.compare, owner)) {
            case (?owner_nids) {
                let updated_nids = PureList.filter(owner_nids, func(nid : NoteId) : Bool { nid != note_id });
                if (not PureList.isEmpty(updated_nids)) {
                    ignore Map.insert(noteIdsByOwner, Text.compare, owner, updated_nids);
                } else {
                    ignore Map.remove(noteIdsByOwner, Text.compare, owner);
                };
            };
            case null {};
        };
        for (user in note_to_delete.users.values()) {
            switch (Map.get(noteIdsByUser, Text.compare, user)) {
                case (?user_nids) {
                    let updated_nids = PureList.filter(user_nids, func(nid : NoteId) : Bool { nid != note_id });
                    if (not PureList.isEmpty(updated_nids)) {
                        ignore Map.insert(noteIdsByUser, Text.compare, user, updated_nids);
                    } else {
                        ignore Map.remove(noteIdsByUser, Text.compare, user);
                    };
                };
                case null {};
            };
        };
        ignore Map.remove(notesById, Nat.compare, note_id);
    };

    // Only the vetKD methods in the IC management canister are required here.
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

    transient let management_canister : VETKD_API = actor ("aaaaa-aa");

    public shared func symmetricKeyVerificationKeyForNote() : async Text {
        let { public_key } = await management_canister.vetkd_public_key({
            canister_id = null;
            context = Text.encodeUtf8("note_symmetric_key");
            key_id = { curve = #bls12_381_g2; name = keyName };
        });
        Hex.encode(Blob.toArray(public_key));
    };

    public shared ({ caller }) func encryptedSymmetricKeyForNote(note_id : NoteId, transport_public_key : Blob) : async Text {
        let caller_text = Principal.toText(caller);
        let (?note) = Map.get(notesById, Nat.compare, note_id) else Runtime.trap("note with id " # Nat.toText(note_id) # "not found");
        if (not is_authorized(caller_text, note)) {
            Runtime.trap("unauthorized");
        };

        let buf = List.empty<Nat8>();
        List.append(buf, List.fromArray<Nat8>(natToBigEndianByteArray(16, note_id))); // fixed-size encoding
        List.append(buf, List.fromArray<Nat8>(Blob.toArray(Text.encodeUtf8(note.owner))));
        let input = Blob.fromArray(List.toArray(buf)); // prefix-free

        let { encrypted_key } = await (with cycles = 26_153_846_153) management_canister.vetkd_derive_key({
            input;
            context = Text.encodeUtf8("note_symmetric_key");
            key_id = { curve = #bls12_381_g2; name = keyName };
            transport_public_key;
        });
        Hex.encode(Blob.toArray(encrypted_key));
    };

    // Converts a nat to a fixed-size big-endian byte (Nat8) array
    private func natToBigEndianByteArray(len : Nat, n : Nat) : [Nat8] {
        let ith_byte = func(i : Nat) : Nat8 {
            assert (i < len);
            let shift : Nat = 8 * (len - 1 - i);
            Nat8.fromIntWrap(n / 2 ** shift);
        };
        Array.tabulate<Nat8>(len, ith_byte);
    };

    // Below, we implement the upgrade hooks for our canister.

    // The work required before a canister upgrade begins.
    system func preupgrade() {
        Debug.print("Starting pre-upgrade hook...");
        notesByIdBackup := Iter.toArray(Map.entries(notesById));
        noteIdsByOwnerBackup := Iter.toArray(Map.entries(noteIdsByOwner));
        noteIdsByUserBackup := Iter.toArray(Map.entries(noteIdsByUser));
        Debug.print("pre-upgrade finished.");
    };

    // The work required after a canister upgrade ends: restore the transient maps
    // from the retained backup buffers.
    system func postupgrade() {
        Debug.print("Starting post-upgrade hook...");

        notesById := Map.fromIter(
            notesByIdBackup.values(),
            Nat.compare,
        );
        notesByIdBackup := [];

        noteIdsByOwner := Map.fromIter(
            noteIdsByOwnerBackup.values(),
            Text.compare,
        );
        noteIdsByOwnerBackup := [];

        noteIdsByUser := Map.fromIter(
            noteIdsByUserBackup.values(),
            Text.compare,
        );
        noteIdsByUserBackup := [];

        Debug.print("post-upgrade finished.");
    };
};
