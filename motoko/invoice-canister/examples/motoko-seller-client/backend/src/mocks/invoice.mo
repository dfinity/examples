import A          "../../../../../src/invoice/Account";   
import Hex        "../../../../../src/invoice/Hex";
import T          "../../../../../src/invoice/Types";
import U          "../../../../../src/invoice/Utils";

import Array      "mo:base/Array";
import Blob       "mo:base/Blob";
import Debug      "mo:base/Debug";
import Hash       "mo:base/Hash";
import HashMap    "mo:base/HashMap";
import Iter       "mo:base/Iter";
import Nat        "mo:base/Nat";
import Nat64      "mo:base/Nat64";
import Option     "mo:base/Option";
import Principal  "mo:base/Principal";
import Result     "mo:base/Result";
import Text       "mo:base/Text";
import Time       "mo:base/Time";

actor InvoiceMock {
// #region Types
  type Details = T.Details;
  type Token = T.Token;
  type TokenVerbose = T.TokenVerbose;
  type AccountIdentifier = T.AccountIdentifier;
  type Invoice = T.Invoice;
// #endregion

  let errInvalidToken = #err({
    message = ?"This token is not yet supported. Currently, this canister supports ICP.";
    kind = #InvalidToken;
  });

/**
* Application State
*/

// #region State
  stable var entries : [(Nat, Invoice)] = [];
  stable var invoiceCounter : Nat = 0;
  let invoices: HashMap.HashMap<Nat, Invoice> = HashMap.fromIter(Iter.fromArray(entries), entries.size(), Nat.equal, Hash.hash);

  var icpBlockHeight : Nat = 0;
  var icpLedgerMock : HashMap.HashMap<Blob, Nat> = HashMap.HashMap(16, Blob.equal, Blob.hash);
  let MAX_INVOICES = 30_000;
// #endregion

/**
* Application Interface
*/    

// #region Create Invoice
  public shared ({caller}) func create_invoice (args: T.CreateInvoiceArgs) : async T.CreateInvoiceResult {
    let id : Nat = invoiceCounter;
    // increment counter
    invoiceCounter += 1;
    let inputsValid = areInputsValid(args);
    if(not inputsValid) {
      return #err({
        message = ?"Bad size: one or more of your inputs exceeds the allowed size.";
        kind = #BadSize;
      });
    };

    if(id > MAX_INVOICES){
      return #err({
        message = ?"The maximum number of invoices has been reached.";
        kind = #MaxInvoicesReached;
      });
    };

    let destinationResult : T.GetDestinationAccountIdentifierResult = getDestinationAccountIdentifier({ 
      token = args.token;
      invoiceId = id;
      caller 
    });

    switch(destinationResult){
      case (#err result) {
        return #err({
          message = ?"Invalid destination account identifier";
          kind = #InvalidDestination;
        });
      };
      case (#ok result) {
        let destination : AccountIdentifier = result.accountIdentifier;
        let token = getTokenVerbose(args.token);

        let invoice : Invoice = {
          id;
          creator = caller;
          details = args.details;
          permissions = args.permissions;
          amount = args.amount;
          amountPaid = 0;
          token;
          verifiedAtTime = null;
          paid = false;
          destination;
        };
    
        invoices.put(id, invoice);

        return #ok({invoice});
      };
    };
  };

  func getTokenVerbose(token: Token) : TokenVerbose { 
    switch(token.symbol){
      case ("ICP") {
        return {
          symbol = "ICP";
          decimals = 8;
          meta = ?{
            Issuer = "e8s";
          }
        };

      };
      case (_) {
        return {
          symbol = "";
          decimals = 1;
          meta = ?{
            Issuer = "";
          }
        }
      };
    };
  };

  func areInputsValid(args : T.CreateInvoiceArgs) : Bool {
    let token = getTokenVerbose(args.token);

    var isValid = true;

    switch (args.details){
      case null {};
      case (? details){
        if (details.meta.size() > 32_000) {
          isValid := false;
        };
        if (details.description.size() > 256) {
          isValid := false;
        };
      };
    };

    switch (args.permissions){
      case null {};
      case (? permissions){
        if (permissions.canGet.size() > 256 or permissions.canVerify.size() > 256) {
          isValid := false;
        };
      };
    };

    return isValid;
  };

