import Debug "mo:base/Debug";

import Option "mo:base/Option";
import Iter "mo:base/Iter";
import Array "mo:base/Array";
import List "mo:base/List";
import Text "mo:base/Text";
import Principal "mo:base/Principal";

import M "mo:matchers/Matchers";
import T "mo:matchers/Testable";
import Suite "mo:matchers/Suite";
import HM "mo:matchers/matchers/Hashmap";

import En "../types";
import UserStore "../user_store";

// Custom [TestableItem] for serialized [UserStore] entries.
// See https://github.com/kritzcreek/motoko-matchers/blob/master/src/Testable.mo
func tuple4(ta: Principal, tb: En.PublicKey, tc: En.DeviceAlias, td: ?En.Ciphertext): 
    T.TestableItem<UserStore.StableUserStoreEntry> = {
    
    item = (ta, tb, tc, td);

    display = func ((a, b, c, d): UserStore.StableUserStoreEntry): Text =
        "(" # Principal.toText(a) # ", " # b # ", " # c # ", " # Option.get(d, "<none>") # ")";

    equals = func ((a1, b1, c1, d1): UserStore.StableUserStoreEntry, 
                   (a2, b2, c2, d2): UserStore.StableUserStoreEntry): Bool =
        Principal.equal(a1, a2) and 
        Text.equal(b1, b2) and 
        Text.equal(c1, c2) and 
        d1 == d2;
};

// Custom [TestableItem] for values of type [Principal].
// See https://github.com/kritzcreek/motoko-matchers/blob/master/src/Testable.mo
func princ(p: Principal): T.TestableItem<Principal> = {
    item = p;

    display = Principal.toText;

    equals = Principal.equal;
};

func user_store(us: UserStore.UserStore): T.TestableItem<UserStore.UserStore> = {
    item = us;

    display = func (us: UserStore.UserStore): Text = 
        "UserStore(principal = " # Principal.toText(us.get_principal()) # ") {\n"
        # "    device_list =     " # Array.foldLeft<(En.DeviceAlias, En.PublicKey), Text>(
            Iter.toArray(us.device_list.entries()),
            "",
            func (buf: Text, (alias, pk): (En.DeviceAlias, En.PublicKey)): Text = buf # "  (" # alias # " -> " # pk # ")") 
        # ";\n"
        # "    ciphertext_list = " # Array.foldLeft<(En.PublicKey, En.Ciphertext), Text>(
            Iter.toArray(us.ciphertext_list.entries()),
            "",
            func (buf: Text, (pk, ct): (En.PublicKey, En.Ciphertext)): Text = buf # "  (" # pk # " -> " # ct # ")")
        # ";\n"
        # "}";

    equals = func (us1: UserStore.UserStore, us2: UserStore.UserStore): Bool {
        let s1: List.List<UserStore.StableUserStoreEntry> = List.fromArray(us1.serialize());
        let s2: List.List<UserStore.StableUserStoreEntry> = List.fromArray(us2.serialize());
        (List.size(s1) == List.size(s2))
        and List.all(
            List.zip(s1, s2), 
            func ((a, b): (UserStore.StableUserStoreEntry, UserStore.StableUserStoreEntry)): Bool {
                a.0 == b.0 and  // Principal
                a.1 == b.1 and  // En.PublicKey
                a.2 == b.2 and  // En.DeviceAlias
                a.3 == b.3      // ?En.Ciphertext
            })
    }
};

func PopulateUserStore(
    principal: Principal, 
    dev_keys: [En.PublicKey], 
    dev_aliases: [En.DeviceAlias], 
    ciphertexts: [?En.Ciphertext],
    permutation: [Nat]): UserStore.UserStore {

    let store = UserStore.UserStore(principal, 10);
    for (i in Iter.fromArray(permutation)) {
        let key = dev_keys[i];
        let alias = dev_aliases[i];
        store.device_list.put(alias, key);
        switch (ciphertexts[i]) {
            case (null) {};
            case (?ciphertext) {
                store.ciphertext_list.put(key, ciphertext);
            };
        };
    };
    store
};

func C(t: Text): ?Text = Option.make(t);

