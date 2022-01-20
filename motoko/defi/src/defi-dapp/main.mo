import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Iter "mo:base/Iter";
import M "mo:base/HashMap";
import Nat "mo:base/Nat";
import Principal "mo:base/Principal";
import Random "mo:base/Random";
import Text "mo:base/Text";

actor Dex {

    type Token = Text;

    type Balance = {
        principal: Principal;
        token: Token;
        amount: Nat;
    };

    type Order = {
        id: Text;
        from: Token;
        fromAmount: Nat;
        to: Token;
        toAmount: Nat;
    };

    type OrderPlacementResult = {
        status: Text;
    };


    stable var book_stable : [(Principal,Balance)] = [];
    stable var orders_stable : [(Text,Order)] = [];
    stable var lastId : Nat = 0;

    let book = M.fromIter<Principal,Balance>(
        book_stable.vals(),10, Principal.equal, Principal.hash
    );
    let orders = M.fromIter<Text,Order>(
        orders_stable.vals(),10,Text.equal,Text.hash
    );
    // Required since maps cannot be stable.
    system func preupgrade() {
        book_stable := Iter.toArray(book.entries());
        orders_stable := Iter.toArray(orders.entries());
    };
    system func postupgrade() {
        book_stable := [];
        orders_stable := [];
    };

    public func deposit() {
        Debug.print("Deposit...");
    };

    public func place_order(from: Token, fromAmount: Nat, to: Token, toAmount: Nat) : async OrderPlacementResult {
        let id : Text = nextId();
        Debug.print("Placing order "# id #"...");
        let order : Order = {
            id;
            from;
            fromAmount;
            to;
            toAmount;
        };
        orders.put(id, order);
        let status = "Ok";
        let res : OrderPlacementResult = {
            status;
        };
        res;
    };

    func nextId() : Text {
        lastId += 1;
        Nat.toText(lastId);
    };

    public func withdraw() {
        Debug.print("Withdraw...");
    };

    public func cancel_order(order_id: Text) {
        Debug.print("Cancelling order "# order_id #"...");
    };

    public func check_order(order_id: Text) : async(?Order) {
        Debug.print("Checking order "# order_id #"...");
        orders.get(order_id);
    };

    public query func list_order() : async([Order]) {
        Debug.print("List orders...");
        let buff : Buffer.Buffer<Order> = Buffer.Buffer(orders.size());
        for (o in orders.vals()) {
            buff.add(o);
        };
        buff.toArray();
    };

}