// #region Get Destination Account Identifier
  func getDestinationAccountIdentifier (args: T.GetDestinationAccountIdentifierArgs) : T.GetDestinationAccountIdentifierResult {
    let token = args.token;
    switch(token.symbol){
      case("ICP"){
        let canisterId = Principal.fromActor(InvoiceMock);

        let account = U.getICPAccountIdentifier({
          principal = canisterId;
          subaccount = U.generateInvoiceSubaccount({ 
            caller = args.caller;
            id = args.invoiceId;
          });
        });
        let hexEncoded = Hex.encode(Blob.toArray(account));
        let result: AccountIdentifier = #text(hexEncoded);
        return #ok({accountIdentifier = result});
      };
      case(_){
        return errInvalidToken;
      };
    };
  };
// #endregion
// #endregion

// #region Get Invoice
  public shared query ({caller}) func get_invoice (args: T.GetInvoiceArgs) : async T.GetInvoiceResult {
    let invoice = invoices.get(args.id);
    switch(invoice){
      case(null){
        return #err({
          message = ?"Invoice not found";
          kind = #NotFound;
        });
      };
      case (?i) {
        if (i.creator == caller) {
          return #ok({invoice = i});
        };
        // If additional permissions are provided
        switch (i.permissions) {
          case (null) {
            return #err({
              message = ?"You do not have permission to view this invoice";
              kind = #NotAuthorized;
            });
          };
          case (?permissions) {
            let hasPermission = Array.find<Principal>(
              permissions.canGet,
              func (x : Principal) : Bool {
                return x == caller;
              }
            );
            if (Option.isSome(hasPermission)) {
              return #ok({invoice = i});
            } else {
              return #err({
                message = ?"You do not have permission to view this invoice";
                kind = #NotAuthorized;
              });
            };
          };
        };
        #ok({invoice = i});
      };
    };
  };
// #endregion

// #region Get Balance
  public shared ({caller}) func get_balance (args: T.GetBalanceArgs) : async T.GetBalanceResult {
    let token = args.token;
    let canisterId = Principal.fromActor(InvoiceMock);
    switch(token.symbol){
      case("ICP"){
        let defaultAccount = U.getDefaultAccount({
          canisterId;
          principal = caller;
        });
        let balance = icpLedgerMock.get(defaultAccount);
        switch(balance){
          case(null){
            return #err({
              message = ?"Could not get balance";
              kind = #NotFound;
            });
          };
          case(? b){
            return #ok({balance = b});
          };
        };
      };
      case(_){
        return errInvalidToken;
      };
    };
  };
// #endregion

