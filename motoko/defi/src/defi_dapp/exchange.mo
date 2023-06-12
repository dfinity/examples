import Array "mo:base/Array";
import B "mo:base/Buffer";
import Debug "mo:base/Debug";
import Float "mo:base/Float";
import Int "mo:base/Int";
import Iter "mo:base/Iter";
import L "mo:base/List";
import M "mo:base/HashMap";
import Nat "mo:base/Nat";
import Text "mo:base/Text";
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import Order "mo:base/Order";
import Principal "mo:base/Principal";
import RBTree "mo:base/RBTree";

import Book "book";
import T "types";

module {

    // internal types
    public type TradingPair = (T.Token,T.Token);

    // An exchange between ICP and DIP20 tokens.
    public class Exchange(trading_pair: TradingPair, book: Book.Book) {

        // The map of all orders (not differentiated by pairs).
        let orders = M.HashMap<T.OrderId, T.Order>(10, func(x,y){x == y}, func(x) {x});

        public func getOrder(id: T.OrderId) : ?T.Order {
            orders.get(id)
        };

        public func getOrders() : [T.Order] {
            Debug.print("List orders on exchange " # Principal.toText(trading_pair.0) # "/" # Principal.toText(trading_pair.1));
            let buff : B.Buffer<T.Order> = B.Buffer(10);
            for (o in orders.vals()) {
                buff.add(o);
            };
            buff.toArray();
        };

        // Cancel order WITHOUT verifying ownership.
        public func cancelOrder(id: T.OrderId) : ?T.Order {
            orders.remove(id)
        };

        public func addOrders(dex : Principal, orders: [T.Order]) {
            for(o in orders.vals()) {
                addOrder(dex, o);
            }
        };

        public func addOrder(dex : Principal, o: T.Order) {
            orders.put(o.id, o);
            detectMatch(dex, o);
        };

        func detectMatch(dex : Principal, order: T.Order) {
            let a = order;

            // Find matching orders.
            let matches : B.Buffer<T.Order> = B.Buffer(10);
            for(b in orders.vals()) {
                if(a.id!=b.id
                    and a.from==b.to and a.to==b.from
                    and a.fromAmount * b.fromAmount >= a.toAmount * b.toAmount
                ) {
                    matches.add(b);
                }
            };

            label iter_matches for(b in matches.vals()) {
                var a_to_amount = 0;
                var b_to_amount = 0;
                // Check if some orders can be completed in their entirety.
                if (b.fromAmount >= a.toAmount) {
                    a_to_amount := a.toAmount;
                };
                if (a.fromAmount >= b.toAmount) {
                    b_to_amount := b.toAmount;
                };

                // Check if some orders can be completed partially.
                if (a_to_amount == 0 and b_to_amount > 0) {
                    a_to_amount := b.fromAmount;
                    // Verify that we can complete the partial order with natural number tokens remaining.
                    if ((a_to_amount * a.fromAmount) % a.toAmount != 0)
                    {
                        continue iter_matches;
                    };
                };
                if (b_to_amount == 0 and a_to_amount > 0) {
                    b_to_amount := a.fromAmount;
                    // Verify that we can complete the partial order with natural number tokens remaining.
                    if ((b_to_amount * b.fromAmount) % b.toAmount != 0)
                    {
                        continue iter_matches;
                    };
                };

                if (a_to_amount > 0 and b_to_amount > 0) {
                    processTrade(dex, a, b, a_to_amount, b_to_amount);
                }
            };
        };

        func processTrade(dex : Principal, orderA : T.Order, orderB: T.Order, aToAmount: Nat, bToAmount: Nat) {
            Debug.print("Process trade between order " # Nat32.toText(orderA.id) # " and order " # Nat32.toText(orderB.id));
            let ra=orders.remove(orderA.id);
            let rb=orders.remove(orderB.id);

            // Calculate "cost" to each
            let aFromAmount : Nat = (aToAmount*orderA.fromAmount) / orderA.toAmount;
            let bFromAmount : Nat = (bToAmount*orderB.fromAmount) / orderB.toAmount;

            // Update order with remaining tokens
            let a : T.Order = {
                id = orderA.id;
                owner = orderA.owner;
                from = orderA.from;
                fromAmount = orderA.fromAmount - aFromAmount;
                to = orderA.to;
                toAmount = orderA.toAmount - aToAmount;
            };

            let b : T.Order = {
                id = orderB.id;
                owner = orderB.owner;
                from = orderB.from;
                fromAmount = orderB.fromAmount - bFromAmount;
                to = orderB.to;
                toAmount = orderB.toAmount - bToAmount;
            };

            // Update DEX balances (book)
            let removedA=book.removeTokens(a.owner, a.from, aFromAmount);
            book.addTokens(a.owner, a.to, aToAmount);
            let removedB=book.removeTokens(b.owner, b.from, bFromAmount);
            book.addTokens(b.owner, b.to, bToAmount);

            // The DEX keeps any tokens not required to satisfy the parties.
            let dex_amount_a : Nat = aFromAmount - bToAmount;
            if (dex_amount_a > 0) {
                book.addTokens(dex, a.from, dex_amount_a);
            };
            let dex_amount_b : Nat = bFromAmount - aToAmount;
            if (dex_amount_b > 0) {
                book.addTokens(dex, b.from, dex_amount_b);
            };

            // Maintain the orders only if not empty
            if (a.fromAmount != 0) {
                orders.put(a.id, a);
            };
            if (b.fromAmount != 0) {
                orders.put(b.id, b);
            };
        };

    }

}
