import Option "mo:base/Option";
import Iter "mo:base/Iter";
import Map "mo:base/HashMap";
import Buffer "mo:base/Buffer";
import Array "mo:base/Array";

import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Nat "mo:base/Nat";

import En "types";

module {

    // Each element (a, b, c, d) represents the following:
    //  "a" = user principal,  "b" = device public key,  "c" = device alias,  "d" = the optional ciphertext
    public type StableUserStoreEntry = (Principal, En.PublicKey, En.DeviceAlias, ?En.Ciphertext);

    public class UserStore(principal: Principal, starting_buf_size: Nat) {

        public let device_list = Map.HashMap<En.DeviceAlias, En.PublicKey>(starting_buf_size, Text.equal, Text.hash);
        public let ciphertext_list = Map.HashMap<En.PublicKey, En.Ciphertext>(10, Text.equal, Text.hash);

        private var stable_users: [StableUserStoreEntry] = [];

        public func get_principal(): Principal = principal;

        public func serialize(): [StableUserStoreEntry] {

            // sort devices by public key
            let devices: [(En.DeviceAlias, En.PublicKey)] = En.sort_pairs_by_column(
                device_list.entries(),
                #right);

            // sort ciphertexts by public key
            let ciphertexts: [(En.PublicKey, En.Ciphertext)] = En.sort_pairs_by_column(
                ciphertext_list.entries(),
                #left);

            // Invariant: 
            // is_sorted(devices) ^ is_sorted(ciphertexts) 
            // ^ is_subset(set(ciphertexts#left), set(devices#right))
            
            // Zipping idea:
            // devices     [ (a, x)       (b, y)     (c, z)       (d, t)       ]
            // ciphertexts [ (x, 0)                  (z, 1)       (t, 2)       ]
            // buf         [ (x, Some(0)) (y, None)  (z, Some(1)) (t, Some(2)) ]

            let buf = Buffer.Buffer<StableUserStoreEntry>(
                devices.size());

            var pos = 0;
            var cyp_pos = 0;
            while (pos < devices.size()) {
                let device_alias: En.DeviceAlias = devices[pos].0;
                let pub_key: En.PublicKey = devices[pos].1;

                if (cyp_pos < ciphertexts.size() and pub_key == ciphertexts[cyp_pos].0) {
                    let cipher_text: En.Ciphertext = ciphertexts[cyp_pos].1;
                    buf.add((principal, pub_key, device_alias, Option.make(cipher_text)));
                    cyp_pos += 1;
                } else {
                    buf.add((principal, pub_key, device_alias, null));
                };

                pos += 1;
            };
            buf.toArray()
        }
    };

    public func serializeAll(users: Map.HashMap<Principal, UserStore>): [StableUserStoreEntry] = 
        Array.flatten(Iter.toArray(Iter.map(
            users.vals(), 
            func(user_store: UserStore): [StableUserStoreEntry] =
                user_store.serialize())));

    public func deserialize(serial: [StableUserStoreEntry], starting_buf_size: Nat): Map.HashMap<Principal, UserStore> {

        let users = Map.HashMap<Principal, UserStore>(starting_buf_size, Principal.equal, Principal.hash);

        for (entry in serial.vals()) {
            let principal: Principal = entry.0;
            let pub_key: En.PublicKey = entry.1;
            let device_alias: En.DeviceAlias = entry.2;
            let cipher_text_maybe: ?En.Ciphertext = entry.3;

            switch (users.get(principal)) {
                case (null) {
                    // initialize user store
                    let new_store = UserStore(principal, starting_buf_size);
                    new_store.device_list.put(device_alias, pub_key);
                    switch (cipher_text_maybe) {
                        case (null) {};
                        case (?cipher_text) new_store.ciphertext_list.put(pub_key, cipher_text);
                    };
                    users.put(principal, new_store);
                };
                case (?store) {
                    // extend user store
                    store.device_list.put(device_alias, pub_key);
                    switch (cipher_text_maybe) {
                        case (null) {};
                        case (?cipher_text) store.ciphertext_list.put(pub_key, cipher_text);
                    };
                };
            };
        };

        users
    };

}