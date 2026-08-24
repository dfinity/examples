import Map "mo:core/Map";
import Text "mo:core/Text";
import Array "mo:core/Array";
import List "mo:core/List";
import PureList "mo:core/pure/List";
import Nat "mo:core/Nat";
import Nat8 "mo:core/Nat8";
import Bool "mo:core/Bool";
import Principal "mo:core/Principal";
import Option "mo:core/Option";
import Runtime "mo:core/Runtime";
import Blob "mo:core/Blob";
import Hex "./utils/Hex";

actor {
    // The vetKD key name this canister derives from. Set the `VETKD_KEY_NAME`
    // canister environment variable (see `icp.yaml`) to pick a different key; it
    // defaults to `test_key_1` so a deploy can never leave the canister
    // half-initialized.
    //
    // This is a stable variable on purpose, so it is captured at the first install
    // and never re-read: changing the environment variable later is silently
    // ignored, and only a reinstall — which drops all data — switches keys. The key
    // feeds vetKD derivation, so a different key cannot decrypt what the old one
    // encrypted, and this canister only ever sees ciphertext and so cannot
    // re-encrypt. Were it `transient` instead, a changed variable would take effect
    // on the next upgrade and silently orphan the stored data.
    let keyName = Runtime.envVar<system>("VETKD_KEY_NAME") ?? "test_key_1";

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
    // Actor fields are automatically retained across canister upgrades unless
    // declared `transient` (enhanced orthogonal persistence), so no `preupgrade`/
    // `postupgrade` hooks are needed to persist the state below.
    //
    // See https://docs.internetcomputer.org/guides/canister-management/lifecycle/#upgrade-a-canister

    // Design choice: Use globally unique note identifiers for all users.
    private var nextNoteId : Nat = 1;

    // Store notes by their ID, so that note-specific encryption keys can be derived.
    private let notesById = Map.empty<NoteId, EncryptedNote>();
    // Store which note IDs are owned by a particular principal
    private let noteIdsByOwner = Map.empty<PrincipalName, PureList.List<NoteId>>();
    // Store which notes are shared with a particular principal. Does not include the owner, as this is tracked by `noteIdsByOwner`.
    private let noteIdsByUser = Map.empty<PrincipalName, PureList.List<NoteId>>();

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
        user == note.owner or Option.isSome(note.users.find(func(x : PrincipalName) : Bool { x == user }));
    };

    public shared ({ caller }) func whoami() : async Text {
        return caller.toText();
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
        assert not caller.isAnonymous();
        let owner = caller.toText();

        let newNote : EncryptedNote = {
            id = nextNoteId;
            encryptedText = "";
            owner = owner;
            users = [];
        };

        switch (noteIdsByOwner.get(owner)) {
            case (?owner_nids) {
                assert owner_nids.size() < MAX_NOTES_PER_USER;
                ignore noteIdsByOwner.insert(owner, owner_nids.pushFront(newNote.id));
            };
            case null {
                assert noteIdsByOwner.size() < MAX_USERS;
                ignore noteIdsByOwner.insert(owner, PureList.singleton(newNote.id));
            };
        };

        ignore notesById.insert(newNote.id, newNote);
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
        assert not caller.isAnonymous();
        let user = caller.toText();

        let owned_notes = PureList.map(
            noteIdsByOwner.get(user).get(PureList.empty()),
            func(nid : NoteId) : EncryptedNote {
                expect(notesById.get(nid), "missing note with ID " # nid.toText());
            },
        );
        let shared_notes = PureList.map(
            noteIdsByUser.get(user).get(PureList.empty()),
            func(nid : NoteId) : EncryptedNote {
                expect(notesById.get(nid), "missing note with ID " # nid.toText());
            },
        );

        let buf = List.empty<EncryptedNote>();
        buf.append(List.fromArray<EncryptedNote>(owned_notes.toArray()));
        buf.append(List.fromArray<EncryptedNote>(shared_notes.toArray()));
        buf.toArray();
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
        assert not caller.isAnonymous();
        let caller_text = caller.toText();
        let (?note_to_update) = notesById.get(id) else Runtime.trap("note with id " # id.toText() # "not found");
        if (not is_authorized(caller_text, note_to_update)) {
            Runtime.trap("unauthorized");
        };
        assert note_to_update.encryptedText.size() <= MAX_NOTE_CHARS;
        ignore notesById.insert(id, { note_to_update with encryptedText });
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
        assert not caller.isAnonymous();
        let caller_text = caller.toText();
        let (?note) = notesById.get(note_id) else Runtime.trap("note with id " # note_id.toText() # "not found");
        if (caller_text != note.owner) {
            Runtime.trap("unauthorized");
        };
        assert note.users.size() < MAX_SHARES_PER_NOTE;
        if (not Option.isSome(note.users.find(func(u : PrincipalName) : Bool { u == user }))) {
            let users_buf = List.fromArray(note.users);
            users_buf.add(user);
            let updated_note = { note with users = users_buf.toArray() };
            ignore notesById.insert(note_id, updated_note);
        };
        switch (noteIdsByUser.get(user)) {
            case (?user_nids) {
                if (not user_nids.any(func(nid : NoteId) : Bool { nid == note_id })) {
                    ignore noteIdsByUser.insert(user, user_nids.pushFront(note_id));
                };
            };
            case null {
                ignore noteIdsByUser.insert(user, PureList.singleton(note_id));
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
        assert not caller.isAnonymous();
        let caller_text = caller.toText();
        let (?note) = notesById.get(note_id) else Runtime.trap("note with id " # note_id.toText() # "not found");
        if (caller_text != note.owner) {
            Runtime.trap("unauthorized");
        };
        let updated_note = { note with users = note.users.filter(func(u : PrincipalName) : Bool { u != user }) };
        ignore notesById.insert(note_id, updated_note);

        switch (noteIdsByUser.get(user)) {
            case (?user_nids) {
                let updated_nids = user_nids.filter(func(nid : NoteId) : Bool { nid != note_id });
                if (not updated_nids.isEmpty()) {
                    ignore noteIdsByUser.insert(user, updated_nids);
                } else {
                    noteIdsByUser.remove(user);
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
        assert not caller.isAnonymous();
        let caller_text = caller.toText();
        let (?note_to_delete) = notesById.get(note_id) else Runtime.trap("note with id " # note_id.toText() # "not found");
        let owner = note_to_delete.owner;
        if (owner != caller_text) {
            Runtime.trap("unauthorized");
        };
        switch (noteIdsByOwner.get(owner)) {
            case (?owner_nids) {
                let updated_nids = owner_nids.filter(func(nid : NoteId) : Bool { nid != note_id });
                if (not updated_nids.isEmpty()) {
                    ignore noteIdsByOwner.insert(owner, updated_nids);
                } else {
                    noteIdsByOwner.remove(owner);
                };
            };
            case null {};
        };
        for (user in note_to_delete.users.values()) {
            switch (noteIdsByUser.get(user)) {
                case (?user_nids) {
                    let updated_nids = user_nids.filter(func(nid : NoteId) : Bool { nid != note_id });
                    if (not updated_nids.isEmpty()) {
                        ignore noteIdsByUser.insert(user, updated_nids);
                    } else {
                        noteIdsByUser.remove(user);
                    };
                };
                case null {};
            };
        };
        notesById.remove(note_id);
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
        Hex.encode(public_key.toArray());
    };

    public shared ({ caller }) func encryptedSymmetricKeyForNote(note_id : NoteId, transport_public_key : Blob) : async Text {
        let caller_text = caller.toText();
        let (?note) = notesById.get(note_id) else Runtime.trap("note with id " # note_id.toText() # "not found");
        if (not is_authorized(caller_text, note)) {
            Runtime.trap("unauthorized");
        };

        let buf = List.empty<Nat8>();
        buf.append(List.fromArray(natToBigEndianByteArray(16, note_id))); // fixed-size encoding
        buf.append(List.fromArray(note.owner.encodeUtf8().toArray()));
        let input = buf.toArray().toBlob(); // prefix-free

        let { encrypted_key } = await (with cycles = 26_153_846_153) management_canister.vetkd_derive_key({
            input;
            context = Text.encodeUtf8("note_symmetric_key");
            key_id = { curve = #bls12_381_g2; name = keyName };
            transport_public_key;
        });
        Hex.encode(encrypted_key.toArray());
    };

    // Converts a nat to a fixed-size big-endian byte (Nat8) array
    private func natToBigEndianByteArray(len : Nat, n : Nat) : [Nat8] {
        let ith_byte = func(i : Nat) : Nat8 {
            assert (i < len);
            let shift : Nat = 8 * (len - 1 - i);
            Nat8.fromIntWrap(n / 2 ** shift);
        };
        Array.tabulate(len, ith_byte);
    };
};
