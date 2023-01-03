import Ledger "canister:nns-ledger";

import ICPUtils "./ICPUtils";
import T "./Types";

import Blob "mo:base/Blob";
import Nat "mo:base/Nat";
import Nat64 "mo:base/Nat64";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Time "mo:base/Time";

module {
  public type Memo = Nat64;

  public type Tokens = {
    e8s : Nat64;
  };

  public type TimeStamp = {
    timestamp_nanos : Nat64;
  };

  public type AccountIdentifier = Blob;

  public type SubAccount = Blob;

  public type BlockIndex = Nat64;

  public type TransferError = {
    message : ?Text;
    kind : {
      #BadFee : {
        expected_fee : Tokens;
      };
      #InsufficientFunds : {
        balance : Tokens;
      };
      #TxTooOld : {
        allowed_window_nanos : Nat64;
      };
      #TxCreatedInFuture;
      #TxDuplicate : {
        duplicate_of : BlockIndex;
      };
      #Other;
    };
  };

  public type TransferArgs = {
    memo : Memo;
    amount : Tokens;
    fee : Tokens;
    from_subaccount : ?SubAccount;
    to : AccountIdentifier;
    created_at_time : ?TimeStamp;
  };

  public type TransferResult = Result.Result<T.TransferSuccess, TransferError>;

  public func transfer(args : TransferArgs) : async TransferResult {
    let result = await Ledger.transfer(args);
    switch result {
      case (#Ok index) {
        return #ok({ blockHeight = index });
      };
      case (#Err err) {
        switch err {
          case (#BadFee kind) {
            let expected_fee = kind.expected_fee;
            return #err({
              message = ?("Bad Fee. Expected fee of " # Nat64.toText(expected_fee.e8s) # " but got " # Nat64.toText(args.fee.e8s));
              kind = #BadFee({ expected_fee });
            });
          };
          case (#InsufficientFunds kind) {
            let balance = kind.balance;
            return #err({
              message = ?("Insufficient balance. Current balance is " # Nat64.toText(balance.e8s));
              kind = #InsufficientFunds({ balance });
            });
          };
          case (#TxTooOld kind) {
            let allowed_window_nanos = kind.allowed_window_nanos;
            return #err({
              message = ?("Error - Tx Too Old. Allowed window of " # Nat64.toText(allowed_window_nanos));
              kind = #TxTooOld({ allowed_window_nanos });
            });
          };
          case (#TxCreatedInFuture) {
            return #err({
              message = ?"Error - Tx Created In Future";
              kind = #TxCreatedInFuture;
            });
          };
          case (#TxDuplicate kind) {
            let duplicate_of = kind.duplicate_of;
            return #err({
              message = ?("Error - Duplicate transaction. Duplicate of " # Nat64.toText(duplicate_of));
              kind = #TxDuplicate({ duplicate_of });
            });
          };
        };
      };
    };
  };

  type AccountArgs = {
    // Hex-encoded AccountIdentifier
    account : Text;
  };
  type BalanceResult = Result.Result<BalanceSuccess, BalanceError>;

  type BalanceSuccess = {
    balance : Nat;
  };
  type BalanceError = {
    message : ?Text;
    kind : {
      #InvalidToken;
      #InvalidAccount;
      #NotFound;
      #Other;
    };
  };
  public func balance(args : AccountArgs) : async BalanceResult {
    switch (ICPUtils.accountIdentifierFromValidText(args.account)) {
      case (#err err) {
        return #err({
          kind = #InvalidAccount;
          message = ?"Invalid account";
        });
      };
      case (#ok account) {
        let balance = await Ledger.account_balance({ account });
        return #ok({
          balance = Nat64.toNat(balance.e8s);
        });
      };
    };
  };

  public type ICPVerifyInvoiceArgs = {
    invoice : T.Invoice;
    caller : Principal;
    canisterId : Principal;
  };

  public func verifyInvoice(args : ICPVerifyInvoiceArgs) : async T.VerifyInvoiceResult {
    let i = args.invoice;

    // is going to be "fixed" the commit after this one
    let invoiceSubaccountAddress = switch(i.destination) { case (#text(identifier)) { identifier }; case _ { "" /* not needed atm just for showing equivalence while refactoring*/ }; };

    let balanceResult = await balance({ account = invoiceSubaccountAddress });
    switch (balanceResult) {
      case (#ok b) {
        let currentInvoiceSubaccountBalance = b.balance;
        if (currentInvoiceSubaccountBalance < i.amount) {
          return #err({
            message = ?("Insufficient balance. Current Balance is " # Nat.toText(currentInvoiceSubaccountBalance));
            kind = #NotYetPaid;
          });          
        };
        let verifiedAtTime : ?Time.Time = ?Time.now();
        let transferResult = await transfer({
          memo = 0;
          fee = { e8s = 10000 };
          amount = { // Total amount, minus the fee
            e8s = Nat64.sub(Nat64.fromNat(currentInvoiceSubaccountBalance), 10000);
          };
          from_subaccount = ?ICPUtils.subaccountForInvoice(i.id, i.creator);
          to = ICPUtils.toAccountIdentifierAddress(
            args.canisterId,
            ICPUtils.subaccountForPrincipal(i.creator)
          );
          created_at_time = null; // use Nat64.fromIntWrap(Time.now()); ?
        });
        switch (transferResult) {
          case (#ok result) {
            let verifiedInvoice = {
              id = i.id;
              creator = i.creator;
              details = i.details;
              permissions = i.permissions;
              amount = i.amount;
              // update amountPaid
              amountPaid = currentInvoiceSubaccountBalance;
              token = i.token;
              // update verifiedAtTime
              verifiedAtTime;
              // update paid
              paid = true; // since transfer has succeeded
              destination = i.destination;
            };
            return #ok(#Paid({ invoice = verifiedInvoice; }));
          };
          case (#err err) {
            switch (err.kind) {
              case (#BadFee f) { return #err({ message = ?"Bad fee"; kind = #TransferError }) };
              case (#InsufficientFunds f) { return #err({ message = ?"Insufficient funds"; kind = #TransferError }) };
              case (_) { return #err({ message = ?"Could not transfer funds to invoice creator."; kind = #TransferError }) };
            };
          };
        };
      };
      case (#err err) { return #err(err) };
    };
  };
};
