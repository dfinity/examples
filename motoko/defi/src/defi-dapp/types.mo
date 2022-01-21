import Time "mo:base/Time";


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


    public type TxReceipt = {
        #Ok: Nat;
        #Err: {
            #InsufficientAllowance;
            #InsufficientBalance;
            #ErrorOperationStyle;
            #Unauthorized;
            #LedgerTrap;
            #ErrorTo;
            #Other;
            #BlockUsed;
            #AmountTooSmall;
        };
    };

    public type Metadata = {
        logo : Text; // base64 encoded logo or logo url
        name : Text; // token name
        symbol : Text; // token symbol
        decimals : Nat8; // token decimal
        totalSupply : Nat; // token total supply
        owner : Principal; // token owner
        fee : Nat; // fee for update calls
    };

    public type Operation = {
        #approve;
        #mint;
        #transfer;
        #transferFrom;
    };

    public type TransactionStatus = {
        #succeeded;
        #failed;
    };

    public type TxRecord = {
        caller: ?Principal;
        op: Operation; // operation type
        index: Nat; // transaction index
        from: Principal;
        to: Principal;
        amount: Nat;
        fee: Nat;
        timestamp: Time.Time;
        status: TransactionStatus;
    };

    // Dip20 token interface
    public type DIPInterface = actor {
        transfer : (Principal,Nat) ->  async TxReceipt;
        transferFrom : (Principal,Principal,Nat) -> async TxReceipt;
        allowance : (owner: Principal, spender: Principal) -> async Nat;
    };

}
