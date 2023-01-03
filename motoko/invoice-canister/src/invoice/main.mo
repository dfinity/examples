import ICP "./ICPLedger";
import ICPUtils "./ICPUtils";
import T "./Types";

import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Hash "mo:base/Hash";
import HashMap "mo:base/HashMap";
import Iter "mo:base/Iter";
import Nat "mo:base/Nat";
import Nat64 "mo:base/Nat64";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Time "mo:base/Time";

actor Invoice {
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
  let invoices : HashMap.HashMap<Nat, Invoice> = HashMap.fromIter(Iter.fromArray(entries), entries.size(), Nat.equal, Hash.hash);
  entries := [];

  // Magic Numbers
  let SMALL_CONTENT_SIZE = 256;
  let LARGE_CONTENT_SIZE = 32_000;
  let MAX_INVOICES = 30_000;
  let MINIMUM_BILLABLE_AMOUNT = 2 * 10_000;

  // #endregion
  /**
* Application Interface
*/

  // #region Create Invoice
  public shared ({ caller }) func create_invoice(args : T.CreateInvoiceArgs) : async T.CreateInvoiceResult {

    if (invoiceCounter >= MAX_INVOICES) {
      return #err({
        message = ?"The maximum number of invoices has been reached.";
        kind = #MaxInvoicesReached;
      });
    };

    // confirm the specified token is not invalid
    switch (args.token.symbol) {
      case "ICP" { /* proceed */ };
      case _ { return errInvalidToken };
    };

    if (args.amount < MINIMUM_BILLABLE_AMOUNT) {
      return #err({
        message = ?"The amount is less than what is required to internally transfer funds if the invoice is successfully verified.";
        kind = #InvalidAmount;
      });
    };

    let inputsValid = areInputsValid(args);
    if (not inputsValid) {
      return #err({
        message = ?"Bad size: one or more of your inputs exceeds the allowed size.";
        kind = #BadSize;
      });
    };

    let id : Nat = invoiceCounter;
    // increment counter
    invoiceCounter += 1;

    let paymentAddress = switch (args.token.symbol) {
      case "ICP" { 
          ICPUtils.toHumanReadableForm(
            ICPUtils.toAccountIdentifierAddress(
            getInvoiceCanisterPrincipal(),
            ICPUtils.subaccountForInvoice(id, caller)
          )
        )
      };
      case _ { assert(false); /* already checked this case above */ "";  };
    };

    let invoice : Invoice = {
      id;
      creator = caller;
      details = args.details;
      permissions = args.permissions;
      amount = args.amount;
      amountPaid = 0;
      token = getTokenVerbose(args.token);
      verifiedAtTime = null;
      paid = false;
      //paymentAddress;
      destination = #text(paymentAddress)
    };

    invoices.put(id, invoice);
    return #ok({ invoice });
  };

  func areInputsValid(args : T.CreateInvoiceArgs) : Bool {
    var isValid = true;

    switch (args.details) {
      case null {};
      case (?details) {
        if (details.meta.size() > LARGE_CONTENT_SIZE) {
          isValid := false;
        };
        if (details.description.size() > SMALL_CONTENT_SIZE) {
          isValid := false;
        };
      };
    };

    switch (args.permissions) {
      case null {};
      case (?permissions) {
        if (permissions.canGet.size() > SMALL_CONTENT_SIZE or permissions.canVerify.size() > SMALL_CONTENT_SIZE) {
          isValid := false;
        };
      };
    };

    return isValid;
  };
  // #endregion

  // #region Get Invoice
  public shared query ({ caller }) func get_invoice(args : T.GetInvoiceArgs) : async T.GetInvoiceResult {
    let invoice = invoices.get(args.id);
    switch invoice {
      case null {
        #err({
          message = ?"Invoice not found";
          kind = #NotFound;
        });
      };
      case (?i) {
        if (i.creator == caller) {
          return #ok({ invoice = i });
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
              func(x : Principal) : Bool {
                return x == caller;
              },
            );
            if (Option.isSome(hasPermission)) {
              return #ok({ invoice = i });
            } else {
              return #err({
                message = ?"You do not have permission to view this invoice";
                kind = #NotAuthorized;
              });
            };
          };
        };
        #ok({ invoice = i });
      };
    };
  };
  // #endregion

  // #region Get Balance
  public shared ({ caller }) func get_balance(args : T.GetBalanceArgs) : async T.GetBalanceResult {
    let token = args.token;
    switch (token.symbol) {
      case "ICP" {
        // necessary to pass as text? 
        let subaccountAddress = ICPUtils.toHumanReadableForm(
          ICPUtils.toAccountIdentifierAddress(
            getInvoiceCanisterPrincipal(),
            ICPUtils.subaccountForPrincipal(caller)
          )
        );
        let balance = await ICP.balance({ account = subaccountAddress });
        switch (balance) {
          case (#err err) {
            return #err({
              message = ?"Could not get balance";
              kind = #NotFound;
            });
          };
          case (#ok result) { return #ok({ balance = result.balance }) };
        };
      };
      case _ { return errInvalidToken };
    };
  };
  // #endregion

  // #region Verify Invoice
  public shared ({ caller }) func verify_invoice(args : T.VerifyInvoiceArgs) : async T.VerifyInvoiceResult {
    switch (invoices.get(args.id)) {
      case null {
        return #err({
          message = ?"Invoice not found";
          kind = #NotFound;
        });
      };
      case (?i) {
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
                func(x : Principal) : Bool {
                  return x == caller;
                },
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

        // Return if already verified
        if (i.verifiedAtTime != null) {
          return #ok(
            #AlreadyVerified {
              invoice = i;
            },
          );
        };

        switch (i.token.symbol) {
          case "ICP" {
            let result : T.VerifyInvoiceResult = await ICP.verifyInvoice({
              invoice = i;
              caller;
              canisterId = getInvoiceCanisterPrincipal();
            });
            switch result {
              case (#ok value) {
                switch (value) {
                  case (#AlreadyVerified _) {};
                  case (#Paid paidResult) {
                    let replaced = invoices.replace(i.id, paidResult.invoice);
                  };
                };
              };
              case (#err _) {};
            };
            return result;
          };
          case _ { return errInvalidToken };
        };
      };
    };
  };
  // #endregion

  // #region Transfer
  public shared ({ caller }) func transfer(args : T.TransferArgs) : async T.TransferResult {
    switch (args.token.symbol) {
      case "ICP" {
        // going to "fix" in commit after this one
        let dest = switch(args.destination) { case (#text(identifier)) { identifier }; case _ { "" /* temporary while refactoring */ }; };
        switch (ICPUtils.accountIdentifierFromValidText(dest)) {
          case (#ok accountIdentifer) {
            let transferResult = await ICP.transfer({
              memo = 0;
              fee = { e8s = 10000 };
              amount = { // Total amount, minus the fee
                e8s = Nat64.sub(Nat64.fromNat(args.amount), 10000);
              };
              from_subaccount = ?ICPUtils.subaccountForPrincipal(caller);
              to = accountIdentifer;
              created_at_time = null; // use Nat64.fromIntWrap(Time.now()); ?
            });

            switch (transferResult) {
              case (#ok result) { return #ok(result) };
              case (#err err) {
                switch (err.kind) {
                  case (#BadFee _) { 
                    return #err({ message = err.message; kind = #BadFee; }); 
                  };
                  case (#InsufficientFunds _) { 
                    return #err({ message = err.message; kind = #InsufficientFunds; }); 
                  };
                  case _ { 
                    return #err({ message = err.message; kind = #Other; }); 
                  }; 
                };
              };
            };
          }; 
          case (#err err) { 
            return #err({
              message = ?"Invalid account identifier";
              kind = #InvalidDestination;
            });
          };
        };
      };
      case _ { return errInvalidToken };
    };
  };
  // #endregion

  // #region get_account_identifier
  /*
    * Get Caller Identifier
    * Allows a caller to the accountIdentifier for a given principal
    * for a specific token.
    */
  public query func get_account_identifier(args : T.GetAccountIdentifierArgs) : async T.GetAccountIdentifierResult {
    switch (args.token.symbol) {
      case "ICP" {
        let principalSubaccountAddress = ICPUtils.toHumanReadableForm(
          ICPUtils.toAccountIdentifierAddress(
            getInvoiceCanisterPrincipal(),
            ICPUtils.subaccountForPrincipal(args.principal)
          )
        );
        return #ok({ accountIdentifier = #text(principalSubaccountAddress) });
      };
      case _ { return errInvalidToken };
    };
  };
  // #endregion

  // #region Utils
  public func accountIdentifierToBlob(accountIdentifier : AccountIdentifier) : async T.AccountIdentifierToBlobResult {
    switch (accountIdentifier) {
      case (#text textAccountIdentifier) {
        switch (ICPUtils.accountIdentifierFromValidText(textAccountIdentifier)) {
          case (#ok blob) { return #ok(blob) };
          case (#err msg) { return #err({ message = ?"Invalid account identifier"; kind = #InvalidAccountIdentifier }) };
        };
      };
      case (#blob blobAccountIdentifier) {
        if (ICPUtils.accountIdentifierIsValid(blobAccountIdentifier)) {
          return #ok(blobAccountIdentifier); 
        } else {
          return #err({ message = ?"Invalid account identifier"; kind = #InvalidAccountIdentifier });
        };
      };
      case (#principal principalAccountIdentifier) {
        let blob = ICPUtils.toAccountIdentifierAddress(
            getInvoiceCanisterPrincipal(),
            ICPUtils.subaccountForPrincipal(principalAccountIdentifier)
        );
        return #ok(blob);
      };
    };
  };

  func getTokenVerbose(token : Token) : TokenVerbose {
    switch (token.symbol) {
      case ("ICP") {
        {
          symbol = "ICP";
          decimals = 8;
          meta = ?{
            Issuer = "e8s";
          };
        };
      };
      case (_) {
        {
          symbol = "";
          decimals = 1;
          meta = ?{
            Issuer = "";
          };
        };
      };
    };
  };

  func getInvoiceCanisterPrincipal(): Principal { Principal.fromActor(Invoice) };
  // #endregion

  // #region Upgrade Hooks
  system func preupgrade() {
    entries := Iter.toArray(invoices.entries());
  };
  // #endregion
};
