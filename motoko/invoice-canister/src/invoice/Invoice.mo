import Array "mo:base/Array";
import Error "mo:base/Error";
import HashMap "mo:base/HashMap";
import Nat64 "mo:base/Nat64";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Text "mo:base/Text";
import Time "mo:base/Time";
import Trie "mo:base/Trie";
import XorShift "mo:rand/XorShift";
import Source "mo:ulid/Source";
import ULID "mo:ulid/ULID";

import SupportedToken "./modules/supported-token/SupportedToken";
import Types "./modules/Types";

shared ({ caller = installer_ }) actor class Invoice() = this {

  /*--------------------------------------------------------------------------- 
  */ // Application State

  /** Compulsory constants this canister must adhere to. */
  module MagicNumbers {
    // Invoice Canister Constraints:
    public let SMALL_CONTENT_SIZE = 256;
    public let LARGE_CONTENT_SIZE = 32_000;
    public let MAX_INVOICES = 30_000;
    public let MAX_INVOICE_CREATORS = 256;
  };

  /** Ids of the token ledger canisters used to create actor supertypes. */
  let CANISTER_IDS = {
    icp_ledger_canister = "sgymv-uiaaa-aaaaa-aaaia-cai";
    nns_ledger = "ryjl3-tyaaa-aaaaa-aaaba-cai";
    icrc1_ledger_canister_ex1 = "sbzkb-zqaaa-aaaaa-aaaiq-cai";
    icrc1_ledger_canister_ex2 = "si2b5-pyaaa-aaaaa-aaaja-cai";
  };

  // Invoice canister only uses transfer and balance methods of ledger canisters; these are those supertypes:
  let Ledger_ICP : SupportedToken.Actor_Supertype_ICP = actor (CANISTER_IDS.icp_ledger_canister);
  let Ledger_ICP_nns : SupportedToken.Actor_Supertype_ICP = actor (CANISTER_IDS.nns_ledger);
  let Ledger_ICRC1_Ex1 : SupportedToken.Actor_Supertype_ICRC1 = actor (CANISTER_IDS.icrc1_ledger_canister_ex1);
  let Ledger_ICRC1_Ex2 : SupportedToken.Actor_Supertype_ICRC1 = actor (CANISTER_IDS.icrc1_ledger_canister_ex2);

  /** Stores `Invoice_` records in a stable trie representation. 
    _Note invoices as they are returned to a caller are first decorated by `toCallerExpectedInvoice`  
    to add the `paid : Bool` and `tokenVerbose : VerboseToken` fields.  */
  stable var invoices_ : Trie.Trie<Types.InvoiceId, Types.Invoice_> = Trie.empty();

  /** Source of entropy for substantiating ULID invoice ids. */
  let idCreationEntropy_ = Source.Source(XorShift.toReader(XorShift.XorShift64(null)), 0);

  /** Monotonic counter used to compute each next created invoice id.  */
  stable var invoiceCounter_ : Nat = 0;

  /** List of principals allowed to create invoices. Principals on this list  
    can also call `get_caller_address`, `get_caller_balance`, `transfer`,  
    `to_other_address_format` and `recover_invoice_subaccount_balance` as  
    well as `create_invoice`, `get_invoice` and `verify_invoice`; the latter 
    three of which require invoice specific access permission.  */
  stable var allowedCreatorsList_ : [Principal] = [];

  /** Lock lookup map to synchronize invoice's verification and subaccount balance  
    recovery by invoice id. To prevent edge cases of lock not being released due to  
    unforeseen bug in this canister's code, if the elapsed time between locking the  
    same invoice id is greater than the `isAlreadyProcessingTimeout_` the lock will  
    automatically be released (see `isAlreadyProcessing_` method below).  
    _Note the tuple with `Principal` is used in case developer would need to inspect  
    who's been calling._  */
  let isAlreadyProcessingLookup_ = HashMap.HashMap<Text, (Time.Time, Principal)>(32, Text.equal, Text.hash);
  let isAlreadyProcessingTimeout_ : Nat = 600_000_000_000; // "10 minutes ns"

  /*--------------------------------------------------------------------------- 
  */ // Private Utility And Helper Methods

  /** The classic canister principal getter.  */
  func getInvoiceCanisterId_() : Principal { Principal.fromActor(this) };

  /** Gets the key used by the trie storing `Invoice_` records.  */
  func key_(t : Text) : Trie.Key<Text> { { hash = Text.hash(t); key = t } };

  /** Checks if a principal is in an array of principals.  */
  func includesPrincipal_(p : Principal, ps : [Principal]) : Bool {
    Option.isSome(Array.find<Principal>(ps, func(x : Principal) : Bool { x == p }));
  };

  /** Checks if a principal is canister's `installer_` or in the `allowedCreatorsList_`.  */
  func hasCallPermission_(caller : Principal) : Bool {
    (caller == installer_) or includesPrincipal_(caller, allowedCreatorsList_);
  };

  /** Returns the opt unwrapped `Invoice_` value from the `invoices_` trie if an invoice  
    exists for a given id (or `#NotFound` if it doesn't) if the caller is authorized; or  
    `#NotAuthorized` otherwise even if the invoice doesn't exist.  
    _Used by `get_invoice`, `verify_invoice` and `recover_invoice_subaccount_balance_`. */
  func getInvoiceIfAuthorized_(
    id : Types.InvoiceId,
    caller : Principal,
    permission : {
      #Get;
      #Verify;
    },
  ) : Result.Result<Types.Invoice_, { kind : { #NotAuthorized; #NotFound } }> {
    switch (Trie.get(invoices_, key_(id), Text.equal)) {
      case null {
        // No invoice exists for given id, determine if caller has authorization.
        if (not hasCallPermission_(caller)) {
          // Caller is not an allowed creator.
          return #err({ kind = #NotAuthorized });
        } else {
          // Caller is an allowed creator.
          #err({ kind = #NotFound });
        };
      };
      case (?invoice) {
        // Invoice exists for given id.
        if (invoice.creator == caller) {
          // Caller is creator, no need to look up permissions.
          return #ok(invoice);
        };
        switch (invoice.permissions) {
          // No permissions so caller cannot be authorized.
          case null return #err({ kind = #NotAuthorized });
          case (?permissions) {
            // Permissions exist, get which permission is being checked.
            let allowed = switch (permission) {
              case (#Get) { permissions.canGet };
              case (#Verify) { permissions.canVerify };
            };
            if (includesPrincipal_(caller, allowed)) {
              // Caller is on that permission list of this invoice.
              return #ok(invoice);
            } else #err({ kind = #NotAuthorized });
          };
        };
      };
    };
  };

  /** Checks whether the invoice of the given id is already in the process of being verified or  
    having its subaccount balance recovered. Automatically removes any lock if enough time has  
    passed between checks for the same id.  */
  func isAlreadingProcessing_(id : Types.InvoiceId, caller : Principal) : Bool {
    switch (isAlreadyProcessingLookup_.get(id)) {
      // No concurrent access of this invoice is taking place.
      case null return false;
      // Parallel access could be occurring, check if enough time
      // has elapsed to automatically release the lock.
      case (?(atTime, who)) {
        if ((Time.now() - atTime) >= isAlreadyProcessingTimeout_) {
          // Enough time has elapsed, remove the lock and let the caller proceed.
          isAlreadyProcessingLookup_.delete(id);
          return false;
        } else {
          // Not enough time has elapsed, let the other caller's processing finish.
          true;
        };
      };
    };
  };

  /** Decorates an `Invoice_` with `paid : Bool` and `tokenVerbose : TokenVerbose`  
    before it is returned to the caller as an `Invoice` type record.  */
  func toCallerExpectedInvoice_(invoice : Types.Invoice_) : Types.Invoice {
    {
      invoice with paid = Option.isSome(invoice.verifiedPaidAtTime);
      tokenVerbose = SupportedToken.getTokenVerbose(invoice.token);
    };
  };

  /*--------------------------------------------------------------------------- 
  */ //  Application Programming Interface

  /****Creates an invoice if inputs are acceptable & caller is authorized.**  
    **Be aware** if the details and description are not encrypted prior to the call to  
    create an invoice with them, they could be visible to physical inspection by  
    node operators; if privacy is a concern, consider implementing a cryptographic  
    scheme (see developer docs for more details) for the creation of invoices.  
    _Only authorized for canister's `installer_` and those on allowed creators list._  */
  public shared ({ caller }) func create_invoice(
    { details; permissions; tokenAmount } : Types.CreateInvoiceArgs,
  ) : async Types.CreateInvoiceResult {
    // Confirm the caller is an allowed invoice creator.
    if (not hasCallPermission_(caller)) {
      return #err({ kind = #NotAuthorized });
    };
    // Confirm there's still space to create another invoice.
    if (invoiceCounter_ >= MagicNumbers.MAX_INVOICES) {
      return #err({ kind = #MaxInvoicesCreated });
    };
    // Confirm if any passed-in meta and/or description are within the capacity limit.
    switch details {
      // No details included.
      case null {};
      // Details included, check for a meta blob and/or description to include.
      case (?details_) {
        if (details_.meta.size() > MagicNumbers.LARGE_CONTENT_SIZE) {
          // Included meta blob has too many bytes.
          return #err({ kind = #MetaTooLarge });
        };
        if (details_.description.size() > MagicNumbers.SMALL_CONTENT_SIZE) {
          // Included description text has too many characters.
          return #err({ kind = #DescriptionTooLarge });
        };
      };
    };
    // Confirm both invoice specific permission types are within capacity limit.
    switch permissions {
      // No permissions included.
      case null {};
      // Permissions included, check for a list of principals to give get or verify access to.
      case (?permissions_) {
        // Check if there are too many principals on the get permissions list.
        let g = (permissions_.canGet.size() > MagicNumbers.SMALL_CONTENT_SIZE);
        // Check if there are too many principals on the verify permissions list.
        let v = (permissions_.canVerify.size() > MagicNumbers.SMALL_CONTENT_SIZE);
        if (g or v) {
          // There were too many principals on one of the permissions list.
          return #err({ kind = #TooManyPermissions });
        };
      };
    };
    // The invoice's given amount due is either of the ICP form ({ e8s : Nat64 }) or ICRC1 form (Nat);
    // so unwrap these values into normalized Nat base units & corresponding SupportedToken variant tag.
    let (token, amount) = SupportedToken.unwrapTokenAmount(tokenAmount);
    // Verify the amount due is at least enough to cover the transfer of any proceeds from the
    // invoice's subaccount to the subaccount of its creator upon successful verification of payment.
    if (amount < (2 * SupportedToken.getTransactionFee(token))) {
      // Ie if the invoice uses ICP, the given amount due was less than 10000 e8s.
      return #err({ kind = #InsufficientAmountDue });
    };
    // Caller provided acceptable create_invoice input args! Proceed with invoice creation...
    // Substantiate the invoice's ULID.
    let id = ULID.toText(idCreationEntropy_.new());
    // Get the human readable format of address for payment this invoice will use.
    let paymentAddress : Text = SupportedToken.getEncodedInvoiceSubaccountAddress({
      token;
      id;
      creator = caller;
      canisterId = getInvoiceCanisterId_();
    });
    // Create the `invoice_` record to be stored, this type is used internally only; the
    // type returned to the caller ('Invoice') always includes the `paid : Bool` and
    // `tokenVerbose : TokenVerbose` fields as well.
    let createdInvoice : Types.Invoice_ = {
      token;
      id;
      creator = caller;
      details = details;
      permissions = permissions;
      paymentAddress;
      amountDue = amount;
      amountPaid = 0;
      verifiedPaidAtTime = null;
    };
    // Increment the invoice creation counter.
    invoiceCounter_ += 1;
    // Sets the trie storing invoice records updated with the newly created invoice.
    invoices_ := Trie.put(
      invoices_,
      key_(id),
      Text.equal,
      createdInvoice,
    ).0;
    // Return the newly created invoice in the form the caller is expecting.
    #ok({ invoice = toCallerExpectedInvoice_(createdInvoice) });
  };

  /****Adds a principal to the `allowedCreatorsList_`.**  
    Principals added will also be able to call the following methods:  
    `get_caller_balance`, `get_caller_address`, `transfer` as well as  
    `get_invoice`, `verify_invoice` and `to_other_address_format`.  
    _Only authorized for canister's installer._  */
  public shared ({ caller }) func add_allowed_creator(
    { who } : Types.AddAllowedCreatorArgs,
  ) : async Types.AddAllowedCreatorResult {
    // Check if there are too many allowed creators already.
    if (allowedCreatorsList_.size() < MagicNumbers.MAX_INVOICE_CREATORS) {
      // There's still room.
      // Check the caller is in the original canister installer.
      if (caller == installer_) {
        if (Principal.isAnonymous(who)) {
          // Caller tried adding the anonymous principal to the allowed creators list.
          return #err({ kind = #AnonymousIneligible });
        };
        // Check the principal to add is not already on the list.
        if (not includesPrincipal_(who, allowedCreatorsList_)) {
          // Principal to add is acceptable to be added, add that principal to the list.
          allowedCreatorsList_ := Array.flatten<Principal>([allowedCreatorsList_, [who]]);
          return #ok({
            message = "Successfully added " # Principal.toText(who) # " to creators allowed list.";
          });
        } else {
          // Principal to add already on the list.
          #err({ kind = #AlreadyAdded });
        };
        // Caller is not an allowed creator.
      } else #err({ kind = #NotAuthorized });
    } else {
      // There were too many already.
      #err({ kind = #MaxAllowed });
    };
  };

  /****Removes a principal from the `allowedCreatorsList_`.**  
    _Only authorized for canister's installer._  */
  public shared ({ caller }) func remove_allowed_creator(
    { who } : Types.RemoveAllowedCreatorArgs,
  ) : async Types.RemoveAllowedCreatorResult {
    // Check if the caller is the original canister installer.
    if (caller == installer_) {
      // Caller is authorized, determine if principal to remove is actually on the list.
      if (includesPrincipal_(who, allowedCreatorsList_)) {
        // Principal to remove is on the list, so create a new allowed creators list without that principal.
        allowedCreatorsList_ := Array.filter<Principal>(
          allowedCreatorsList_,
          func(x : Principal) { x != who },
        );
        return #ok({
          message = "Successfully removed principal " # Principal.toText(who) # " from the creators allowed list.";
        });
      } else {
        // Principal to remove was not on the list.
        #err({ kind = #NotFound });
      };
    } else {
      // Caller is not the canister installer and so not authorized to make this call.
      #err({ kind = #NotAuthorized });
    };
  };

  /****Returns the `allowedCreatorsList_`.**  
    _Only authorized for canister's installer._  */
  public shared ({ caller }) func get_allowed_creators_list() : async Types.GetAllowedCreatorsListResult {
    // Check if the caller is the original canister installer.
    if (caller == installer_) {
      return #ok({ allowed = allowedCreatorsList_ });
    } else #err({ kind = #NotAuthorized });
  };

  /****Returns an invoice for the given id if it exists.**  
    _Only authorized for the invoice's creator and those on the invoice's get permission list._  */
  public shared ({ caller }) func get_invoice(
    { id } : Types.GetInvoiceArgs,
  ) : async Types.GetInvoiceResult {
    // Get the invoice for the given id if it exists and if the caller is authorized.
    switch (getInvoiceIfAuthorized_(id, caller, #Get)) {
      // Caller is either not authorized or an invoice for that id doesn't exist.
      case (#err err) return #err(err);
      // Invoice for that id existed, return it to the caller as the record type they expect.
      case (#ok i) {
        #ok({ invoice = toCallerExpectedInvoice_(i) });
      };
    };
  };

  /****Returns an authorized caller's balance.**  
      The balance returned corresponds to the address of the caller as they are an invoice creator;  
    eg the address of the principal subaccount computed for an invoice creator. In other words  
    if the caller as an invoice creator never transfers funds out their subaccount, the returned  
    value would equal to the total amount of proceeds from all created invoices (of same token type)  
    that were successfully verified paid (less the cost of the transfer fee for that specific token  
    type for each paid invoice).  
    _Only authorized for the canister's `installer_` and those on the `allowedCreatorsList_`_  */
  public shared ({ caller }) func get_caller_balance(
    { token } : Types.GetCallerBalanceArgs,
  ) : async Types.GetCallerBalanceResult {
    // Check if the caller is an allowed creator authorized to make this call.
    if (not hasCallPermission_(caller)) return #err({ kind = #NotAuthorized });
    // Caller is authorized, get their creator subaccount address to query its balance.
    let subaccountAddress : SupportedToken.Address = SupportedToken.getCreatorSubaccountAddress({
      token;
      creator = caller;
      canisterId = getInvoiceCanisterId_();
    });
    // Query the corresponding token-ledger canister, wrapping the actual async call with
    // a try/catch to proactively handle the many ways things could go wrong.
    try {
      // SupportedToken.Address is a variant with its tag the name of the token,
      // and its argument the specific address type of that token. Switch on it
      // to unwrap that specific address argument from the tag.
      // The balance is returned to the caller as it is from the token-canister ledger
      // (without the #Ok, since the invoice canister is returning it instead).
      // For example, for a caller querying their ICP creator subaccount balance,
      // they'd get #ok(#ICP{ balance = { e8s = 10000000s }}).
      switch subaccountAddress {
        case (#ICP accountIdentifier) {
          let balance = #ICP(await Ledger_ICP.account_balance({ account = accountIdentifier }));
          #ok({ balance });
        };
        case (#ICP_nns accountIdentifier) {
          let balance = #ICP_nns(await Ledger_ICP_nns.account_balance({ account = accountIdentifier }));
          #ok({ balance });
        };
        case (#ICRC1_ExampleToken account) {
          let balance = #ICRC1_ExampleToken(await Ledger_ICRC1_Ex1.icrc1_balance_of(account));
          #ok({ balance });
        };
        case (#ICRC1_ExampleToken2 account) {
          let balance = #ICRC1_ExampleToken2(await Ledger_ICRC1_Ex2.icrc1_balance_of(account));
          #ok({ balance });
        };
      };
    } catch e {
      // If the inter-canister call failed, return the error's literal message.
      #err({ kind = #CaughtException(Error.message(e)) });
    };
  };

  /****Returns an authorized caller's address.**  
      The address returned corresponds to the address of the caller as they are an invoice creator;  
    eg the address of the principal subaccount computed for an invoice creator. In other words  
    funds sent to pay the amount due of an invoice created by this caller will automatically be  
    transferred from that invoice's subaccount ("payment address") to this address when that  
    invoice is successfully verified as paid. This address is also the same as the one from  
    which funds are sent out of when that same creator calls this canister's `transfer` method.  
    _Only authorized for the canister's `installer_` and those on the `allowedCreatorsList_`_  */
  public shared ({ caller }) func get_caller_address(
    { token } : Types.GetCallerAddressArgs,
  ) : async Types.GetCallerAddressResult {
    // Check if the caller is an allowed creator authorized to make this call.
    if (not hasCallPermission_(caller)) return #err({ kind = #NotAuthorized });
    // Caller is authorized, get their creator subaccount
    // address for the given token type they passed in.
    let asAddress : SupportedToken.Address = SupportedToken.getCreatorSubaccountAddress({
      token;
      creator = caller;
      canisterId = getInvoiceCanisterId_();
    });
    // The token specific address type (account or account identifier) is
    // returned along with its human readable form, so get that as well.
    let asText = SupportedToken.encodeAddress(asAddress);
    // Return both to the caller.
    #ok({ asAddress; asText });
  };

  /****Verifies payment of an invoice for a given invoice id.**  
      An invoice is verified paid if the balance of its payment address ("invoice subaccount") equals  
    or exceeds the amount due for that invoice. If an invoice is verified as paid the total amount of  
    balance in that invoice's subaccount is transferred to the principal subaccount address of that  
    invoice's creator (less the cost of that token type's transfer fee). Additionally the invoice's  
    stored record is updated to include the time of verification and returned to the caller. If no  
    payment has been made the invoice will not be verified and this will return `#Unpaid`; if there's  
    only been partial payment the invoice will also not be verified and this will return `#IncompletePayment`  
    with the `partialAmountPaid`. If this is called after an invoice has already been verified, it will  
    return the invoice as `#VerifiedAlready`.  
      The process of verifying invoices is synchronized by locking to the invoice's id to prevent overwriting  
    in the event multiple callers try to verify the same invoice (or call for balance recovery at the same time);  
    however this lock will automatically be released if enough time has elapsed between calls.  
    _Only authorized for the invoice's creator and those on the invoice's verify permission list._  */
  public shared ({ caller }) func verify_invoice(
    { id } : Types.VerifyInvoiceArgs,
  ) : async Types.VerifyInvoiceResult {
    // Check if an invoice for the given id exists and if the caller is authorized to verify it.
    switch (getInvoiceIfAuthorized_(id, caller, #Verify)) {
      case (#err err) {
        // Caller is either not authorized or an invoice for that id doesn't exist.
        #err(err);
      };
      // Caller is authorized to verify the invoice that does exist.
      case (#ok invoice) {
        let { token; creator } = invoice;
        // Invoice was already verified.
        if (Option.isSome(invoice.verifiedPaidAtTime)) {
          return #ok(#VerifiedAlready({ invoice = toCallerExpectedInvoice_(invoice) }));
        };
        // Check the invoice isn't already being verified or having its subaccount balance recovered.
        if (isAlreadingProcessing_(id, caller)) {
          return #err({ kind = #InProgress });
        };
        // Lock-in the invoice while being processed.
        isAlreadyProcessingLookup_.put(id, (Time.now(), caller));
        // Get the invoice's subaccount address to query its balance.
        let invoiceSubaccountAddress : SupportedToken.Address = SupportedToken.getInvoiceSubaccountAddress({
          token;
          id;
          creator;
          canisterId = getInvoiceCanisterId_();
        });
        // Get the invoice's subaccount address balance (or caught error message).
        // (See `get_caller_balance` for an explanation of using a try/catch, except
        // this time around since there's more to do it'll be set as a Result<ok, err> type).
        let balanceCallResponse : Result.Result<Nat, Text> = try {
          switch invoiceSubaccountAddress {
            case (#ICP accountIdentifier) {
              let { e8s } = await Ledger_ICP.account_balance({
                account = accountIdentifier;
              });
              // Convert the base units as normalized Nat type.
              #ok(Nat64.toNat(e8s));
            };
            case (#ICP_nns accountIdentifier) {
              let { e8s } = await Ledger_ICP_nns.account_balance({
                account = accountIdentifier;
              });
              // Convert the base units as normalized Nat type.
              #ok(Nat64.toNat(e8s));
            };
            case (#ICRC1_ExampleToken account) #ok(await Ledger_ICRC1_Ex1.icrc1_balance_of(account));
            case (#ICRC1_ExampleToken2 account) #ok(await Ledger_ICRC1_Ex2.icrc1_balance_of(account));
          };
        } catch e {
          // If the inter-canister balance query call failed,
          // set it as an err type with the literalized error message.
          #err("Balance call failed for reason: error:\n" # Error.message(e));
        };
        switch balanceCallResponse {
          case (#err err) {
            // Call resulted in a caught error, unlock and inform the caller.
            isAlreadyProcessingLookup_.delete(id);
            #err({ kind = #CaughtException(err) });
          };
          // Balance query call successful, proceed to verifying payment.
          case (#ok bal) {
            if (bal < invoice.amountDue) {
              // Invoice subaccount address balance is less than the invoice's amount due.
              if (bal > 0) {
                // Some payment was made, convert it back to that token's expected amount type.
                let partialAmountPaid : SupportedToken.Amount = SupportedToken.wrapAsTokenAmount(token, bal);
                // Unlock and inform the caller of partial payment.
                isAlreadyProcessingLookup_.delete(id);
                return #err({
                  kind = (#IncompletePayment { partialAmountPaid });
                });
              } else {
                // Zero payment has been made, unlock and inform caller.
                isAlreadyProcessingLookup_.delete(id);
                #err({ kind = #Unpaid });
              };
            } else {
              // Amount due for invoice has at least been paid so transfer those proceeds from
              // the invoice's subaccount to the invoice creator's subaccount.
              // First get the specific token type's transfer fee amount (as normalized base unit Nat type).
              let fee = SupportedToken.getTransactionFee(token);
              // Get the correct transfer args from the invoice's subaccount for this token type.
              let stTransferArgs = SupportedToken.getTransferArgsFromInvoiceSubaccount({
                // (from_subaccount is computed in this method).
                id;
                creator;
                // Be aware that `bal` is required to be >= 2x transfer fee from check during invoice creation.
                amountLessTheFee = (bal - fee);
                fee;
                // Compute the invoice creator's subaccount address as the destination.
                to = SupportedToken.getCreatorSubaccountAddress({
                  token;
                  creator;
                  canisterId = getInvoiceCanisterId_();
                });
              });
              // Make the actual transfer call to the corresponding token-ledger
              // canister wrapped as a try/catch setting a result type so it can be
              // further processed or returned if an error occurred.
              let transferCallResponse : Result.Result<SupportedToken.TransferResult, Text> = try {
                switch stTransferArgs {
                  case (#ICP transferArgs) {
                    let transferResult = await Ledger_ICP.transfer(transferArgs);
                    #ok(#ICP(transferResult));
                  };
                  case (#ICP_nns transferArgs) {
                    let transferResult = await Ledger_ICP_nns.transfer(transferArgs);
                    #ok(#ICP_nns(transferResult));
                  };
                  case (#ICRC1_ExampleToken transferArgs) {
                    let transferResult = await Ledger_ICRC1_Ex1.icrc1_transfer(transferArgs);
                    #ok(#ICRC1_ExampleToken(transferResult));
                  };
                  case (#ICRC1_ExampleToken2 transferArgs) {
                    let transferResult = await Ledger_ICRC1_Ex2.icrc1_transfer(transferArgs);
                    #ok(#ICRC1_ExampleToken2(transferResult));
                  };
                };
              } catch e {
                // If the inter-canister transfer call failed, set it
                // as an err type with the literalized error message.
                #err("Transfer call failed for reason:\n" # Error.message(e));
              };
              switch transferCallResponse {
                case (#err errMsg) {
                  // Call resulted in a caught error, unlock and inform the caller.
                  isAlreadyProcessingLookup_.delete(id);
                  #err({ kind = #CaughtException(errMsg) });
                };
                // Transfer call occurred without failure.
                case (#ok stTransferResult) {
                  // Rewrap transfer results to the type signature the caller expects.
                  switch (SupportedToken.rewrapTransferResults(stTransferResult)) {
                    // Transfer of proceeds to invoice creator's subaccount was a success.
                    case (#ok stTransferSuccess) {
                      // The blockIndex | txIndex could be unwrapped from the
                      // SupportedToken.TransferSuccess (stTransferSuccess)
                      // if logging the invoice's transactions is required.
                      // Create a new updated invoice record to save.
                      let updated = {
                        token;
                        id;
                        creator;
                        details = invoice.details;
                        permissions = invoice.permissions;
                        paymentAddress = invoice.paymentAddress;
                        amountDue = invoice.amountDue;
                        amountPaid = bal;
                        // Update the record with the time of verified payment.
                        verifiedPaidAtTime = ?Time.now();
                      };
                      // Set the trie storing invoices updated with the updated invoice record.
                      invoices_ := Trie.replace(
                        invoices_,
                        key_(id),
                        Text.equal,
                        ?updated,
                      ).0;
                      // Unlock.
                      isAlreadyProcessingLookup_.delete(id);
                      // Return the caller the updated invoice in the form they expected as verified paid.
                      #ok(#VerifiedPaid { invoice = toCallerExpectedInvoice_(updated) });
                    };
                    // Transfer of proceeds to invoice creator's subaccount was not a success.
                    case (#err transferErr) {
                      // Even though amount due was paid and is in the invoice subaccount address, the transferring
                      // of proceeds to the invoice creator's subaccount failed so return the specific token-canister
                      // ledger's transfer error to the caller after unlocking. This should be an edge case as most
                      // of the inputs to the transfer are controlled and prepared specifically not to be a problem.
                      // `verify_invoice` would need to be called again for this invoice to complete successful
                      // verification.
                      isAlreadyProcessingLookup_.delete(id);
                      #err({
                        kind = #SupportedTokenTransferErr(transferErr);
                      });
                    };
                  };
                };
              };
            };
          };
        };
      };
    };
  };

  /****Transfer from caller's subaccount to the specified recipient address.**  
      Funds are transferred out of the caller's subaccount as an invoice creator. In other words, funds  
    available to be transferred are the proceeds of successfully verified invoices that were created by  
    the caller and come from the same address as returned from calling `get_caller_address` (similarly,   
    transferring x amount will reduce the amount returned by `get_caller_balance` less the cost a transfer  
    fee).  
      The given destination can either be an address or its text encoded equivalent provided the text  
    is valid as acceptable address input matching the token type of the given amount.  
    _Only authorized for the canister's `installer_` and those on the `allowedCreatorsList_`_  */
  public shared ({ caller }) func transfer(
    { tokenAmount; destination } : Types.TransferArgs,
  ) : async Types.TransferResult {
    // Check if the caller is an allowed creator authorized to make this call.
    if (not hasCallPermission_(caller)) {
      return #err({ kind = #NotAuthorized });
    };
    // Get the given specific token type and amount in base units as Nat type.
    let (tokenType, amount) = SupportedToken.unwrapTokenAmount(tokenAmount);
    let fee = SupportedToken.getTransactionFee(tokenType);
    // Verify amount to transfer is enough not to trigger the
    // canister a trappin' and instead will end up transferring at least one token.
    if (amount <= fee) {
      return #err({ kind = #InsufficientTransferAmount });
    };
    // Validate the given destination address and decoding it if text was given.
    switch (SupportedToken.getAddressOrUnitErr(tokenType, destination)) {
      // Destination address acceptable.
      case (#ok address) {
        // Prepare the correct transfer args, from the creator's subaccount.
        let stTransferArgs = SupportedToken.getTransferArgsFromCreatorSubaccount({
          // from_subaccount computed in this method.
          creator = caller;
          canisterId = getInvoiceCanisterId_();
          to = address;
          // If it was an ICP token type, this is converted to { e8s } in this method.
          amountLessTheFee = (amount - fee);
          fee;
        });
        // Make the actual transfer call to the corresponding token-ledger
        // canister wrapped as a try/catch setting a result type so it can be
        // further processed or returned if an error occured.
        let transferCallResponse : Result.Result<SupportedToken.TransferResult, Text> = try {
          switch stTransferArgs {
            case (#ICP transferArgs) {
              let transferResult = await Ledger_ICP.transfer(transferArgs);
              #ok(#ICP(transferResult));
            };
            case (#ICP_nns transferArgs) {
              let transferResult = await Ledger_ICP_nns.transfer(transferArgs);
              #ok(#ICP_nns(transferResult));
            };
            case (#ICRC1_ExampleToken transferArgs) {
              let transferResult = await Ledger_ICRC1_Ex1.icrc1_transfer(transferArgs);
              #ok(#ICRC1_ExampleToken(transferResult));
            };
            case (#ICRC1_ExampleToken2 transferArgs) {
              let transferResult = await Ledger_ICRC1_Ex2.icrc1_transfer(transferArgs);
              #ok(#ICRC1_ExampleToken2(transferResult));
            };
          };
        } catch e {
          // If the inter-canister call fails, set result as the error's literal message.
          #err(Error.message(e));
        };
        switch transferCallResponse {
          // Transfer call occurred without failure.
          case (#ok stTransferResult) {
            // Rewrap transfer results to the signature caller expects.
            switch (SupportedToken.rewrapTransferResults(stTransferResult)) {
              case (#ok stTransferSuccess) {
                // stTransferSuccess is a specific token's SupportedToken
                // variant tag with its argument either blockIndex | txIndex.
                #ok(stTransferSuccess);
              };
              case (#err stTransferErr) #err({
                // stTransferErr would be #InsufficientFunds (both ICP and
                // ICRC1 shares this transfer error result) for instance.
                kind = #SupportedTokenTransferErr(stTransferErr);
              });
            };
          };
          // Transfer call resulted in a caught error, inform the caller.
          case (#err errMsg) #err({ kind = #CaughtException(errMsg) });
        };
      };
      // Given destination was not valid or did not match the token amount given to transfer.
      case (#err) #err({ kind = #InvalidDestination });
    };
  };

  /****Recovers funds from an invoice subaccount for the given invoice id.**  
      This method can be used to refund partial payments of an invoice not yet successfully  
    verified paid or transfer funds out from an invoice subaccount already successfully  
    verified if they are mistakenly sent after or in addition to the amount already paid.  
    In either case the total balance of the subaccount will be transferred to the given  
    destination (less the cost a transfer fee); the associated invoice record will not be  
    changed in any way so this is **not** a means to refund an invoice that's already been  
    successfully verified paid (as those proceeds have already been sent  to its creator's  
    subaccount as a result of successful  verification).  
      The given destination can either be an address or its text encoded equivalent provided  
    the text is valid as acceptable address input matching the token type of the invoice for  
    the given id.                 
      The process of recovering an invoice's subaccount balance is synchronized by locking  
    to the invoice's id to prevent conflicts in the event of multiple calls trying to either  
    recover the balance of or verify the same invoice at the same time; however this lock  
    will automatically be released if enough time has elapsed between calls.                                                             
    _Only authorized for the invoice's creator and those on the invoice's verify permission list._  */
  public shared ({ caller }) func recover_invoice_subaccount_balance(
    { id; destination } : Types.RecoverInvoiceSubaccountBalanceArgs,
  ) : async Types.RecoverInvoiceSubaccountBalanceResult {
    // Check if an invoice for the given id exists and if the
    // caller is its creator on its verify permissions list.
    switch (getInvoiceIfAuthorized_(id, caller, #Verify)) {
      case (#err err) {
        // Caller is either not authorized or an invoice for that id doesn't exist.
        #err(err);
      };
      case (#ok invoice) {
        let { token; creator } = invoice;
        // Caller is authorized & invoice exists, first proceed by validating the destination address.
        switch (SupportedToken.getAddressOrUnitErr(token, destination)) {
          case (#err) {
            // Destination was invalid or its address type did not match the payment token type.
            #err({ kind = #InvalidDestination });
          };
          // Destination is acceptable.
          // Proceed to check invoice's subaccount address balance.
          case (#ok destinationAddress) {
            // Check the invoice isn't already being verified or having its subaccount balance recovered.
            if (isAlreadingProcessing_(id, caller)) {
              return #err({ kind = #InProgress });
            };
            // Lock-in the invoice by its id while being processed.
            isAlreadyProcessingLookup_.put(id, (Time.now(), caller));
            // Get the invoice's subaccount address to query its balance.
            let invoiceSubaccountAddress = SupportedToken.getInvoiceSubaccountAddress({
              token;
              id;
              creator;
              canisterId = getInvoiceCanisterId_();
            });
            // Make the actual balance query call to the corresponding token-ledger
            // canister wrapped as a try/catch setting a result type so it can be
            // further processed or returned if an error occured.
            let balanceCallResponse : Result.Result<Nat, Text> = try {
              switch invoiceSubaccountAddress {
                case (#ICP accountIdentifier) {
                  let { e8s } = await Ledger_ICP.account_balance({
                    account = accountIdentifier;
                  });
                  // Convert the amount to the normalized base units as Nat type.
                  #ok(Nat64.toNat(e8s));
                };
                case (#ICP_nns accountIdentifier) {
                  let { e8s } = await Ledger_ICP_nns.account_balance({
                    account = accountIdentifier;
                  });
                  // Convert the balance amount to the normalized base units as Nat type.
                  #ok(Nat64.toNat(e8s));
                };
                case (#ICRC1_ExampleToken account) #ok(await Ledger_ICRC1_Ex1.icrc1_balance_of(account));
                case (#ICRC1_ExampleToken2 account) #ok(await Ledger_ICRC1_Ex2.icrc1_balance_of(account));
              };
            } catch e {
              // If the inter-canister balance query call failed,
              // set it as an err type with the literalized error message.
              #err("Balance call failed for reason:\n" # Error.message(e));
            };
            switch balanceCallResponse {
              case (#err err) {
                // Unlock and inform the caller the balance
                // query call resulted in a caught error.
                isAlreadyProcessingLookup_.delete(id);
                #err({ kind = #CaughtException(err) });
              };
              case (#ok currentBalance) {
                if (currentBalance == 0) {
                  // Unlock and return there's no balance to recover.
                  isAlreadyProcessingLookup_.delete(id);
                  return #err({ kind = #NoBalance });
                } else {
                  let fee = SupportedToken.getTransactionFee(token);
                  // Verify amount to transfer is enough not to trap covering transfer
                  // fee and ends up transferring at least one base unit token.
                  if (currentBalance <= fee) {
                    // Balance was not enough and is effectively irrecoverable.
                    // Unlock and inform the caller.
                    isAlreadyProcessingLookup_.delete(id);
                    return #err({ kind = #InsufficientTransferAmount });
                  } else {
                    // Prepare the correct transfer args from the invoice's subaccount.
                    let stTransferArgs = SupportedToken.getTransferArgsFromInvoiceSubaccount({
                      // from_subaccount computed in this method.
                      id;
                      creator;
                      // If it is ICP it is converted to { e8s } in this method.
                      amountLessTheFee = (currentBalance - fee);
                      fee;
                      to = destinationAddress;
                    });
                    // Make the actual transfer call to the corresponding token-ledger
                    // canister wrapped as a try/catch setting a result type so it can be
                    // further processed or returned if an error occured.
                    let transferCallResponse : Result.Result<SupportedToken.TransferResult, Text> = try {
                      switch stTransferArgs {
                        case (#ICP transferArgs) {
                          let transferResult = await Ledger_ICP.transfer(transferArgs);
                          #ok(#ICP(transferResult));
                        };
                        case (#ICP_nns transferArgs) {
                          let transferResult = await Ledger_ICP_nns.transfer(transferArgs);
                          #ok(#ICP_nns(transferResult));
                        };
                        case (#ICRC1_ExampleToken transferArgs) {
                          let transferResult = await Ledger_ICRC1_Ex1.icrc1_transfer(transferArgs);
                          #ok(#ICRC1_ExampleToken(transferResult));
                        };
                        case (#ICRC1_ExampleToken2 transferArgs) {
                          let transferResult = await Ledger_ICRC1_Ex2.icrc1_transfer(transferArgs);
                          #ok(#ICRC1_ExampleToken2(transferResult));
                        };
                      };
                    } catch e {
                      // If the inter-canister transfer call failed, set it as
                      // an err type with the literalized error message.
                      #err("Transfer call failed for reason:\n" # Error.message(e));
                    };
                    switch transferCallResponse {
                      case (#err errMsg) {
                        // Unlock and inform the caller the transfer call to
                        // recover funds resulted in a caught exception.
                        isAlreadyProcessingLookup_.delete(id);
                        #err({ kind = #CaughtException(errMsg) });
                      };
                      // Transfer call occured without failure.
                      case (#ok stTransferResult) {
                        // Rewrap transfer results to signature caller expects.
                        switch (SupportedToken.rewrapTransferResults(stTransferResult)) {
                          // Transfer to recover funds was a success.
                          case (#ok transferSuccess) {
                            // Determine the amount the caller can expect to find
                            // deposited in the destination address they provided.
                            let balanceRecovered = SupportedToken.wrapAsTokenAmount(token, currentBalance - fee);
                            // Unlock.
                            isAlreadyProcessingLookup_.delete(id);
                            // Return the good news to the caller.
                            #ok({ transferSuccess; balanceRecovered });
                          };
                          // Transfer to recover funds was not a success.
                          case (#err transferErr) {
                            isAlreadyProcessingLookup_.delete(id);
                            #err({
                              kind = #SupportedTokenTransferErr(transferErr);
                            });
                          };
                        };
                      };
                    };
                  };
                };
              };
            };
          };
        };
      };
    };
  };

  /****Converts an address or caller's principal to its other address forms.**  
    * If the caller passes in an address (account or account identifier) to encode as text   
    they do not have to also pass in which token type.  
    * If the caller passes in text they must also pass in which token type to decode as an address.  
    * Finally if the caller only passes which token type this will return the address corresponding  
    to that principal's default subaccount for that address type.  
      In **all** cases both the address (as the argument of the corresponding `SupportedToken` variant)  
    and its equivalent in text format will be returned. Text unacceptable to be decoded or faulty  
    addresses to encoded into text will return an err kind `#InvalidDestination`.  
    _Only authorized for the canister's `installer_` and those on the `allowedCreatorsList_`_  */
  public shared ({ caller }) func to_other_address_format(
    // Note both address and token are opt types.
    { address; token } : Types.ToOtherAddressFormatArgs,
  ) : async Types.ToOtherAddressFormatResult {
    if (not hasCallPermission_(caller)) return #err({ kind = #NotAuthorized });
    // Determine if caller passed in address or is calling for default subaccount computation.
    switch address {
      case null {
        // Caller didn't pass in address or text, determine which kind of default
        // subaccount address to compute from the caller's principal.
        switch token {
          case (?tokenType) {
            // This method returns the address as { asAddress; asText }.
            #ok(SupportedToken.getDefaultSubaccountAddress(tokenType, caller));
          };
          // Caller passed neither address nor token type.
          case null #err({ kind = #MissingTokenType });
        };
      };
      // Caller passed in an address to convert.
      case (?recipientAddress) {
        // Determine if it was text or canister expected format.
        switch recipientAddress {
          // Caller passed in text to decode.
          case (#HumanReadable asText) {
            // Determine which address type to decode text into.
            switch token {
              case (?tokenType) {
                // Caller passed in text and token type of address to decode.
                switch (SupportedToken.getAddressOrUnitErr(tokenType, recipientAddress)) {
                  // Text given was acceptable to be decoded into given
                  // token type, return both formats back to the caller.
                  case (#ok asAddress) #ok({ asAddress; asText });
                  // Text supplied was not valid for the given token type.
                  case (#err) #err({ kind = #InvalidDestination });
                };
              };
              // Caller passed in text, but no token type to decode address into.
              case null #err({ kind = #MissingTokenType });
            };
          };
          // Caller passed in address type (account or account identifier) to encode.
          case (#CanisterExpected asAddress) {
            switch (SupportedToken.encodeAddressOrUnitErr(asAddress)) {
              // Address was acceptable to encode, return both formats back to caller.
              case (#ok asText) #ok({ asAddress; asText });
              // Address was faulty to encode into text, inform the caller it was invalid.
              case (#err) #err({ kind = #InvalidDestination });
            };
          };
        };
      };
    };
  };
};
