import Array "mo:base/Array";
import Debug "mo:base/Debug";
import Principal "mo:base/Principal";
import Iter "mo:base/Iter";

import M "mo:base/HashMap";

import T "types";

module {

    public class Book() {

        var book = M.HashMap<Principal, M.HashMap<T.Token, Nat>>(10, Principal.equal, Principal.hash);

        public func get(user: Principal) : ?M.HashMap<T.Token, Nat> {
            book.get(user)
        };

        public func put(user: Principal, userBalances: M.HashMap<T.Token, Nat>) {
            book.put(user, userBalances);
        };

        public func entries() : Iter.Iter<(Principal,M.HashMap<T.Token,Nat>)> {
            book.entries()
        };

        public func size() : Nat {
            book.size()
        };

        // For development only.
        public func print_balances(){
            for ((x, y) in book.entries()) {
                Debug.print( debug_show("PRINCIPAL: ", x));
                for ((key: T.Token, value: Nat) in y.entries()) {
                    Debug.print( debug_show("Balance: Token: ", key, " amount: ",value));
                };
            };
        };

        public func clear() {
            book := M.HashMap<Principal, M.HashMap<T.Token, Nat>>(10, Principal.equal, Principal.hash);
        };

        // function that adds tokens to book. Book keeps track of users deposits.
        public func addTokens(user: Principal, token: T.Token, amount: Nat){
            switch (book.get(user)) {
                case (?token_balance) {
                    // check if user already has existing balance for this token
                    switch (token_balance.get(token)){
                        case (?balance) {
                            Debug.print( debug_show("User", user, "has existing balance of ", token, " new amount: ",balance+amount));
                            token_balance.put(token, balance+amount);
                        };
                        case(null){
                            Debug.print( debug_show("User", user, "had no balance of ", token, " new amount: ",amount));
                            token_balance.put(token, amount);
                        };
                    };
                };
                case (null) {
                    // user didn't exist
                    Debug.print( debug_show("User", user, "had no balance of ", token, " new amount: ",amount));
                    var x1 = M.HashMap<T.Token, Nat>(2, Principal.equal, Principal.hash);
                    x1.put(token,amount);
                    book.put(user,x1);
                };
            };
        };

        // return the new balance.
        public func removeTokens(user: Principal, token: T.Token, amount: Nat) : ?Nat {
            switch (book.get(user)) {
                case (?token_balance) {
                    // check if user already has existing balance for this token
                    switch (token_balance.get(token)){
                        case (?balance) {
                            if (balance>=amount){
                                token_balance.put(token, balance-amount);
                                ?(balance-amount)
                            } else {
                                null
                            }
                        };
                        case(null){
                            Debug.print("User " # Principal.toText(user) # " has no balance of token " # Principal.toText(token));
                            null
                        };
                    };
                };
                case (null) {
                    // user didn't exist
                    Debug.print("User " # Principal.toText(user) # " doesn't exist in book, cannot remove tokens.");
                    null
                };
            };
        };

        // Return true if a user has at least amount tokens in the book, false otherwise.
        public func hasEnoughBalance(user: Principal, token: Principal, amount: Nat) : Bool {
            switch (book.get(user)) {
                case (?balances) {
                    switch(balances.get(token)) {
                        case (?balance) return balance >= amount;
                        case null return false;
                    }
                };
                case null return false;
            };
        }
    }
}
