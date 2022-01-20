import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Iter "mo:base/Iter";
import M "mo:base/HashMap";
import Nat "mo:base/Nat";
import Principal "mo:base/Principal";

actor Dex {

    type Token = Text;

    type Balance = {
        principal: Principal;
        token: Token;
        amount: Nat;
    };

    type Order = {
        from: Token;
        fromAmount: Nat;
        to: Token;
        toAmount: Nat;
    };

    type OrderPlacementResult = {
        status: Text;
    };

    stable var orders : [Order] = [];
    stable var balances : [(Principal,Balance)] = [];

    let book = M.fromIter<Principal,Balance>(
        balances.vals(),10, Principal.equal, Principal.hash
    );
    // Required since maps cannot be stable.
    system func preupgrade() {
        balances := Iter.toArray(book.entries());
    };
    system func postupgrade() {
        balances := [];
    };


    public func deposit() {
        Debug.print("Deposit...");
    };

    public func place_order(from: Token, fromAmount: Nat, to: Token, toAmount: Nat) : async OrderPlacementResult {
        Debug.print("Place order...");
        let order : Order = {
            from;
            fromAmount;
            to;
            toAmount;
        };
        let buff : Buffer.Buffer<Order> = Buffer.Buffer(orders.size());
        buff.add(order);
        orders := buff.toArray();
        let status = "Ok";
        let res : OrderPlacementResult = {
            status;
        };
        res;
    };

    public func withdraw() {
        Debug.print("Withdraw...");
    };

    public func cancel_order(order_id: Nat) {
        Debug.print("Cancelling order "# Nat.toText(order_id) #"...");
    };

    public func check_order(order_id: Nat) {
        Debug.print("Checking order "# Nat.toText(order_id) #"...");
    };

    public query func list_order() : async([Order]) {
        Debug.print("List order...");
        orders;
    };

}