// #region Verify Invoice
  public shared ({caller}) func verify_invoice (args: T.VerifyInvoiceArgs) : async T.VerifyInvoiceResult {
    let invoice = invoices.get(args.id);
    let canisterId = Principal.fromActor(InvoiceMock);

    switch(invoice){
      case(null){
        return #err({
          message = ?"Invoice not found";
          kind = #NotFound;
        });
      };
      case(? i){
        // Return if already verified
        if (i.verifiedAtTime != null){
          return #ok(#AlreadyVerified{
            invoice = i;
          });
        };

        if (i.creator != caller) {
          switch (i.permissions) {
            case null {
              return #err({
                message = ?"You do not have permission to verify this invoice";
                kind = #NotAuthorized;
              });
            };
            case (?permissions) {
              let hasPermission = Array.find<Principal>(
                permissions.canVerify,
                func (x : Principal) : Bool {
                  return x == caller;
                }
              );
              if (Option.isSome(hasPermission)) {
                // May proceed
              } else {
                return #err({
                  message = ?"You do not have permission to verify this invoice";
                  kind = #NotAuthorized;
                });
              };
            };
          };
        };

        switch (i.token.symbol){
          case("ICP"){
            let destinationResult = U.accountIdentifierToBlob({
              accountIdentifier = i.destination;
              canisterId = ?canisterId;
              });
            switch(destinationResult){
              case(#err err){
                return #err({
                  kind = #InvalidAccount;
                  message = ?"Invalid destination account";
                });
              };
              case (#ok destination){
                let destinationBalance = icpLedgerMock.get(destination);
                switch(destinationBalance){
                  case (null){
                    return #err({
                      message = ?"Insufficient balance. Current Balance is 0";
                      kind = #NotYetPaid;
                    });
                  };
                  case (? balance){
                    if(balance < i.amount){
                      return #err({
                        message = ?Text.concat("Insufficient balance. Current Balance is ", Nat.toText(balance));
                        kind = #NotYetPaid;
                      });
                    };
                    let verifiedAtTime: ?Time.Time = ?Time.now();
                    // Otherwise, update with latest balance and mark as paid
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
                      paid = true;
                      destination = i.destination;
                    };

                    // TODO Transfer funds to default subaccount of invoice creator
                    let subaccount = U.generateInvoiceSubaccount({ caller = i.creator; id = i.id });
                    let transfer = await mockICPTransfer({
                      amount = {
                        e8s = Nat64.fromNat(balance - 10_000);
                      };
                      fee = {
                        e8s = 10_000;
                      };
                      memo = 0;
                      from_subaccount = ?subaccount;
                      to = #blob(U.getDefaultAccount({
                        canisterId;
                        principal = i.creator;
                      }));
                      token = i.token;
                      canisterId = ?canisterId;
                      created_at_time = null;
                    });
                    let replaced = invoices.replace(i.id, verifiedInvoice);
                    return #ok(#Paid({
                      invoice = verifiedInvoice;
                    }));
                  };
                };

              };
            }
          };
          case(_){
            return errInvalidToken;
          };
        };
      };
    };
  };
// #endregion

// #region Transfer
  public shared ({caller}) func transfer (args: T.TransferArgs) : async T.TransferResult {
    let token = args.token;
    let accountResult = U.accountIdentifierToBlob({
      accountIdentifier = args.destination;
      canisterId = ?Principal.fromActor(InvoiceMock);
    });
    switch (accountResult){
      case (#err err){
        return #err({
          message = err.message;
          kind = #InvalidDestination;
        });
      };
      case (#ok destination){
        switch(token.symbol){
          case("ICP"){
            let now = Nat64.fromIntWrap(Time.now());
            

            let transferResult = await mockICPTransfer({
              memo = 0;
              fee = {
                e8s = 10000;
              };
              amount = {
                // Total amount, minus the fee
                e8s = Nat64.sub(Nat64.fromNat(args.amount), 10000);
              };
              from_subaccount = ?A.principalToSubaccount(caller);

              to = #blob(destination);
              created_at_time = null;
            });
            switch (transferResult) {
              case (#ok result) {
                return #ok(result);
              };
              case (#err err) {
                switch (err.kind){
                  case (#BadFee _){
                    return #err({
                      message = err.message;
                      kind = #BadFee;
                    });
                  };
                  case (#InsufficientFunds _){
                    return #err({
                      message = err.message;
                      kind = #InsufficientFunds;
                    });
                  };
                  case (_){
                    return #err({
                      message = err.message;
                      kind = #Other;
                    });
                  }
                };
              };
            };
          };
          case(_){
            return #err({
              message = ?"Token not supported";
              kind = #InvalidToken;
            });
          };
        };
      };
    };
  };
// #endregion

