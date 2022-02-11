import Array "mo:base/Array";
import Buffer "mo:base/Buffer";
import Debug "mo:base/Debug";
import Bool "mo:base/Debug";
import Float "mo:base/Float";
import Int "mo:base/Int";
import Int64 "mo:base/Int64";
import Iter "mo:base/Iter";
import M "mo:base/HashMap";
import Nat64 "mo:base/Nat64";
import Nat32 "mo:base/Nat32";
import Nat "mo:base/Nat";
import Hash "mo:base/Hash";
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

shared(init_msg) actor class Dex() = this {
    let icp_fee: Nat = 10_000;

    stable var orders_stable : [T.Order] = [];
    stable var lastId : Nat32 = 0;
    var exchanges = M.HashMap<E.TradingPair, E.Exchange>(10, func (k1: E.TradingPair,k2: E.TradingPair): Bool {
        Principal.equal(k1.0,k2.0) and Principal.equal(k1.1,k2.1)
    }, func (k : E.TradingPair) {
        Text.hash(Text.concat(Principal.toText(k.0),Principal.toText(k.1)))
    });

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

        // consturct trading pair which is used to select correct exchange
        // following pair is constructed (X,Y) where X is less accoriding to principal compare function
        // this is needed that buy and sell orders use the same exchange
        var trading_pair=(from,to);
        switch(create_trading_pair(from,to)){
            case(?tp){
                trading_pair:=tp;
            };
            case(null){
                return #Err(#InvalidOrder);
            }
        };

        // Check if user balance in book is enough before creating the order.
        if(book.hasEnoughBalance(owner,from,fromAmount) == false) {
            Debug.print("Not enough balance for user " # Principal.toText(owner) # " in token " # Principal.toText(from));
            return #Err(#InvalidOrder);
        };

        let exchange = switch (exchanges.get(trading_pair)) {
            case null {
                Debug.print("Creating Exchange for trading pair: " # Principal.toText(trading_pair.0) # "::" # Principal.toText(trading_pair.1));
                let exchange : E.Exchange = E.Exchange(trading_pair, book);
                exchanges.put(trading_pair,exchange);
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
         };
        exchange.addOrder(order);
        #Ok(order)
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
                            case null return #Err(#NotAllowed)
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

    public query func getOrders() : async([T.Order]) {
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
    public shared(msg) func withdraw(token: T.Token, amount: Nat, address: Principal) : async T.WithdrawReceipt {
        if (token == E.ledger()) {
            let account_id = Account.accountIdentifier(address, Account.defaultSubaccount());
            await withdrawIcp(msg.caller, amount, account_id)
        } else {
            await withdrawDip(msg.caller, token, amount, address)
        }
    };

    private func withdrawIcp(caller: Principal, amount: Nat, account_id: Blob) : async T.WithdrawReceipt {
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
            memo: Nat64    = 0;
            from_subaccount = ?Account.defaultSubaccount();
            to = account_id;
            amount = { e8s = Nat64.fromNat(amount + icp_fee) };
            fee = { e8s = Nat64.fromNat(icp_fee) };
            created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
        });

        switch icp_reciept {
            case (#Err e) {
                // add tokens back to user account balance
                book.addTokens(caller,E.ledger(),amount+icp_fee);
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        #Ok(amount)
    };

    private func withdrawDip(caller: Principal, token: T.Token, amount: Nat, address: Principal) : async T.WithdrawReceipt {
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
        let txReceipt =  await dip20.transfer(address, amount);

        switch txReceipt {
            case (#Err e) {
                // add tokens back to user account balance
                book.addTokens(caller,token,amount + dip_fee);
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


    public shared query (msg) func getAllBalances() : async [T.Balance] {
        
        // could probably allocate more but this is minimum
        let buff : Buffer.Buffer<T.Balance> = Buffer.Buffer(book.size());
        for ((owner, user_balances) in book.entries()) {
            let b : Buffer.Buffer<T.Balance> = Buffer.Buffer(user_balances.size());
            for ((token, amount) in user_balances.entries()) {
                b.add({
                    owner;
                    token;
                    amount;
                });
            };
            buff.append(b);
        };
        buff.toArray()
    };

    public shared query (msg) func whoami() : async Principal {
        return msg.caller;
    };


    // ===== DEPOSIT FUNCTIONS =====

    // Return the account ID specific to this user's subaccount
    public shared(msg) func getDepositAddress(): async Blob {
        Account.accountIdentifier(Principal.fromActor(this), Account.principalToSubaccount(msg.caller));
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
        let balance : Nat = (await dip20.allowance(caller, Principal.fromActor(this)));

        // Transfer to account.
        let token_reciept = if (balance > dip_fee) {
            await dip20.transferFrom(caller, Principal.fromActor(this),balance - dip_fee);
        } else {
            return #Err(#BalanceLow);
        };

        switch token_reciept {
            case (#Err e) {
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        let available = balance - dip_fee;

        // add transferred amount to user balance
        book.addTokens(caller,token,available);

        // Return result
        #Ok(available)
    };

    // After user transfers ICP to the target subaccount
    private func depositIcp(caller: Principal): async T.DepositReceipt {

        // Calculate target subaccount
        // NOTE: Should this be hashed first instead?
        let source_account = Account.accountIdentifier(Principal.fromActor(this), Account.principalToSubaccount(caller));

        // Check ledger for value
        let balance = await Ledger.account_balance({ account = source_account });

        // Transfer to default subaccount
        let icp_receipt = if (Nat64.toNat(balance.e8s) > icp_fee) {
            await Ledger.transfer({
                memo: Nat64    = 0;
                from_subaccount = ?Account.principalToSubaccount(caller);
                to = Account.accountIdentifier(Principal.fromActor(this), Account.defaultSubaccount());
                amount = { e8s = balance.e8s - Nat64.fromNat(icp_fee)};
                fee = { e8s = Nat64.fromNat(icp_fee) };
                created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
            })
        } else {
            return #Err(#BalanceLow);
        };

        switch icp_receipt {
            case ( #Err _) {
                return #Err(#TransferFailure);
            };
            case _ {};
        };
        let available = { e8s : Nat = Nat64.toNat(balance.e8s) - icp_fee };

        // keep track of deposited ICP
        book.addTokens(caller,E.ledger(),available.e8s);

        // Return result
        #Ok(available.e8s)
    };


    // ===== INTERNAL FUNCTIONS =====
    private func fetch_dip_fee(token: T.Token) : async Nat {
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;
        let metadata = await dip20.getMetadata();
        metadata.fee
    };

    public func getSymbol(token: T.Token) : async Text {
        let dip20 = actor (Principal.toText(token)) : T.DIPInterface;
        if (token==E.ledger()){
            return "ICP"
        };
        let metadata = await dip20.getMetadata();
        metadata.symbol
    };

    private func create_trading_pair(from: T.Token, to: T.Token) : ?E.TradingPair {
        switch(Principal.compare(from,to)){
            case(#less){
                ?(from,to)
            };
            case(#greater){
                ?(to,from)
            };
            case(#equal){
                null
            };
        };
    };

    private func create_trading_pair_symbol(from: T.Token, to: T.Token) : async ?(Text,Text) {
        let trading_pair =  switch (create_trading_pair(from,to)){
            // should not occur here since all orders already validated
            case null return null;
            case (?tp) tp;
        };
        ?(await getSymbol(trading_pair.0),await getSymbol(trading_pair.1))
    };
    
    public shared(msg) func getWithdrawalAddress(): async Blob {
        Account.accountIdentifier(msg.caller, Account.defaultSubaccount())
    };

    // For testing
    public shared(msg) func credit(user: Principal, token_canister_id: Principal, amount: Nat) {
        assert (msg.caller == init_msg.caller);
        book.addTokens(user,token_canister_id,amount);
    };

    // For testing.
    public shared(msg) func clear() {
        assert (msg.caller == init_msg.caller);
        book.clear();

        exchanges := M.HashMap<E.TradingPair, E.Exchange>(10, func (k1: E.TradingPair,k2: E.TradingPair): Bool {
                Principal.equal(k1.0,k2.0) and Principal.equal(k1.1,k2.1)
            },
            func (k : E.TradingPair) {
                Text.hash(Text.concat(Principal.toText(k.0),Principal.toText(k.1)))
            });
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
            let trading_pair = switch (create_trading_pair(o.from,o.to)){
                // should not occur here since all orders already validated
                case null (o.from,o.to);
                case (?tp) tp;
            };
            let exchange = switch (exchanges.get(trading_pair)) {
                case null {
                    let exchange : E.Exchange = E.Exchange(trading_pair, book);
                    exchanges.put(trading_pair,exchange);
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
