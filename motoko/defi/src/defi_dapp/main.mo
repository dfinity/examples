import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Float "mo:base/Float";
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
    let icp_fee: Nat = 10_000;

    stable var orders_stable : [T.Order] = [];
    stable var lastId : Nat32 = 0;
    var exchanges = M.HashMap<T.Symbol, E.Exchange>(10, Text.equal, Text.hash);

    // User balance datastructure
    private var book = B.Book();
    private stable var book_stable : [var (Principal, [(T.Token, Nat)])] = [var];

    // ===== ORDER FUNCTIONS =====
    public shared(msg) func placeOrder(from: T.Token, fromAmount: Nat, to: T.Token, toAmount: Nat) : async T.OrderPlacementReceipt {
        let id = nextId();
        Debug.print("");
        Debug.print("Placing order "# Nat32.toText(id) #" from user " # Principal.toText(msg.caller) # " for selling " # Nat.toText(fromAmount) # " tokens " # Principal.toText(from));
        let owner=msg.caller;
        let submitted = Time.now();
        var price : Float = -1;

        // Find dip token and symbol.
        var dip : ?T.Token = null;
        if(from==E.ledger()) {
            dip := ?to;
            price := Float.fromInt(fromAmount) / Float.fromInt(toAmount);
        } else if(to==E.ledger()) {
            dip := ?from;
            price := Float.fromInt(toAmount) / Float.fromInt(fromAmount);
        } else {
            Debug.print("Order must be from or to ICP.");
        };

        // Check if user balance in book is enough before creating the order.
        if(book.hasEnoughBalance(owner,from,fromAmount) == false) {
            Debug.print("Not enough balance for user " # Principal.toText(owner) # " in token " # Principal.toText(from));
            return #Err(#InsufficientFunds);
        };

        switch(dip) {
            case (?dip_token) {
                let dip_symbol = await symbol(dip_token);
                let exchange = switch (exchanges.get(dip_symbol)) {
                    case null {
                        let exchange : E.Exchange = E.Exchange(dip_token, dip_symbol, book);
                        exchanges.put(dip_symbol,exchange);
                        exchange
                    };
                    case (?e) e
                };
                let order : T.Order = {
                    id;
                    owner;
                    from;
                    fromAmount;
                    to;
                    toAmount;
                    dip_symbol;
                    submitted;
                    price;
                    status = #Submitted;
                };
                exchange.addOrder(order);
                #Ok(order)
            };
            case null {
                #Err(#InvalidOrder)
            };
        }
    };

    public shared(msg) func cancelOrder(order_id: T.OrderId) : async T.CancelOrderReceipt {
        Debug.print("Cancelling order "# Nat32.toText(order_id) #"...");
        for(e in exchanges.vals()) {
            switch (e.getOrder(order_id)) {
                case (?order)
                    if(order.owner != msg.caller) {
                        return #Err(#NotAllowed);
                    } else {
                        switch (e.cancelOrder(order_id)) {
                            case (?canceled) return #Ok(canceled.id);
                            case null return #Err(#InternalError)
                        }
                    };
                case null {}
            };
        };
        return #Err(#NotExistingOrder);
    };

    public func getOrder(order_id: T.OrderId) : async(?T.Order) {
        Debug.print("Checking order "# Nat32.toText(order_id) #"...");
        for(e in exchanges.vals()) {
            switch (e.getOrder(order_id)) {
                case (?order) return ?order;
                case null {}
            };
        };
        null;
    };

    public query func listOrders() : async([T.Order]) {
        Debug.print("List orders...");
        getAllOrders()
    };

    private func getAllOrders() : [T.Order] {
        let buff : Buffer.Buffer<T.Order> = Buffer.Buffer(10);
        for(e in exchanges.vals()) {
            for(o in e.getOrders().vals()) {
                buff.add(o);
            };
        };
        buff.toArray();
    };

    private func nextId() : Nat32 {
        lastId += 1;
        lastId;
    };

    // ===== WITHDRAW FUNCTIONS =====
    public shared(msg) func withdraw(token: T.Token, amount: Nat) : async T.WithdrawReceipt {
        if (token == E.ledger()) {
            await withdrawIcp(msg.caller, amount)
        } else {
            await withdrawDip(msg.caller, token, amount)
        }
    };

    private func withdrawIcp(caller: Principal, amount: Nat) : async T.WithdrawReceipt {
        Debug.print("Withdraw...");

        // remove withdrawal amount from book
        switch (book.removeTokens(caller, E.ledger(), amount+icp_fee)){
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
            to = Account.accountIdentifier(caller, Account.defaultSubaccount());
            amount = { e8s = Nat64.fromNat(amount) };
            fee = { e8s = Nat64.fromNat(icp_fee) };
            created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
        });

        switch icp_reciept {
            case (#Err e) {
                // add tokens back to user account balance
                book.add_tokens(caller,E.ledger(),amount+icp_fee);
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        #Ok(amount)
    };

    private func withdrawDip(caller: Principal, token: T.Token, amount: Nat) : async T.WithdrawReceipt {
        Debug.print("Withdraw...");

        // cast canisterID to token interface
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;

        // get dip20 fee
        let dip_fee = await fetch_dip_fee(token);

        // remove withdrawal amount from book
        switch (book.removeTokens(caller,token,amount+dip_fee)){
            case(null){
                return #Err(#BalanceLow)
            };
            case _ {};
        };

        // Transfer amount back to user
        let txReceipt =  await dip20.transfer(caller, amount - dip_fee);

        switch txReceipt {
            case (#Err e) {
                // add tokens back to user account balance
                book.add_tokens(caller,token,amount + dip_fee);
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        return #Ok(amount)
    };


    // ===== DEX STATE FUNCTIONS =====
    public shared query (msg) func getBalance(token: T.Token) : async Nat {
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

    public shared query (msg) func getBalances() : async [T.Balance] {
        switch (book.get(msg.caller)) {
            case (?token_balance) {
                Array.map<(T.Token, Nat),T.Balance>(Iter.toArray(token_balance.entries()), func (k : T.Token, v: Nat) : T.Balance {
                    {
                        owner = msg.caller;
                        token = k;
                        amount = v;
                    }
                })
            };
            case (null) {
                return [];
            };
        };
    };

    public shared query (msg) func whoami() : async Principal {
        return msg.caller;
    };


    // ===== DEPOSIT FUNCTIONS =====

    // Return the account ID specific to this user's subaccount
    public shared(msg) func depositAddress(): async Blob {
        Account.accountIdentifier(Principal.fromActor(Dex), Account.principalToSubaccount(msg.caller));
    };

    public shared(msg) func deposit(token: T.Token): async T.DepositReceipt {
        Debug.print("Depositing Token: " # Principal.toText(token) # " LEDGER: " # Principal.toText(E.ledger()));
        if (token == E.ledger()) {
            Debug.print("Depositing ICP");
            await depositIcp(msg.caller)
        } else {
            Debug.print("Depositing DIP20");
            await depositDip(msg.caller, token)
        }
    };

    // After user approves tokens to the DEX
    private func depositDip(caller: Principal, token: T.Token): async T.DepositReceipt {
        // ATTENTION!!! NOT SAFE
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;

        // get DIP fee
        let dip_fee = await fetch_dip_fee(token);

        // Check DIP20 allowance for DEX
        let balance : Nat = (await dip20.allowance(caller, Principal.fromActor(Dex)));

        // Transfer to account.
        let token_reciept = if (balance > dip_fee) {
            await dip20.transferFrom(caller, Principal.fromActor(Dex),balance - dip_fee);
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
        book.add_tokens(caller,token,balance - dip_fee);

        // Return result
        #Ok(balance - dip_fee)
    };

    // After user transfers ICP to the target subaccount
    private func depositIcp(caller: Principal): async T.DepositReceipt {

        // Calculate target subaccount
        // NOTE: Should this be hashed first instead?
        let source_account = Account.accountIdentifier(Principal.fromActor(Dex), Account.principalToSubaccount(caller));

        // Check ledger for value
        let balance = await Ledger.account_balance({ account = source_account });

        // Transfer to default subaccount
        let icp_reciept = if (Nat64.toNat(balance.e8s) > icp_fee) {
            await Ledger.transfer({
                memo: Nat64    = 0;
                from_subaccount = ?Account.principalToSubaccount(caller);
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
        book.add_tokens(caller,E.ledger(),available.e8s);

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
        book_stable := Array.init(book.size(), (Principal.fromText(""), []));
        var i = 0;
        for ((x, y) in book.entries()) {
            book_stable[i] := (x, Iter.toArray(y.entries()));
            i += 1;
        };
        orders_stable := getAllOrders();
    };

    // After canister upgrade book map gets reconstructed from stable array
    system func postupgrade() {
        // Reload book.
        for ((key: Principal, value: [(T.Token, Nat)]) in book_stable.vals()) {
            let tmp: M.HashMap<T.Token, Nat> = M.fromIter<T.Token, Nat>(Iter.fromArray<(T.Token, Nat)>(value), 10, Principal.equal, Principal.hash);
            book.put(key, tmp);
        };

        // TODO Reload exchanges (find solution for async symbol retrieving).
        for(o in orders_stable.vals()) {
            let exchange = switch (exchanges.get(o.dip_symbol)) {
                case null {
                    let dip_token = if(o.from==E.ledger()) { o.to } else { o.from };
                    let exchange : E.Exchange = E.Exchange(dip_token, o.dip_symbol, book);
                    exchanges.put(o.dip_symbol,exchange);
                    exchange
                };
                case (?e) e
            };
            exchange.addOrder(o);
        };

        // Clean stable memory.
        book_stable := [var];
        orders_stable := [];
    };

}
