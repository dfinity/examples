import Ledger     "canister:ledger";

import A          "./Account";
import CRC32      "./CRC32";
import Hex        "./Hex";
import SHA224     "./SHA224";
import T          "./Types";
import U          "./Utils";

import Blob       "mo:base/Blob";
import Nat        "mo:base/Nat";
import Nat64      "mo:base/Nat64";
import Principal  "mo:base/Principal";
import Result     "mo:base/Result";
import Time       "mo:base/Time";

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
    }
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

  public func transfer (args : TransferArgs) : async TransferResult {
    let result = await Ledger.transfer(args);
    switch result {
      case (#Ok index) {
        #ok({blockHeight = index});
      };
      case (#Err err) {
        switch err {
          case (#BadFee kind) {
            let expected_fee = kind.expected_fee;
            #err({
              message = ?("Bad Fee. Expected fee of " # Nat64.toText(expected_fee.e8s) # " but got " # Nat64.toText(args.fee.e8s));
              kind = #BadFee({expected_fee});
            });
          };
          case (#InsufficientFunds kind) {
            let balance = kind.balance;
            #err({
              message = ?("Insufficient balance. Current balance is " # Nat64.toText(balance.e8s));
              kind = #InsufficientFunds({balance});
            })
          };
          case (#TxTooOld kind) {
            let allowed_window_nanos = kind.allowed_window_nanos;
            #err({
              message = ?("Error - Tx Too Old. Allowed window of " # Nat64.toText(allowed_window_nanos));
              kind = #TxTooOld({allowed_window_nanos});
            })
          };
          case (#TxCreatedInFuture) {
            #err({
              message = ?"Error - Tx Created In Future";
              kind = #TxCreatedInFuture;
            })
          };
          case (#TxDuplicate kind) {
            let duplicate_of = kind.duplicate_of;
            #err({
              message = ?("Error - Duplicate transaction. Duplicate of " # Nat64.toText(duplicate_of));
              kind = #TxDuplicate({duplicate_of});
            })
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
    switch (Hex.decode(args.account)) {
      case (#err err) {
        #err({
          kind = #InvalidAccount;
          message = ?"Invalid account";
        });
      };
      case (#ok account) {
        let balance = await Ledger.account_balance({account = Blob.fromArray(account)});
        #ok({
          balance = Nat64.toNat(balance.e8s);
        });
      };
    };
  };

  public type GetICPAccountIdentifierArgs = {
    principal : Principal;
    subaccount : SubAccount;
  };
  public func getICPAccountIdentifier(args : GetICPAccountIdentifierArgs) : Blob {
    A.accountIdentifier(args.principal, args.subaccount);
  };

  public type ICPVerifyInvoiceArgs = {
    invoice : T.Invoice;
    caller : Principal;
    canisterId : Principal;
  };
  public func verifyInvoice(args : ICPVerifyInvoiceArgs) : async T.VerifyInvoiceResult {
    let i = args.invoice;
    let destinationResult = U.accountIdentifierToText({
      accountIdentifier = i.destination;
      canisterId = ?args.canisterId;
    });
    switch destinationResult {
      case (#err err) {
        #err({
          kind = #InvalidAccount;
          message = ?"Invalid destination account";
        });
      };
      case (#ok destination) {
        let balanceResult = await balance({account = destination});
        switch balanceResult {
          case (#err err) {
            #err(err);
          };
          case (#ok b) {
            let balance = b.balance;
            // If balance is less than invoice amount, return error
            if (balance < i.amount) {
              return #err({
                message = ?("Insufficient balance. Current Balance is " # Nat.toText(balance));
                kind = #NotYetPaid;
              });
            };

            let verifiedAtTime : ?Time.Time = ?Time.now();

            // TODO Transfer funds to default subaccount of invoice creator
            let subaccount : SubAccount = U.generateInvoiceSubaccount({ caller = i.creator; id = i.id });

            let transferResult = await transfer({
              memo = 0;
              fee = {
                e8s = 10000;
              };
              amount = {
                // Total amount, minus the fee
                e8s = Nat64.sub(Nat64.fromNat(balance), 10000);
              };
              from_subaccount = ?subaccount;
              to = U.getDefaultAccount({
                canisterId = args.canisterId;
                principal = i.creator;
              });
              created_at_time = null;
            });
            switch transferResult {
              case (#ok result) {
                let verifiedInvoice = {
                  id = i.id;
                  creator = i.creator;
                  details = i.details;
                  permissions = i.permissions;
                  amount = i.amount;
                  // update amountPaid
                  amountPaid = balance;
                  token = i.token;
                  // update verifiedAtTime
                  verifiedAtTime;
                  // update paid
                  paid = true; // since transfer has succeeded
                  destination = i.destination;
                };

                #ok(#Paid {
                  invoice = verifiedInvoice;
                });
              };
              case (#err err) {
                switch (err.kind) {
                  case (#BadFee f) {
                    #err({
                      message = ?"Bad fee";
                      kind = #TransferError;
                    });
                  };
                  case (#InsufficientFunds f) {
                    #err({
                      message = ?"Insufficient funds";
                      kind = #TransferError;
                    });
                  };
                  case _ {
                    #err({
                      message = ?"Could not transfer funds to invoice creator.";
                      kind = #TransferError;
                    });
                  }
                };
              };
            };
          };
        };
      }
    };
  };
}
