import Array "mo:base/Array";
import B "mo:base/Buffer";
import Debug "mo:base/Debug";
import Float "mo:base/Float";
import Int "mo:base/Int";
import Iter "mo:base/Iter";
import L "mo:base/List";
import M "mo:base/HashMap";
import Nat "mo:base/Nat";
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import Order "mo:base/Order";
import Principal "mo:base/Principal";
import RBTree "mo:base/RBTree";

import DIP20 "../DIP20/motoko/src/token";
import Ledger "canister:ledger";

import Book "book";
import T "types";

module {

    public let ledger = func(): Principal { Principal.fromActor(Ledger) };

    // An exchange between ICP and a DIP20 token.
    public class Exchange(dip: T.Token, symbol: Text, book: Book.Book) {

        // The implicit pair will be dip/ICP (to have the price of a dip in ICP), therefore:
        // bid is for buying dip (ie selling ICP).
        // ask is for selling dip (ie buying ICP).
        let orders_bid = M.HashMap<T.OrderId, T.Order>(10, func(x,y){x == y}, func(x) {x});
        let orders_ask = M.HashMap<T.OrderId, T.Order>(10, func(x,y){x == y}, func(x) {x});

        public func getSymbol() : Text { symbol };

        public func getOrder(id: T.OrderId) : ?T.Order {
            switch (orders_bid.get(id)) {
                case (?bid) ?bid;
                case null {
                    switch(orders_ask.get(id)) {
                        case(?ask) ?ask;
                        case null null
                    }
                }
            }
        };

        public func getOrders() : [T.Order] {
            Debug.print("List orders on exchange " # symbol);
            let buff : B.Buffer<T.Order> = B.Buffer(10);
            for (o in orders_bid.vals()) {
                buff.add(o);
            };
            for (o in orders_ask.vals()) {
                buff.add(o);
            };
            buff.toArray();
        };

        // Cancel order WITHOUT verifying ownership.
        public func cancelOrder(id: T.OrderId) : ?T.Order {
            var r = orders_bid.remove(id);
            if(r==null) {
                r := orders_ask.remove(id);
            };
            r
        };

        public func addOrders(orders: [T.Order]) {
            for(o in orders.vals()) {
                addOrder(o);
            }
        };

        public func addOrder(o: T.Order) {
            if(o.from == ledger()) {
                // convert ICP to token.
                orders_bid.put(o.id, o);
            } else if (o.to == ledger()) {
                // convert token to ICP.
                orders_ask.put(o.id, o);
            } else {
                // TODO handle invalid order (already filtered in main but still need proper error msg here).
                Debug.print("Invalid order");
            };
            print_book();
            var m=true;
            while(m) {
                m:=detect_match();
                if(m) {
                    Debug.print("Order book after execution:");
                    print_book();
                }
            };

        };

        // Print the order book in bid/ask columns.
        // For debug only.
        func print_book() {
            let bid = RBTree.RBTree<Float,B.Buffer<T.Order>>(Float.compare);
            let ask = RBTree.RBTree<Float,B.Buffer<T.Order>>(Float.compare);
            for(o in orders_bid.vals()) {
                switch (bid.get(o.price)) {
                    case null {
                        let b = B.Buffer<T.Order>(1);
                        b.add(o);
                        bid.put(o.price, b);
                    };
                    case (?b) b.add(o);
                };
            };
            for(o in orders_ask.vals()) {
                switch(ask.get(o.price)) {
                   case null {
                       let b = B.Buffer<T.Order>(1);
                       b.add(o);
                       ask.put(o.price, b);
                   };
                   case (?b) b.add(o);
                };
            };

            let nb_bid = Iter.size(bid.entries());
            let nb_ask = Iter.size(ask.entries());
            let it_bid = bid.entriesRev();
            let it_ask = ask.entries();

            Debug.print("======= " # symbol # " / ICP =======");
            Debug.print("=== BID === | === ASK ===");
            var i = 0;
            while (i < nb_bid or i < nb_ask) {
                let sb = switch(it_bid.next()) {
                    case null "-";
                    case (?(p,o)) Nat.toText(sum_bid_orders(o)) # " " # Float.toText(p)
                };
                let sa = switch(it_ask.next()) {
                    case null "-";
                    case (?(p,o)) Float.toText(p) # " " # Nat.toText(sum_ask_orders(o))
                };
                Debug.print(sb # " | " # sa);
                i+=1;
            };

        };

        func sum_ask_orders(orders: B.Buffer<T.Order>) : Nat {
            var nb=0;
            for(o in orders.vals()) {
                nb += o.fromAmount;
            };
            nb;
        };

        func sum_bid_orders(orders: B.Buffer<T.Order>) : Nat {
            var nb=0;
            for(o in orders.vals()) {
                nb += o.toAmount;
            };
            nb;
        };

        private func asc(o1: T.Order, o2: T.Order) : Order.Order {
            if(o1.price < o2.price) {
                #less
            } else if(o1.price > o2.price) {
                #greater
            } else {
                #equal
            }
        };

        private func dsc(o1: T.Order, o2: T.Order) : Order.Order {
            if(o1.price < o2.price) {
                #greater
            } else if(o1.price > o2.price) {
                #less
            } else {
                #equal
            }
        };

        func detect_match() : Bool {
            Debug.print("Detecting orders match");
            var bids=Iter.toArray(orders_bid.vals());
            var asks=Iter.toArray(orders_ask.vals());
            bids := Array.sort(bids,dsc);
            asks := Array.sort(asks,asc);

            //let it_bid = bid.entriesRev();
            //let it_ask = ask.entries();
            let it_bid = bids.vals();
            let it_ask = asks.vals();

            switch (it_bid.next()) {
                case null return false;
                case (?bo) {
                    switch (it_ask.next()) {
                        case null return false;
                        case (?ao) {
                            let spread = ao.price-bo.price;
                            if (spread<=0) {
                                let price=bo.price+spread/2;
                                Debug.print("Crossing at midspread: " # Float.toText(price));
                                return execute(bo, ao, price);
                            } else {
                                Debug.print("No match. Spread: " # Float.toText(spread));
                                return false;
                            };
                        }
                    }
                }
            }
            // TODO continue matching
        };

        // Execute two orders with matching prices.
        func execute(bid: T.Order, ask: T.Order, price: Float) : Bool {

            // for debug.
            Debug.print("Book balances:");
            book.print_balances();

            // Find volume of DIP20 (min of volumes).
            var vol_dip : Nat = 0;
            if (bid.toAmount < ask.fromAmount) {
                // complete bid order.
                vol_dip := bid.toAmount;
            } else {
                // complete ask order.
                vol_dip := ask.fromAmount;
            };
            let vol_icp_int : Int = Float.toInt(Float.fromInt(vol_dip) * price);
            if(vol_icp_int < 1) {
                Debug.print("[execution] Invalid ICP volume");
                return false;
            };
            let vol_icp : Nat = Int.abs(vol_icp_int);
            Debug.print("Executing exchange of " # Nat.toText(vol_dip) # " DIP for " # Nat.toText(vol_icp) # " ICP (price " # Float.toText(price) # " icp per dip)" );

            // we transfer the icp from bid to ask and the dip from ask to bid.
            switch (book.removeTokens(bid.owner, bid.from, vol_icp)) {
                case (?new_icp_balance) {
                    switch (book.removeTokens(ask.owner, ask.from, vol_dip)) {
                        case (?new_dip_balance) {
                            // Numbers match, adding tokens.
                            book.add_tokens(bid.owner, bid.to, vol_dip);
                            book.add_tokens(ask.owner, ask.to, vol_icp);
                            // Remove/update executed orders.
                            // Check if orders were complete or partial.
                            if(bid.toAmount == vol_dip) {
                                Debug.print("Bid order complete");
                                let r=orders_bid.remove(bid.id);
                            } else {
                                Debug.print("Bid order partially executed");
                                switch (orders_bid.remove(bid.id)) {
                                    case null return false; // TODO handle error
                                    case (?r) {
                                        Debug.print("updating bid order " # Nat32.toText(r.id));
                                        let remainingToAmount : Nat = r.toAmount - vol_dip;
                                        //let remainingFromAmount : Nat = r.fromAmount - vol_icp;
                                        // reduced proportionally to the reduction in amount.
                                        let remainingFromAmount : Nat = Int.abs(Float.toInt(1/price * Float.fromInt(remainingToAmount)));
                                        let order : T.Order = {
                                            id = r.id;
                                            owner = r.owner;
                                            from = r.from;
                                            fromAmount = remainingFromAmount;
                                            to = r.to;
                                            toAmount = remainingToAmount;
                                            dip_symbol = r.dip_symbol;
                                            submitted = r.submitted;
                                            price = r.price;
                                            status = #PartiallyExecuted;
                                        };
                                        orders_bid.put(r.id, order);
                                    }
                                }
                            };
                            if(ask.fromAmount == vol_dip) {
                                // ask complete.
                                Debug.print("Ask order complete");
                                let r=orders_ask.remove(ask.id);
                            } else {
                                Debug.print("Ask order partially executed");
                                switch (orders_ask.remove(ask.id)) {
                                    case null return false; // TODO handle error
                                    case (?r) {
                                        Debug.print("updating ask order " # Nat32.toText(r.id));
                                        let remainingFromAmount : Nat = r.fromAmount - vol_dip;
                                        // reduced proportionally to the reduction in amount.
                                        let remainingToAmount : Nat = Int.abs(Float.toInt(price * Float.fromInt(remainingFromAmount)));
                                        let order : T.Order = {
                                            id = r.id;
                                            owner = r.owner;
                                            from = r.from;
                                            fromAmount = remainingFromAmount;
                                            to = r.to;
                                            toAmount = remainingToAmount;
                                            dip_symbol = r.dip_symbol;
                                            submitted = r.submitted;
                                            price = r.price;
                                            status = #PartiallyExecuted;
                                        };
                                        orders_ask.put(r.id, order);
                                    }
                                }
                            };
                            return true;
                        };
                        case null return false;
                    }
                };
                case null return false;
            }
        }

        //public func transactions() : [Transaction] {}


    }

}
