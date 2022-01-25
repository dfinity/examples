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

import E "exchange";
import T "types";

actor Dex {

    // TODO: Sort out fees
    // ----------------------------------------
    // NOTE: Initial work with a single token
    let icp_fee: Nat64 = 10_000;
    // ----------------------------------------

    stable var orders_stable : [(T.OrderId,T.Order)] = [];
    stable var lastId : Nat32 = 0;
    var exchanges = M.HashMap<T.Token, E.Exchange>(10, Principal.equal, Principal.hash);

    // User balance datastructure
    private var book = M.HashMap<Principal, M.HashMap<T.Token, Nat64>>(10, Principal.equal, Principal.hash);
    private stable var upgradeMap : [var (Principal, [(T.Token, Nat64)])] = [var];


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
        orders.put(id, order);
        // Find or create the exchange.
        var dip : ?T.Token = null;
        if(from==E.ledger()) {
            dip := ?to;
        } else if(to==E.ledger()) {
            dip := ?from;
        } else {
            Debug.print("Order must be from or to ICP.");
        };
        switch(dip) {
            case (?dip_token) {
                let exchange = switch (exchanges.get(dip_token)) {
                    case null {
                        let exchange : E.Exchange = E.Exchange(dip_token);
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
    public shared(msg) func withdraw_icp(amount: Nat64) : async T.WithdrawReceipt {
        Debug.print("Withdraw...");

        // remove withdrawl amount from book
        switch (remove_from_book(msg.caller,E.ledger(),amount+icp_fee)){
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
            amount = { e8s = amount };
            fee = { e8s = icp_fee };
            created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
        });

        switch icp_reciept {
            case (#Err e) {
                // add tokens back to user account balance
                add_deposit_to_book(msg.caller,E.ledger(),amount+icp_fee);
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        #Ok(amount)
    };

    public shared(msg) func withdraw_dip(token: T.Token, amount: Nat64) : async T.WithdrawReceipt {
        Debug.print("Withdraw...");

        // cast canisterID to token interface
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;

        // get dip20 fee
        let dip_fee = await fetch_dip_fee(token);

        // remove withdrawl amount from book
        switch (remove_from_book(msg.caller,token,amount+dip_fee)){
            case(null){
                return #Err(#BalanceLow)
            };
            case _ {};
        };

        // Transfer amount back to user
        let txReceipt =  await dip20.transfer(msg.caller, Nat64.toNat(amount - dip_fee));

        switch txReceipt {
            case (#Err e) {
                // add tokens back to user account balance
                add_deposit_to_book(msg.caller,token,amount + dip_fee);
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        return #Ok(amount)
    };


    // ===== DEX STATE FUNCTIONS =====
    public shared query (msg) func balance(token: T.Token) : async Nat64 {
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
        let balance = Nat64.fromNat(await dip20.allowance(msg.caller, Principal.fromActor(Dex))) - dip_fee;

        // Transfer to account.
        let token_reciept = if (balance > dip_fee) {
            await dip20.transferFrom(msg.caller, Principal.fromActor(Dex),Nat64.toNat(balance - dip_fee));
        } else {
            return #Err(#BalanceLow);
        };

        switch token_reciept {
            case (#Err e) {
                return #Err(#TransferFailure);
            };
            case _ {};
        };

        // add transfered amount to useres balance
        add_deposit_to_book(msg.caller,token,balance - dip_fee);

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
            return #Err(#BalanceLow);
        };

        switch icp_reciept {
            case ( #Err _) {
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        // (Proactively save ICP fee for second transfer)
        let available = { e8s = balance.e8s - (icp_fee * 2) };

        // keep track of deposited ICP
        add_deposit_to_book(msg.caller,E.ledger(),available.e8s);

        // Return result
        #Ok(available.e8s)
    };


    // ===== INTERNAL FUNCTIONS =====
    private func fetch_dip_fee(token: T.Token) : async Nat64 {

        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;
        let metadata = await dip20.getMetadata();

        return Nat64.fromNat(metadata.fee);
    };

    // function that adds tokens to book. Book keeps track of users deposits.
    private func add_deposit_to_book(user: Principal, token: T.Token,amount: Nat64){
        switch (book.get(user)) {
            case (?token_balance) {
                // check if user already has existing balance for this token
                switch (token_balance.get(token)){
                    case (?balance) {
                        Debug.print( debug_show("User", user, "has existing balance of ", token, " new amount: ",balance+amount));
                        token_balance.put(token, balance+amount);
                    };
                    case(null){
                        Debug.print( debug_show("User", user, "has no balance of ", token, " new amount: ",amount));
                        token_balance.put(token, amount);
                    };
                };
            };
            case (null) {
                // user didn't exist
                Debug.print( debug_show("User", user, "has no balance of ", token, " new amount: ",amount));
                var x1 = M.HashMap<T.Token, Nat64>(2, Principal.equal, Principal.hash);
                x1.put(token,amount);
                book.put(user,x1);
            };
        };

    };

    // function that adds tokens to book. Book keeps track of users deposits.
    private func remove_from_book(user: Principal, token: T.Token,amount: Nat64) : ?Nat64 {

        switch (book.get(user)) {
            case (?token_balance) {
                // check if user already has existing balance for this token
                switch (token_balance.get(token)){
                    case (?balance) {
                        Debug.print( debug_show("User", user, "has existing balance of ", token, " new amount: ",balance+amount));
                        if (balance>=amount){
                            token_balance.put(token, balance-amount);
                            ?(balance-amount)
                        } else {
                            null
                        }
                    };
                    case(null){
                        null
                    };
                };
            };
            case (null) {
                // user didn't exist
                null
            };
        };
    };


    // For development only.
    private func print_balances(){
        for ((x, y) in book.entries()) {
            Debug.print( debug_show("PRINCIPLE: ", x));
            var i=0;
            for ((key: T.Token, value: Nat64) in y.entries()) {
                Debug.print( debug_show("Balance: ", i, "Token: ", key, " amount: ",value));
            };
        };
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
        for ((key: Principal, value: [(T.Token, Nat64)]) in upgradeMap.vals()) {
            let tmp: M.HashMap<T.Token, Nat64> = M.fromIter<T.Token, Nat64>(Iter.fromArray<(T.Token, Nat64)>(value), 10, Principal.equal, Principal.hash);
            book.put(key, tmp);
        };
        upgradeMap := [var];
        orders_stable := [];
        //exchange := E.Exchange(dip_tokens);
    };

}
