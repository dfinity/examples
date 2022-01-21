module {

    public type Token = Text;

    public type Balance = {
        principal: Principal;
        balances: [TokenBalance];
    };

    public type TokenBalance = {
        principal: Principal;
        token: Token;
        amount: Nat;
    };

    public type Order = {
        id: Text;
        owner: Principal;
        from: Token;
        fromAmount: Nat;
        to: Token;
        toAmount: Nat;
    };

    // Not required, will use ledger.
    public type Transaction = {
        id: Nat;
        time: Nat;
        price: Float;
        order1: Order;
        order2: Order;
    };

}
