import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Int "mo:base/Int";
import Iter "mo:base/Iter";
import M "mo:base/HashMap";
import Nat64 "mo:base/Nat64";
import Nat "mo:base/Nat";
import Principal "mo:base/Principal";
import Random "mo:base/Random";
import Text "mo:base/Text";
import Time "mo:base/Time";

import DIP20 "../DIP20/motoko/src/token";
import Account "./Account";

import Ledger "canister:ledger";


actor Dex {

    type Token = Text;

    type Balance = {
        principal: Principal;
        balances: [TokenBalance];
    };

    type TokenBalance = {
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

    type OrderPlacementResponse = {
        status: Text;
        order: Order;
    };
    
    type CancelOrderResponse = {
        order_id: Text;
        status: Text;
    };

    // ----------------------------------------
    // NOTE: Initial work with a single token
    stable var dip_tokens: ?DIP20.Token = null;
    let dip_fee: Nat = 5;
    let icp_fee: Nat64 = 10_000;
    // ----------------------------------------

    stable var book_stable : [(Principal,[TokenBalance])] = [];
    stable var orders_stable : [(Text,Order)] = [];
    stable var lastId : Nat = 0;

    let book = M.fromIter<Principal,[TokenBalance]>(
        book_stable.vals(),10, Principal.equal, Principal.hash
    );
    let orders = M.fromIter<Text,Order>(
        orders_stable.vals(),10, Text.equal, Text.hash
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
        // TODO
    };

    public func place_order(from: Token, fromAmount: Nat, to: Token, toAmount: Nat) : async OrderPlacementResponse {
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
        {
            status;
            order;
        }
    };

    func nextId() : Text {
        lastId += 1;
        Nat.toText(lastId);
    };

    public func withdraw() {
        Debug.print("Withdraw...");
        // TODO
    };

    public func cancel_order(order_id: Text) : async(CancelOrderResponse) {
        Debug.print("Cancelling order "# order_id #"...");
        let o=orders.remove(order_id);
        let status = if(o==null) {
            "Not_existing"
        } else {
            "Canceled"
        };
        {
            order_id;
            status;
        }
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

    // For development only.
    public query func balances() : async([TokenBalance]) {
        Debug.print("List balances...");
        let buff : Buffer.Buffer<TokenBalance> = Buffer.Buffer(book.size());
        for (tb in book.vals()) {
            for(x in tb.vals()) {
                buff.add(x);
            };
        };
        buff.toArray();
    };

    public shared query (msg) func whoami() : async Principal {
        return msg.caller;
    };

    // ----------------------------------------
    // NOTE: Initial work with a single token
    public func init(): async ?Text {
        do ? {
            if (dip_tokens == null) {
                dip_tokens := ?(await DIP20.Token("https://dogbreedslist.com/wp-content/uploads/2019/08/Are-Golden-Retrievers-easy-to-train.png",
                    "Golden Coin",
                    "DOG",
                    8,
                    10000000000000000,
                    Principal.fromActor(Dex),
                    dip_fee));

                return ?("Initialized: '" # Principal.toText(Principal.fromActor(dip_tokens!)) # "'");
            } else {
                return ?("Already initialized at: '" # Principal.toText(Principal.fromActor(dip_tokens!)) # "'");
            };
        };
    };

    // Return the account ID specific to this user's subaccount
    public shared(msg) func deposit_address(): async Blob {
        Account.accountIdentifier(Principal.fromActor(Dex), Account.principalToSubaccount(msg.caller));
    };

    // After user transfers ICP to the target subaccount, convert all of it to tokens.
    public shared(msg) func convert_icp(): async ?Text {
        do ? {
            // Calculate target subaccount
            // NOTE: Should this be hashed first instead?
            let source_account = Account.accountIdentifier(Principal.fromActor(Dex), Account.principalToSubaccount(msg.caller));

            // Check ledger for value
            let balance = await Ledger.account_balance({ account = source_account });

            // Transfer to default subaccount
            let icp_reciept = if (balance.e8s > icp_fee) {
                await Ledger.transfer({
                    memo: Nat64    = 0;
                    from_subaccount = ?Account.principalToSubaccount(msg.caller);
                    to = Account.accountIdentifier(Principal.fromActor(Dex), Account.defaultSubaccount());
                    amount = { e8s = balance.e8s - icp_fee };
                    fee = { e8s = icp_fee };
                    created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
                })
            } else {
                Debug.trap("Cannot afford to transfer ICP.");
            };

            switch icp_reciept {
                case ( #Err _) {
                    Debug.trap("Failed to transfer ICP.");
                };
                case _ {};
            };
            // (Proactively save ICP fee for second transfer)
            let available = { e8s = balance.e8s - (icp_fee * 2) };

            // Convert ICP into DIP20
            let output: Nat = if (Nat64.toNat(balance.e8s) > dip_fee) {
                Nat64.toNat(available.e8s) - dip_fee
            } else {
                Debug.trap("Cannot afford to transfer tokens.");
            };

            // TODO: Check that we actually have enough tokens remaining to send.

            // Transfer DIP20 to target account
            let token_reciept = await dip_tokens!.transfer(msg.caller, output);

            switch token_reciept {
                case ( #Err _) {
                    Debug.trap("Failed to transfer tokens.");
                };
                case _ {};
            };

            // Return result
            "Converted '" # Nat64.toText(balance.e8s) # "' ICP into '" # Nat.toText(output) # "' tokens."
            }
    };

    // After user approves tokens to the Dex, convert all of it to ICP.
    public shared(msg) func convert_token(): async ?Text {
        do ? {
            // Check DIP20 value
            let balance: Nat = (await dip_tokens!.allowance(msg.caller, Principal.fromActor(Dex))) - dip_fee;

            // Transfer to account
            let token_reciept = if (balance > dip_fee) {
                await dip_tokens!.transferFrom(msg.caller, Principal.fromActor(Dex), balance - dip_fee);
            } else {
                Debug.trap("Cannot affort to transfer tokens.");
            };

            switch token_reciept {
                case ( #Err _) {
                    Debug.trap("Failed to transfer tokens.");
                };
                case _ {};
            };
            let available: Nat = balance - dip_fee;

            // Convert DIP20 into ICP
            let output = available;

            // TODO: Check that we actually have enough ICP remaining to send.

            // Transfer ICP to target account
            let icp_reciept = await Ledger.transfer({
                memo: Nat64 = 0;
                from_subaccount = null;
                to = Account.accountIdentifier(msg.caller, Account.defaultSubaccount());
                amount = { e8s = Nat64.fromNat(available) + icp_fee};
                fee = { e8s = icp_fee };
                created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
            });

            switch icp_reciept {
                case ( #Err _) {
                    Debug.trap("Failed to transfer ICP.");
                };
                case _ {};
            };

            // Return result
            "Converted '" # Nat.toText(available) # "' tokens into '" # Nat.toText(output) # "' ICP."
        }
    };
}