// #region get_account_identifier
  /*
    * Get Caller Identifier
    * Allows a caller to the accountIdentifier for a given principal
    * for a specific token.
    */
  public query func get_account_identifier (args: T.GetAccountIdentifierArgs) : async T.GetAccountIdentifierResult {
    let token = args.token;
    let principal = args.principal;
    let canisterId = Principal.fromActor(InvoiceMock);
    switch(token.symbol){
      case("ICP"){
        let subaccount = U.getDefaultAccount({principal; canisterId;});
        let hexEncoded = Hex.encode(
          Blob.toArray(subaccount)
        );
        let result: AccountIdentifier = #text(hexEncoded);
        return #ok({accountIdentifier = result});
      };
      case(_){
        return errInvalidToken;
      };
    };
  };
// #endregion

// #region Utils
  public func accountIdentifierToBlob (accountIdentifier: AccountIdentifier) : async T.AccountIdentifierToBlobResult {
    return U.accountIdentifierToBlob({
      accountIdentifier;
      canisterId = ?Principal.fromActor(InvoiceMock);
    });
  };
  
  func mockICPTransfer (args: T.ICPTransferArgs) : async T.ICPTransferResult {
    let FEE = 10_000;
    let amount = args.amount;
    switch(args.from_subaccount){
      case(? subaccount){
        let fromAccount = U.getICPAccountIdentifier({
          subaccount;
          principal = Principal.fromActor(InvoiceMock);
        });
        let balance = icpLedgerMock.get(fromAccount);
        switch(balance){
          case(? b){
            if(b < Nat64.toNat(amount.e8s) + FEE){
              Debug.trap("InsufficientFunds");
            };
            let newBalance = Nat.sub(Nat.sub(b, Nat64.toNat(amount.e8s)), FEE);
            icpLedgerMock.put(fromAccount, newBalance);
            
            let destinationResult = U.accountIdentifierToBlob({
              accountIdentifier = args.to;
              canisterId = ?Principal.fromActor(InvoiceMock);
            });
            switch(destinationResult){
              case(#err err){
                switch(err.message){
                  case (null){
                    Debug.trap("InvalidDestination");
                  };
                  case(? message){
                    Debug.trap(message);
                  };
                }
              };
              case (#ok destination){
                let destinationBalance = icpLedgerMock.get(destination);
                let newDestinationBalance = newBalance + Nat64.toNat(amount.e8s);
                icpLedgerMock.put(destination, newDestinationBalance);
                icpBlockHeight := icpBlockHeight + 1;
                return #ok({
                  blockHeight = Nat64.fromNat(icpBlockHeight);
                });

              };
            };
          };
          case(_){
            Debug.trap("InsufficientFunds");
          };
        };
      };
      case(null){
        Debug.trap("InvalidSubaccount");
      };
    };
  };

  // Useful for testing
  type FreeMoneyArgs = {
    amount: Nat;
    accountIdentifier: AccountIdentifier;
  };
  type FreeMoneyResult = Result.Result<Nat, FreeMoneyError>;
  type FreeMoneyError = {
    message: ?Text;
    kind: {
      #InvalidDestination;
    };
  };
  public func deposit_free_money (args: FreeMoneyArgs) : async FreeMoneyResult {
    let amount = args.amount;
    let accountBlob = U.accountIdentifierToBlob({
      accountIdentifier = args.accountIdentifier;
      canisterId = ?Principal.fromActor(InvoiceMock);
    });
    
    switch(accountBlob){
      case(#err err){
        return #err({
          message = err.message;
          kind = #InvalidDestination;
        });
      };
      case(#ok account){
        let balanceResult = icpLedgerMock.get(account);
        switch(balanceResult){
          case(null){
            icpLedgerMock.put(account, amount);
            return #ok(amount);
          };
          case (? balance){
            let newBalance = balance + amount;
            icpLedgerMock.put(account, newBalance);
            return #ok(newBalance);
          };
        };
      };
    };
  };
// #endregion

// #region Upgrade Hooks
  system func preupgrade() {
      entries := Iter.toArray(invoices.entries());
  };
// #endregion
}