let USER_1 = Principal.fromText("2vxsx-fae");
let USER_1_dev_keys    = [ "A1", "B1", "C1", "D1" ];
let USER_1_dev_aliases = [ "a1", "b1", "c1", "d1" ];
let USER_1_ciphertexts = [ C("Ax"), C("Bx"), C("Cx"), C("Dx") ];
let USER_1_permuts = [
    [0, 1, 2, 3],  // default order
    [3, 2, 1, 0],  // test permutation 1
    [2, 0, 3, 1],  // test permutation 2
];
func USER_1_store(permutation: [Nat]): UserStore.UserStore = 
    PopulateUserStore(
        USER_1, 
        USER_1_dev_keys, 
        USER_1_dev_aliases, 
        USER_1_ciphertexts, 
        permutation);

let USER_2 = Principal.fromText("2vxsx-fae");  // TODO: use a distinct principal for USER_2
let USER_2_dev_keys    = [ "A2", "B2", "C2", "D2", "E2" ];
let USER_2_dev_aliases = [ "a2", "b2", "c2", "d2", "e2" ];
let USER_2_ciphertexts = [ null, null, C("Cy"), C("Dy"), null ];
let USER_2_permuts = [
    [0, 1, 2, 3, 4],  // default order
    [4, 3, 2, 1, 0],  // test permutation 1
    [3, 0, 4, 2, 1],  // test permutation 2
];
func USER_2_store(permutation: [Nat]): UserStore.UserStore = 
    PopulateUserStore(
        USER_2, 
        USER_2_dev_keys, 
        USER_2_dev_aliases, 
        USER_2_ciphertexts, 
        permutation);

Suite.run(
    Suite.suite("UserStore", [
        Suite.suite("UserStore.serialize", 
            Array.append(
                Array.map(USER_1_permuts, func (perm: [Nat]): Suite.Suite =
                    Suite.test(
                        "Serializing a user store with 4 fully-synced devices", 
                        USER_1_store(perm).serialize(), 
                        M.array([
                            M.equals(tuple4(USER_1, "A1", "a1", Option.make("Ax"))),
                            M.equals(tuple4(USER_1, "B1", "b1", Option.make("Bx"))),
                            M.equals(tuple4(USER_1, "C1", "c1", Option.make("Cx"))),
                            M.equals(tuple4(USER_1, "D1", "d1", Option.make("Dx"))),
                        ]))),
                Array.map(USER_2_permuts, func (perm: [Nat]): Suite.Suite =
                    Suite.test(
                        "Serializing a user store with 5 devices only 3 of which are synced", 
                        USER_2_store(perm).serialize(), 
                        M.array([
                            M.equals(tuple4(USER_2, "A2", "a2", null)),
                            M.equals(tuple4(USER_2, "B2", "b2", null)),
                            M.equals(tuple4(USER_2, "C2", "c2", Option.make("Cy"))),
                            M.equals(tuple4(USER_2, "D2", "d2", Option.make("Dy"))),
                            M.equals(tuple4(USER_2, "E2", "e2", null)),
                        ]))))),
        Suite.suite("UserStore.deserialize", 
            Array.append(
                [
                    Suite.test(
                        "Smoke test",
                        UserStore.deserialize(USER_1_store(USER_1_permuts[0]).serialize(), 10),
                        M.allOf([
                            HM.hasKey<Principal, UserStore.UserStore>(princ(USER_1)),
                            M.not_(HM.atKey<Principal, UserStore.UserStore>(princ(USER_1), M.equals(user_store(USER_2_store(USER_2_permuts[0])))))
                        ]))
                ],
                Array.append(
                    Array.map(USER_1_permuts, func (perm: [Nat]): Suite.Suite = 
                        Suite.test(
                            "Deserialize a serialized user store with 4 fully-synced devices",
                            UserStore.deserialize(USER_1_store(perm).serialize(), 10),
                            M.allOf([
                                HM.hasKey<Principal, UserStore.UserStore>(princ(USER_1)),
                                HM.atKey<Principal, UserStore.UserStore>(princ(USER_1), M.equals(user_store(USER_1_store(perm))))
                            ]))),
                    Array.map(USER_2_permuts, func (perm: [Nat]): Suite.Suite = 
                        Suite.test(
                            "Deserialize a serialized user store with 5 devices only 3 of which are synced",
                            UserStore.deserialize(USER_2_store(perm).serialize(), 10),
                            M.allOf([
                                HM.hasKey<Principal, UserStore.UserStore>(princ(USER_2)),
                                HM.atKey<Principal, UserStore.UserStore>(princ(USER_2), M.equals(user_store(USER_2_store(perm))))
                            ]))))))
    ]));

