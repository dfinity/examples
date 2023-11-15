import Map "mo:base/HashMap";
import Text "mo:base/Text";
import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import List "mo:base/List";
import Iter "mo:base/Iter";
import Int "mo:base/Int";
import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat8";
import Bool "mo:base/Bool";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Option "mo:base/Option";
import Debug "mo:base/Debug";
import Order "mo:base/Order";
import Blob "mo:base/Blob";
import Hash "mo:base/Hash";
import Hex "./utils/Hex";

shared ({ caller = initializer }) actor class () {

    private let MAX_USERS = 1_000;
    private let MAX_NOTES_PER_USER = 500;
    private let MAX_NOTE_CHARS = 1000;
    private let MAX_SHARES_PER_NOTE = 50;

    private type PrincipalName = Text;
    private type NoteId = Nat;

    public type EncryptedNote = {
        encrypted_text : Text;
        id : Nat;
        owner : PrincipalName;
        users : [PrincipalName];
    };

    private stable var nextNoteId : Nat = 1;

    private var notesById = Map.HashMap<NoteId, EncryptedNote>(0, Nat.equal, Hash.hash);
    private var noteIdsByOwner = Map.HashMap<PrincipalName, List.List<NoteId>>(0, Text.equal, Text.hash);
    private var noteIdsByUser = Map.HashMap<PrincipalName, List.List<NoteId>>(0, Text.equal, Text.hash);

    private func expect<T>(opt : ?T, violation_msg : Text) : T {
        switch (opt) {
            case (null) {
                Debug.trap(violation_msg);
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

    public shared ({ caller }) func create_note() : async NoteId {
        let owner = Principal.toText(caller);

        let newNote : EncryptedNote = {
            id = nextNoteId;
            encrypted_text = "";
            owner = owner;
            users = [];
        };

        switch (noteIdsByOwner.get(owner)) {
            case (?owner_nids) {
                assert List.size(owner_nids) < MAX_NOTES_PER_USER;
                noteIdsByOwner.put(owner, List.push(newNote.id, owner_nids));
            };
            case null {
                assert noteIdsByOwner.size() < MAX_USERS;
                noteIdsByOwner.put(owner, List.make(newNote.id));
            };
        };

        notesById.put(newNote.id, newNote);
        nextNoteId += 1;
        newNote.id;
    };

    public shared ({ caller }) func get_notes() : async [EncryptedNote] {
        let user = Principal.toText(caller);

        let owned_notes = List.map(
            Option.get(noteIdsByOwner.get(user), List.nil()),
            func(nid : NoteId) : EncryptedNote {
                expect(notesById.get(nid), "missing note with ID " # Nat.toText(nid));
            },
        );
        let shared_notes = List.map(
            Option.get(noteIdsByUser.get(user), List.nil()),
            func(nid : NoteId) : EncryptedNote {
                expect(notesById.get(nid), "missing note with ID " # Nat.toText(nid));
            },
        );
        Array.append(List.toArray(owned_notes), List.toArray(shared_notes));
    };

    public shared ({ caller }) func update_note(id : NoteId, encrypted_text : Text) : async () {
        let caller_text = Principal.toText(caller);
        let (?note_to_update) = notesById.get(id) else Debug.trap("note with id " # Nat.toText(id) # "not found");
        if (not is_authorized(caller_text, note_to_update)) {
            Debug.trap("unauthorized");
        };
        assert note_to_update.encrypted_text.size() <= MAX_NOTE_CHARS;
        notesById.put(id, { note_to_update with encrypted_text });
    };

    public shared ({ caller }) func add_user(note_id : NoteId, user : PrincipalName) : async () {
        let caller_text = Principal.toText(caller);
        let (?note) = notesById.get(note_id) else Debug.trap("note with id " # Nat.toText(note_id) # "not found");
        if (caller_text != note.owner) {
            Debug.trap("unauthorized");
        };
        assert note.users.size() < MAX_SHARES_PER_NOTE;
        if (not Option.isSome(Array.find(note.users, func(u : PrincipalName) : Bool { u == user }))) {
            let users_buf = Buffer.fromArray<PrincipalName>(note.users);
            users_buf.add(user);
            let updated_note = { note with users = Buffer.toArray(users_buf) };
            notesById.put(note_id, updated_note);
        };
        switch (noteIdsByUser.get(user)) {
            case (?user_nids) {
                if (not List.some(user_nids, func(nid : NoteId) : Bool { nid == note_id })) {
                    noteIdsByUser.put(user, List.push(note_id, user_nids));
                };
            };
            case null {
                noteIdsByUser.put(user, List.make(note_id));
            };
        };
    };

    public shared ({ caller }) func remove_user(note_id : NoteId, user : PrincipalName) : async () {
        let caller_text = Principal.toText(caller);
        let (?note) = notesById.get(note_id) else Debug.trap("note with id " # Nat.toText(note_id) # "not found");
        if (caller_text != note.owner) {
            Debug.trap("unauthorized");
        };
        let users_buf = Buffer.fromArray<PrincipalName>(note.users);
        users_buf.filterEntries(func(i : Nat, u : PrincipalName) : Bool { u != user });
        let updated_note = { note with users = Buffer.toArray(users_buf) };
        notesById.put(note_id, updated_note);

        switch (noteIdsByUser.get(user)) {
            case (?user_nids) {
                let updated_nids = List.filter(user_nids, func(nid : NoteId) : Bool { nid != note_id });
                if (not List.isNil(updated_nids)) {
                    noteIdsByUser.put(user, updated_nids);
                } else {
                    let _ = noteIdsByUser.remove(user);
                };
            };
            case null {};
        };
    };

    public shared ({ caller }) func delete_note(note_id : NoteId) : async () {
        let caller_text = Principal.toText(caller);
        let (?note_to_delete) = notesById.get(note_id) else Debug.trap("note with id " # Nat.toText(note_id) # "not found");
        let owner = note_to_delete.owner;
        if (owner != caller_text) {
            Debug.trap("unauthorized");
        };
        switch (noteIdsByOwner.get(owner)) {
            case (?owner_nids) {
                let updated_nids = List.filter(owner_nids, func(nid : NoteId) : Bool { nid != note_id });
                if (not List.isNil(updated_nids)) {
                    noteIdsByOwner.put(owner, updated_nids);
                } else {
                    let _ = noteIdsByOwner.remove(owner);
                };
            };
            case null {};
        };
        for (user in note_to_delete.users.vals()) {
            switch (noteIdsByUser.get(user)) {
                case (?user_nids) {
                    let updated_nids = List.filter(user_nids, func(nid : NoteId) : Bool { nid != note_id });
                    if (not List.isNil(updated_nids)) {
                        noteIdsByUser.put(user, updated_nids);
                    } else {
                        let _ = noteIdsByUser.remove(user);
                    };
                };
                case null {};
            };
        };
        let _ = notesById.remove(note_id);
    };

    type VETKD_SYSTEM_API = actor {
        vetkd_public_key : ({
            canister_id : ?Principal;
            derivation_path : [Blob];
            key_id : { curve : { #bls12_381 }; name : Text };
        }) -> async ({ public_key : Blob });
        vetkd_encrypted_key : ({
            public_key_derivation_path : [Blob];
            derivation_id : Blob;
            key_id : { curve : { #bls12_381 }; name : Text };
            encryption_public_key : Blob;
        }) -> async ({ encrypted_key : Blob });
    };

    let vetkd_system_api : VETKD_SYSTEM_API = actor ("s55qq-oqaaa-aaaaa-aaakq-cai");

    public shared ({ caller }) func symmetric_key_verification_key_for_note() : async Text {
        let { public_key } = await vetkd_system_api.vetkd_public_key({
            canister_id = null;
            derivation_path = Array.make(Text.encodeUtf8("note_symmetric_key"));
            key_id = { curve = #bls12_381; name = "test_key_1" };
        });
        Hex.encode(Blob.toArray(public_key));
    };

    public shared ({ caller }) func encrypted_symmetric_key_for_note(note_id : NoteId, encryption_public_key : Blob) : async Text {
        let caller_text = Principal.toText(caller);
        let (?note) = notesById.get(note_id) else Debug.trap("note with id " # Nat.toText(note_id) # "not found");
        if (not is_authorized(caller_text, note)) {
            Debug.trap("unauthorized");
        };
        let derivation_id = Buffer.Buffer<Nat8>(32);
        derivation_id.append(Buffer.fromArray(natToBigEndianByteArray(16, note_id))); // fixed-size encoding
        derivation_id.append(Buffer.fromArray(Blob.toArray(Text.encodeUtf8(note.owner))));

        let { encrypted_key } = await vetkd_system_api.vetkd_encrypted_key({
            derivation_id = Blob.fromArray(Buffer.toArray(derivation_id)); // prefix-free
            public_key_derivation_path = Array.make(Text.encodeUtf8("note_symmetric_key"));
            key_id = { curve = #bls12_381; name = "test_key_1" };
            encryption_public_key;
        });
        Hex.encode(Blob.toArray(encrypted_key));
    };

    private func natToBigEndianByteArray(len : Nat, n : Nat) : [Nat8] {
        let ith_byte = func(i : Nat) : Nat8 {
            assert (i < len);
            let shift : Nat = 8 * (len - 1 - i);
            Nat8.fromIntWrap(n / 2 ** shift);
        };
        Array.tabulate<Nat8>(len, ith_byte);
    };
};
