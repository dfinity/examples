import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Int "mo:base/Int";
import Int64 "mo:base/Int64";
import Iter "mo:base/Iter";
import M "mo:base/HashMap";
import Nat64 "mo:base/Nat64";
import Nat32 "mo:base/Nat32";
import Nat "mo:base/Nat";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Time "mo:base/Time";
import Result "mo:base/Result";

import DIP20 "../DIP20/motoko/src/token";
import Account "./Account";
import Ledger "canister:ledger";

import B "book";
import E "exchange";
import T "types";

actor Dex {

    // TODO: Sort out fees
    // ----------------------------------------
    // NOTE: Initial work with a single token
    let icp_fee: Nat = 10_000;
    // ----------------------------------------

    stable var orders_stable : [(T.OrderId,T.Order)] = [];
    stable var lastId : Nat32 = 0;
    var exchanges = M.HashMap<T.Token, E.Exchange>(10, Principal.equal, Principal.hash);

    // User balance datastructure
    private var book = B.Book();
    private stable var upgradeMap : [var (Principal, [(T.Token, Nat)])] = [var];


    let orders = M.fromIter<T.OrderId,T.Order>(
        orders_stable.vals(),10, Nat32.equal, func(v) {v}
    );


    // ===== ORDER FUNCTIONS =====
    public shared(msg) func place_order(from: T.Token, fromAmount: Nat, to: T.Token, toAmount: Nat) : async T.OrderPlacementReceipt {
        let id = nextId();
        Debug.print("Placing order "# Nat32.toText(id) #"...");
        let owner=msg.caller;
        let order : T.Order = {
            id;
            owner;
            from;
            fromAmount;
            to;
            toAmount;
        };

        // Find or create the exchange.
        var dip : ?T.Token = null;
        if(from==E.ledger()) {
            dip := ?to;
        } else if(to==E.ledger()) {
            dip := ?from;
        } else {
            Debug.print("Order must be from or to ICP.");
        };

        orders.put(id, order);

        switch(dip) {
            case (?dip_token) {
                let exchange = switch (exchanges.get(dip_token)) {
                    case null {
                        let dip_symbol = await symbol(dip_token);
                        let exchange : E.Exchange = E.Exchange(dip_token, dip_symbol, book);
                        exchanges.put(dip_token,exchange);
                        exchange
                    };
                    case (?e) e
                };
                exchange.addOrder(order);
                #Ok(order)
            };
            case null {
                #Err(#InvalidOrder)
            };
        }
    };

    public shared(msg) func cancel_order(order_id: T.OrderId) : async T.CancelOrderReceipt {
        Debug.print("Cancelling order "# Nat32.toText(order_id) #"...");
        switch (orders.get(order_id)) {
            case null
                return #Err(#NotExistingOrder);
            case (?order)
                if(order.owner != msg.caller) {
                    #Err(#NotAllowed)
                } else {
                    orders.delete(order_id);
                    #Ok(order_id)
                }
        };
    };

    public func check_order(order_id: T.OrderId) : async(?T.Order) {
        Debug.print("Checking order "# Nat32.toText(order_id) #"...");
        orders.get(order_id);
    };

    public query func list_order() : async([T.Order]) {
        Debug.print("List orders...");
        let buff : Buffer.Buffer<T.Order> = Buffer.Buffer(orders.size());
        for (o in orders.vals()) {
            buff.add(o);
        };
        buff.toArray();
    };

    func nextId() : Nat32 {
        lastId += 1;
        lastId;
    };

    // ===== WITHDRAW FUNCTIONS =====
    public shared(msg) func withdraw_icp(amount: Nat) : async T.WithdrawReceipt {
        Debug.print("Withdraw...");

        // remove withdrawal amount from book
        switch (book.remove_tokens(msg.caller, E.ledger(), amount+icp_fee)){
            case(null){
                return #Err(#BalanceLow)
            };
            case _ {};
        };

        // Transfer amount back to user
        let icp_reciept =  await Ledger.transfer({
            // todo: memo relevant?
            memo: Nat64    = 0;
            from_subaccount = ?Account.defaultSubaccount();
            to = Account.accountIdentifier(msg.caller, Account.defaultSubaccount());
            amount = { e8s = Nat64.fromNat(amount) };
            fee = { e8s = Nat64.fromNat(icp_fee) };
            created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
        });

        switch icp_reciept {
            case (#Err e) {
                // add tokens back to user account balance
                book.add_tokens(msg.caller,E.ledger(),amount+icp_fee);
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        #Ok(amount)
    };

    public shared(msg) func withdraw_dip(token: T.Token, amount: Nat) : async T.WithdrawReceipt {
        Debug.print("Withdraw...");

        // cast canisterID to token interface
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;

        // get dip20 fee
        let dip_fee = await fetch_dip_fee(token);

        // remove withdrawal amount from book
        switch (book.remove_tokens(msg.caller,token,amount+dip_fee)){
            case(null){
                return #Err(#BalanceLow)
            };
            case _ {};
        };

        // Transfer amount back to user
        let txReceipt =  await dip20.transfer(msg.caller, amount - dip_fee);

        switch txReceipt {
            case (#Err e) {
                // add tokens back to user account balance
                book.add_tokens(msg.caller,token,amount + dip_fee);
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        return #Ok(amount)
    };


    // ===== DEX STATE FUNCTIONS =====
    public shared query (msg) func balance(token: T.Token) : async Nat {
        switch (book.get(msg.caller)) {
            case (?token_balance) {
                switch (token_balance.get(token)){
                    case (?balance) {
                        return balance;
                    };
                    case(null){
                        return 0;
                    };
                };
            };
            case (null) {
                return 0;
            };
        };
    };

    public shared query (msg) func whoami() : async Principal {
        return msg.caller;
    };


    // ===== DEPOSIT FUNCTIONS =====

    // Return the account ID specific to this user's subaccount
    public shared(msg) func deposit_address(): async Blob {
        Account.accountIdentifier(Principal.fromActor(Dex), Account.principalToSubaccount(msg.caller));
    };

    // After user transfers ICP to the target subaccount
    public shared(msg) func deposit_dip(token: T.Token): async T.DepositReceipt {
        // ATTENTION!!! NOT SAFE
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;

        // get DIP fee
        let dip_fee = await fetch_dip_fee(token);

        // Check DIP20 allowance for DEX
        let balance : Nat = (await dip20.allowance(msg.caller, Principal.fromActor(Dex))) - dip_fee;

        // Transfer to account.
        let token_reciept = if (balance > dip_fee) {
            await dip20.transferFrom(msg.caller, Principal.fromActor(Dex),balance - dip_fee);
        } else {
            return #Err(#BalanceLow);
        };

        switch token_reciept {
            case (#Err e) {
                return #Err(#TransferFailure);
            };
            case _ {};
        };

        // add transferred amount to user balance
        book.add_tokens(msg.caller,token,balance - dip_fee);

        // Return result
        #Ok(balance - dip_fee)
    };

    // After user transfers ICP to the target subaccount
    public shared(msg) func deposit_icp(): async T.DepositReceipt {

        // Calculate target subaccount
        // NOTE: Should this be hashed first instead?
        let source_account = Account.accountIdentifier(Principal.fromActor(Dex), Account.principalToSubaccount(msg.caller));

        // Check ledger for value
        let balance = await Ledger.account_balance({ account = source_account });

        // Transfer to default subaccount
        let icp_reciept = if (Nat64.toNat(balance.e8s) > icp_fee) {
            await Ledger.transfer({
                memo: Nat64    = 0;
                from_subaccount = ?Account.principalToSubaccount(msg.caller);
                to = Account.accountIdentifier(Principal.fromActor(Dex), Account.defaultSubaccount());
                amount = { e8s = balance.e8s - Nat64.fromNat(icp_fee)};
                fee = { e8s = Nat64.fromNat(icp_fee) };
                created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
            })
        } else {
            return #Err(#BalanceLow);
        };

        switch icp_reciept {
            case ( #Err _) {
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        // (Proactively save ICP fee for second transfer)
        let available = { e8s : Nat = Nat64.toNat(balance.e8s) - (icp_fee * 2) };

        // keep track of deposited ICP
        book.add_tokens(msg.caller,E.ledger(),available.e8s);

        // Return result
        #Ok(available.e8s)
    };


    // ===== INTERNAL FUNCTIONS =====
    private func fetch_dip_fee(token: T.Token) : async Nat {
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;
        let metadata = await dip20.getMetadata();
        metadata.fee
    };

    public func symbol(token: T.Token) : async Text {
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;
        let metadata = await dip20.getMetadata();
        metadata.symbol
    };


    // Required since maps cannot be stable and need to be moved to stable memory
    // Before canister upgrade book hashmap gets stored in stable memory such that it survives updates
    system func preupgrade() {
        upgradeMap := Array.init(book.size(), (Principal.fromText(""), []));
        var i = 0;
        for ((x, y) in book.entries()) {
            upgradeMap[i] := (x, Iter.toArray(y.entries()));
            i += 1;
        };
        orders_stable := Iter.toArray(orders.entries());
    };
    // After canister upgrade book map gets reconstructed from stable array
    system func postupgrade() {
        for ((key: Principal, value: [(T.Token, Nat)]) in upgradeMap.vals()) {
            let tmp: M.HashMap<T.Token, Nat> = M.fromIter<T.Token, Nat>(Iter.fromArray<(T.Token, Nat)>(value), 10, Principal.equal, Principal.hash);
            book.put(key, tmp);
        };
        upgradeMap := [var];
        orders_stable := [];
        // TODO reload exchanges (find solution for async symbol retrieving).
        //exchange := E.Exchange(dip_tokens);
    };

}
