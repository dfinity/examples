import Time "mo:base/Time";


module {

    public type Token = Principal;

    public type OrderId = Nat32;
    public type Symbol = Text;

    public type OrderStatus = {
        #Submitted;
        #Cancelled;
        #Executed;
        #PartiallyExecuted;
    };

    public type Order = {
        id: OrderId;
        owner: Principal;
        from: Token;
        fromAmount: Nat;
        to: Token;
        toAmount: Nat;
        dip_symbol: Symbol;
        submitted: Time.Time;
        price: Float;
        status: OrderStatus;
    };
    
    // ledger types
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


    public type DIPInterface = actor {
        transfer : (Principal,Nat) ->  async TxReceipt;
        transferFrom : (Principal,Principal,Nat) -> async TxReceipt;
        allowance : (owner: Principal, spender: Principal) -> async Nat;
        getMetadata: () -> async Metadata;
    };

    // return types
    public type OrderPlacementReceipt = {
        #Ok: Order;
        #Err: {
            #InvalidOrder;
            #OrderBookFull;
        };
    };
    public type CancelOrderReceipt = {
        #Ok: OrderId;
        #Err: {
            #NotExistingOrder;
            #NotAllowed;
            #InternalError
        };
    };
    public type WithdrawReceipt = {
        #Ok: Nat;
        #Err: {
            #BalanceLow;
            #TransferFailure;
        };
    };
    public type DepositReceipt = {
        #Ok: Nat;
        #Err: {
            #BalanceLow;
            #TransferFailure;
        };
    };

}
