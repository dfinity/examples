import Array "mo:base/Array";
import B "mo:base/Buffer";
import Debug "mo:base/Debug";
import L "mo:base/List";
import Float "mo:base/Float";
import Iter "mo:base/Iter";
import Nat "mo:base/Nat";
import RBTree "mo:base/RBTree";

import DIP20 "../DIP20/motoko/src/token";

import T "types";

module {

    type Order = T.Order;

    // An exchange between ICP and a DIP20 token.
    //public class Exchange(dip: ?DIP20.Token) {
    public class Exchange() {

        // The implicit pair will be dip/ICP (to have the price of a dip in ICP), therefore:
        // bid is for buying dip (ie selling ICP).
        // ask is for selling dip (ie buying ICP).
        var bid = RBTree.RBTree<Float,B.Buffer<Order>>(Float.compare);
        var ask = RBTree.RBTree<Float,B.Buffer<Order>>(Float.compare);

        //let dip_symbol = ?(await dip.symbol());

        public func addOrders(orders: [Order]) {
            for(o in orders.vals()) {
                addOrder(o);
            }
        };

        public func addOrder(o: Order) {
            if(o.from == "ICP") {
                // convert ICP to token.
                let price : Float = Float.fromInt(o.fromAmount) / Float.fromInt(o.toAmount);
                switch (bid.get(price)) {
                    case null {
                        let b = B.Buffer<Order>(1);
                        b.add(o);
                        bid.put(price, b);
                    };
                    case (?b) {
                        b.add(o);
                    };
                };

            } else if (o.to == "ICP") {
                // convert token to ICP.
                let price : Float = Float.fromInt(o.toAmount) / Float.fromInt(o.fromAmount);
                switch(ask.get(price)) {
                   case null {
                       let b = B.Buffer<Order>(1);
                       b.add(o);
                       ask.put(price, b);
                   };
                   case (?b) {
                        b.add(o);
                   };
                };
            } else {
                // TODO handle invalid order.
                Debug.print("Invalid order");
            };
            print_book();
            detect_match();
        };

        // For debug only.
        func print_book() {
            Debug.print("======= XXX / ICP =======");
            Debug.print("=== BID === | === ASK ===");

            let nb_bid = Iter.size(bid.entries());
            let nb_ask = Iter.size(ask.entries());

            let it_bid = bid.entriesRev();
            let it_ask = ask.entries();
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

        func sum_ask_orders(orders: B.Buffer<Order>) : Nat {
            var nb=0;
            for(o in orders.vals()) {
                nb += o.fromAmount;
            };
            nb;
        };

        func sum_bid_orders(orders: B.Buffer<Order>) : Nat {
            var nb=0;
            for(o in orders.vals()) {
                nb += o.toAmount;
            };
            nb;
        };

        func detect_match() {
            Debug.print("Detecting orders match");
            let it_bid = bid.entriesRev();
            let it_ask = ask.entries();

            switch (it_bid.next()) {
                case null return;
                case (?(bp,bo))
                    switch (it_ask.next()) {
                        case null return;
                        case (?(ap,ao)) {
                            let spread = ap-bp;
                            if (spread<=0) {
                                Debug.print("Crossing at midspread: " # Float.toText(bp+spread/2));
                                execute(bo.get(0), ao.get(0));
                            } else {
                                Debug.print("No match. Spread: " # Float.toText(spread));
                                return;
                            };
                        }
                    }
            }
            // TODO continue matching
        };

        func execute(order1: Order, order2: Order) {
            Debug.print("Executing transaction");
            // TODO
        }

        //public func transactions() : [Transaction] {}


    }

}
