import Array "mo:base/Array";
import Debug "mo:base/Debug";
import L "mo:base/List";
import Float "mo:base/Float";
import Nat "mo:base/Nat";
import RBTree "mo:base/RBTree";

import DIP20 "../DIP20/motoko/src/token";

import T "types";

module {

    type Order = T.Order;

    // An exchange between ICP and a DIP20 token.
    //public class Exchange(dip: ?DIP20.Token) {
    public class Exchange() {

        // The implicit pair will be ICP/dip (to have the price of a dip in ICP), therefore:
        // bid is for buying ICP/dip (ie selling dip).
        // ask is for selling ICP/dip (ie buying ICP)
        var bid = RBTree.RBTree<Float,Order>(Float.compare);
        var ask = RBTree.RBTree<Float,Order>(Float.compare);

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
                bid.put(price,o);
            } else if (o.to == "ICP") {
                // convert token to ICP.

                let price : Float = Float.fromInt(o.toAmount) / Float.fromInt(o.fromAmount);
                ask.put(price,o);
            } else {
                // TODO handle invalid order.
            };
            print_book();
            detect_match();
        };

        // For debug only.
        func print_book() {
            Debug.print("======= ICP / XXX =======");
            Debug.print("=== BID === | === ASK ===");
            let it_bid = bid.entriesRev();
            let it_ask = ask.entries();
            var be = it_bid.next();
            var ae = it_ask.next();
            while (be !=null or ae != null) {
                let sb = switch(be) {case null "-"; case (?(p,o)) Nat.toText(o.toAmount) # " " # Float.toText(p) };
                let sa = switch(ae) {case null "-"; case (?(p,o)) Float.toText(p) # " " # Nat.toText(o.fromAmount) };
                Debug.print(sb # " | " # sa);
                if(be!=null) {
                    be:=it_bid.next();
                };
                if(ae!=null) {
                    ae:=it_ask.next();
                };
            };
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
                                execute(bo,ao);
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
