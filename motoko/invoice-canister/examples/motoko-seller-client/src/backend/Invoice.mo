import Array "mo:base/Array";
import Error "mo:base/Error";
import HashMap "mo:base/HashMap";
import Iter "mo:base/Iter";
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

import MockTokenLedgerCanisters "./modules/MockTokenLedgerCanisters";
import SupportedToken "./modules/SupportedToken";
import Types "./modules/Types";

// Same as the original Invoice.mo, Types.mo and SupportedToken.mo except for:
//  -Only two SupportedToken variant members: #ICP and #ICRC1.
//  -seller canister id is hard-coded and added to the allowed creator's list.
//  -deposit_free_money to transfer an amount to either of the mock ledgers.
//  -All distinct modules of `SupportedToken` are contained in `SupportedToken.mo`.

shared ({ caller = installer_ }) actor class Invoice() = this {

  /*--------------------------------------------------------------------------- 
  */ // Motoko-Seller-Client Functionality

  // Hard coded seller canister's id to be added to allowed creators list.
  let SELLER_CANISTER_ID = Principal.fromText("r7inp-6aaaa-aaaaa-aaabq-cai");

  /** 
    Normally actual actors would be used (commented out below in "Application State" section), 
  but for this example the mock ICP and ICRC1 token-ledger canisters are used instead.
  Note this deposit is for testing purposes, deposit_free_money is called again by the seller
  when mocking an invoice payment (after an invoice is generated and confirm is clicked).  */
  let Ledger_ICP : MockTokenLedgerCanisters.ICP.MockLedger = MockTokenLedgerCanisters.ICP.MockLedger(
    {
      who = installer_;
      amount = { e8s = 100_000_000_000 };
    },
    ?true,
  );
  let Ledger_ICRC1 : MockTokenLedgerCanisters.ICRC1.MockLedger = MockTokenLedgerCanisters.ICRC1.MockLedger(
    {
      who = installer_;
      amount = 100_000_000_000;
    },
    ?true,
  );

  /** Args of deposit_free_money utility method.  */
  public type FreeMoneyArgs = {
    tokenAmount : SupportedToken.Amount;
    destination : SupportedToken.RecipientAddress;
  };

  /** Results of deposit_free_money utility method.  */
  public type FreeMoneyResult = Result.Result<SupportedToken.TransferSuccess, FreeMoneyError>;

  /** err results of deposit_free_money utility method.  */
  public type FreeMoneyError = {
    kind : {
      #InvalidDestination;
      #SupportedTokenTransferErr : Text;
    };
  };

  /** Utility method to easily deposit funds into an ICP or ICRC1 address (account, account identifier or
    address encoded as text). If text is passed, it must match the token type of `tokenAmount`.  */
  public func deposit_free_money({ tokenAmount; destination } : FreeMoneyArgs) : async FreeMoneyResult {
    let (token, amount) = SupportedToken.unwrapTokenAmount(tokenAmount);
    switch (SupportedToken.getAddressOrUnitErr(token, destination)) {
      case (#ok address) {
        switch address {
          case (#ICP accountIdentifier) {
            if (token == #ICP) {
              switch (Ledger_ICP.deposit_free_money({ recipient = accountIdentifier; amount = { e8s = Nat64.fromNat(amount) } })) {
                case (#Ok blockIndex) #ok(#ICP(blockIndex));
                case (#Err transferErr) #err({
                  kind = (#SupportedTokenTransferErr(debug_show (transferErr)));
                });
              };
            } else {
              #err({ kind = #InvalidDestination });
            };
          };
          case (#ICRC1 account) {
            if (token == #ICRC1) {
              switch (Ledger_ICRC1.deposit_free_money({ recipient = account; amount })) {
                case (#Ok txIndex) #ok(#ICRC1(txIndex));
                case (#Err transferErr) #err({
                  kind = (#SupportedTokenTransferErr(debug_show (transferErr)));
                });
              };
            } else {
              #err({ kind = #InvalidDestination });
            };
          };
        };
      };
      case (#err) #err({ kind = #InvalidDestination });
    };
  };

  /*--------------------------------------------------------------------------- 
  */ // Application State

  /** Compulsory constants this canister must adhere to.  */
  module MagicNumbers {
    // Invoice Canister Constraints:
    public let SMALL_CONTENT_SIZE = 256;
    public let LARGE_CONTENT_SIZE = 32_000;
    public let MAX_INVOICES = 30_000;
    public let MAX_INVOICE_CREATORS = 256;
  };

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
  stable var allowedCreatorsList_ : [Principal] = [SELLER_CANISTER_ID];

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
    exists for a given id or #NotFound if it doesn't, if the caller is authorized; or  
    #NotAuthorized otherwise and even if it doesn't exist if the caller is not authorized.  
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
        if (not hasCallPermission_(caller)) {
          #err({ kind = #NotAuthorized });
        } else {
          #err({ kind = #NotFound });
        };
      };
      case (?invoice) {
        if (invoice.creator == caller) {
          return #ok(invoice);
        };
        switch (invoice.permissions) {
          case null return #err({ kind = #NotAuthorized });
          case (?permissions) {
            let allowed = switch (permission) {
              case (#Get) { permissions.canGet };
              case (#Verify) { permissions.canVerify };
            };
            if (includesPrincipal_(caller, allowed)) {
              return #ok(invoice);
            } else {
              #err({ kind = #NotAuthorized });
            };
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
      case null return false;
      case (?(atTime, who)) {
        if ((Time.now() - atTime) >= isAlreadyProcessingTimeout_) {
          isAlreadyProcessingLookup_.delete(id);
          return false;
        } else {
          true;
        };
      };
    };
  };

  /** Decorates an `Invoice_` with `paid : Bool` and `tokenVerbose : TokenVerbose`  
    before it returned to a caller as an `Invoice`.  */
  func toCallerExpectedInvoice_(i : Types.Invoice_) : Types.Invoice {
    {
      i with paid = Option.isSome(i.verifiedPaidAtTime);
      tokenVerbose = SupportedToken.getTokenVerbose(i.token);
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
    if (not hasCallPermission_(caller)) {
      return #err({ kind = #NotAuthorized });
    };
    if (invoiceCounter_ >= MagicNumbers.MAX_INVOICES) {
      return #err({ kind = #MaxInvoicesCreated });
    };
    switch details {
      case null {};
      case (?details_) {
        if (details_.meta.size() > MagicNumbers.LARGE_CONTENT_SIZE) {
          return #err({ kind = #MetaTooLarge });
        };
        if (details_.description.size() > MagicNumbers.SMALL_CONTENT_SIZE) {
          return #err({ kind = #DescriptionTooLarge });
        };
      };
    };
    switch permissions {
      case null {};
      case (?permissions_) {
        let g = (permissions_.canGet.size() > MagicNumbers.SMALL_CONTENT_SIZE);
        let v = (permissions_.canVerify.size() > MagicNumbers.SMALL_CONTENT_SIZE);
        if (g or v) {
          return #err({ kind = #TooManyPermissions });
        };
      };
    };
    let (token, amount) = SupportedToken.unwrapTokenAmount(tokenAmount);
    // Verify the amount due is at least enough to cover the transfer of any proceeds from its
    // subaccount to the subaccount of its creator upon successful verification of payment.
    if (amount < (2 * SupportedToken.getTransactionFee(token))) {
      return #err({ kind = #InsufficientAmountDue });
    };
    // Caller provided acceptable create_invoice input args! Proceed with invoice creation...
    let id = ULID.toText(idCreationEntropy_.new());
    let paymentAddress : Text = SupportedToken.getEncodedInvoiceSubaccountAddress({
      token;
      id;
      creator = caller;
      canisterId = getInvoiceCanisterId_();
    });
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
    invoiceCounter_ += 1;
    invoices_ := Trie.put(
      invoices_,
      key_(id),
      Text.equal,
      createdInvoice,
    ).0;
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
    if (allowedCreatorsList_.size() < MagicNumbers.MAX_INVOICE_CREATORS) {
      if (caller == installer_) {
        if (Principal.isAnonymous(who)) {
          return #err({ kind = #AnonymousIneligible });
        };
        if (not includesPrincipal_(who, allowedCreatorsList_)) {
          allowedCreatorsList_ := Array.flatten<Principal>([allowedCreatorsList_, [who]]);
          return #ok({
            message = "Successfully added " # Principal.toText(who) # " to creators allowed list.";
          });
        } else {
          #err({ kind = #AlreadyAdded });
        };
      } else #err({ kind = #NotAuthorized });
    } else {
      #err({ kind = #MaxAllowed });
    };
  };

  /****Removes a principal from the `allowedCreatorsList_`.**  
    _Only authorized for canister's installer._  */
  public shared ({ caller }) func remove_allowed_creator(
    { who } : Types.RemoveAllowedCreatorArgs,
  ) : async Types.RemoveAllowedCreatorResult {
    if (caller == installer_) {
      if (includesPrincipal_(who, allowedCreatorsList_)) {
        allowedCreatorsList_ := Array.filter<Principal>(
          allowedCreatorsList_,
          func(x : Principal) { x != who },
        );
        return #ok({
          message = "Successfully removed principal " # Principal.toText(who) # " from the creators allowed list.";
        });
      } else { #err({ kind = #NotFound }) };
    } else {
      #err({ kind = #NotAuthorized });
    };
  };

  /****Returns the `allowedCreatorsList_`.**  
    _Only authorized for canister's installer._  */
  public shared ({ caller }) func get_allowed_creators_list() : async Types.GetAllowedCreatorsListResult {
    if (caller == installer_) {
      return #ok({ allowed = allowedCreatorsList_ });
    } else #err({ kind = #NotAuthorized });
  };

  /****Returns an invoice for the given id if it exists.**  
    _Only authorized for the invoice's creator and those on the invoice's get permission list._  */
  public shared ({ caller }) func get_invoice(
    { id } : Types.GetInvoiceArgs,
  ) : async Types.GetInvoiceResult {
    switch (getInvoiceIfAuthorized_(id, caller, #Get)) {
      case (#err err) return #err(err);
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
    if (not hasCallPermission_(caller)) return #err({ kind = #NotAuthorized });
    let subaccountAddress = SupportedToken.getCreatorSubaccountAddress({
      token;
      creator = caller;
      canisterId = getInvoiceCanisterId_();
    });
    try {
      switch subaccountAddress {
        case (#ICP accountIdentifier) {
          let balance = #ICP(await Ledger_ICP.account_balance({ account = accountIdentifier }));
          #ok({ balance });
        };
        case (#ICRC1 account) {
          let balance = #ICRC1(await Ledger_ICRC1.icrc1_balance_of(account));
          #ok({ balance });
        };
      };
    } catch e {
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
    if (not hasCallPermission_(caller)) return #err({ kind = #NotAuthorized });
    let asAddress = SupportedToken.getCreatorSubaccountAddress({
      token;
      creator = caller;
      canisterId = getInvoiceCanisterId_();
    });
    let asText = SupportedToken.encodeAddress(asAddress);
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
    switch (getInvoiceIfAuthorized_(id, caller, #Verify)) {
      case (#err err) return #err(err);
      case (#ok invoice) {
        let { token; creator } = invoice;
        if (Option.isSome(invoice.verifiedPaidAtTime)) {
          return #ok(#VerifiedAlready({ invoice = toCallerExpectedInvoice_(invoice) }));
        };
        if (isAlreadingProcessing_(id, caller)) {
          return #err({ kind = #InProgress });
        };
        isAlreadyProcessingLookup_.put(id, (Time.now(), caller));
        let invoiceSubaccountAddress = SupportedToken.getInvoiceSubaccountAddress({
          token;
          id;
          creator;
          canisterId = getInvoiceCanisterId_();
        });
        let balanceCallResponse : Result.Result<Nat, Text> = try {
          switch invoiceSubaccountAddress {
            case (#ICP accountIdentifier) {
              let { e8s } = await Ledger_ICP.account_balance({
                account = accountIdentifier;
              });
              #ok(Nat64.toNat(e8s));
            };
            case (#ICRC1 account) #ok(await Ledger_ICRC1.icrc1_balance_of(account));
          };
        } catch e {
          #err("Balance called failed for reason:\n" # Error.message(e));
        };
        switch balanceCallResponse {
          case (#err err) {
            isAlreadyProcessingLookup_.delete(id);
            return #err({ kind = #CaughtException(err) });
          };
          case (#ok bal) {
            if (bal < invoice.amountDue) {
              if (bal > 0) {
                let partialAmountPaid = SupportedToken.wrapAsTokenAmount(token, bal);
                isAlreadyProcessingLookup_.delete(id);
                #err({
                  kind = (#IncompletePayment { partialAmountPaid });
                });
              } else {
                isAlreadyProcessingLookup_.delete(id);
                #err({ kind = #Unpaid });
              };
            } else {
              // Since balance has been paid, transfer proceeds to creator's subaccount.
              let fee = SupportedToken.getTransactionFee(token);
              let stTransferArgs = SupportedToken.getTransferArgsFromInvoiceSubaccount({
                id;
                creator;
                // Note bal is required to be >= 2x transfer fee when invoice was created.
                amountLessTheFee = (bal - fee);
                fee;
                to = SupportedToken.getCreatorSubaccountAddress({
                  token;
                  creator;
                  canisterId = getInvoiceCanisterId_();
                });
              });
              let transferCallResponse : Result.Result<SupportedToken.TransferResult, Text> = try {
                switch stTransferArgs {
                  case (#ICP transferArgs) {
                    let transferResult = await Ledger_ICP.transfer(getInvoiceCanisterId_(), transferArgs);
                    #ok(#ICP(transferResult));
                  };
                  case (#ICRC1 transferArgs) {
                    let transferResult = await Ledger_ICRC1.icrc1_transfer(getInvoiceCanisterId_(), transferArgs);
                    #ok(#ICRC1(transferResult));
                  };
                };
              } catch e {
                #err("Transfer call failed for reason:\n" # Error.message(e));
              };
              switch transferCallResponse {
                case (#err errMsg) {
                  isAlreadyProcessingLookup_.delete(id);
                  #err({ kind = #CaughtException(errMsg) });
                };
                case (#ok stTransferResult) {
                  switch (SupportedToken.rewrapTransferResults(stTransferResult)) {
                    case (#ok transferSuccess) {
                      // transferSuccess contains (blockIndex | txIndex) and could
                      // also be returned to caller or sent to a logger if indexing
                      // invoice transactions is needed.
                      let updated = {
                        token;
                        id;
                        creator;
                        details = invoice.details;
                        permissions = invoice.permissions;
                        paymentAddress = invoice.paymentAddress;
                        amountDue = invoice.amountDue;
                        amountPaid = bal;
                        verifiedPaidAtTime = ?Time.now();
                      };
                      invoices_ := Trie.replace(
                        invoices_,
                        key_(id),
                        Text.equal,
                        ?updated,
                      ).0;
                      isAlreadyProcessingLookup_.delete(id);
                      #ok(#VerifiedPaid { invoice = toCallerExpectedInvoice_(updated) });
                    };
                    case (#err transferErr) {
                      // Even though balance was paid, transferring proceeds to the invoice creator's subaccount
                      // failed so return the specific token ledger's transfer err to the caller after unlocking.
                      // verify_invoice would need to be called again for this invoice to complete successful
                      // verification (note the amount due balance _has_ been paid & still in invoice subaccount).
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
    if (not hasCallPermission_(caller)) {
      return #err({ kind = #NotAuthorized });
    };
    let (tokenType, amount) = SupportedToken.unwrapTokenAmount(tokenAmount);
    let fee = SupportedToken.getTransactionFee(tokenType);
    // Verify amount to transfer is enough not to trap covering the
    // transfer fee and ending up transferring at least one token.
    if (amount <= fee) {
      return #err({ kind = #InsufficientTransferAmount });
    };
    switch (SupportedToken.getAddressOrUnitErr(tokenType, destination)) {
      case (#ok address) {
        let stTransferArgs = SupportedToken.getTransferArgsFromCreatorSubaccount({
          creator = caller;
          canisterId = getInvoiceCanisterId_();
          to = address;
          amountLessTheFee = (amount - fee);
          fee;
        });
        let transferCallResponse : Result.Result<SupportedToken.TransferResult, Text> = try {
          switch stTransferArgs {
            case (#ICP transferArgs) {
              let transferResult = await Ledger_ICP.transfer(getInvoiceCanisterId_(), transferArgs);
              #ok(#ICP(transferResult));
            };
            case (#ICRC1 transferArgs) {
              let transferResult = await Ledger_ICRC1.icrc1_transfer(getInvoiceCanisterId_(), transferArgs);
              #ok(#ICRC1(transferResult));
            };
          };
        } catch e {
          #err(Error.message(e));
        };
        switch transferCallResponse {
          case (#ok stTransferResult) {
            switch (SupportedToken.rewrapTransferResults(stTransferResult)) {
              case (#ok stTransferSuccess) #ok(stTransferSuccess);
              case (#err stTransferErr) #err({
                kind = #SupportedTokenTransferErr(stTransferErr);
              });
            };
          };
          case (#err errMsg) {
            #err({ kind = #CaughtException(errMsg) });
          };
        };
      };
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
    switch (getInvoiceIfAuthorized_(id, caller, #Verify)) {
      case (#err err) return #err(err);
      case (#ok invoice) {
        let { token; creator } = invoice;
        switch (SupportedToken.getAddressOrUnitErr(token, destination)) {
          case (#err) return #err({ kind = #InvalidDestination });
          case (#ok destinationAddress) {
            if (isAlreadingProcessing_(id, caller)) {
              return #err({ kind = #InProgress });
            };
            isAlreadyProcessingLookup_.put(id, (Time.now(), caller));
            let invoiceSubaccountAddress = SupportedToken.getInvoiceSubaccountAddress({
              token;
              id;
              creator;
              canisterId = getInvoiceCanisterId_();
            });
            let balanceCallResponse : Result.Result<Nat, Text> = try {
              switch invoiceSubaccountAddress {
                case (#ICP accountIdentifier) {
                  let { e8s } = await Ledger_ICP.account_balance({
                    account = accountIdentifier;
                  });
                  #ok(Nat64.toNat(e8s));
                };
                case (#ICRC1 account) #ok(await Ledger_ICRC1.icrc1_balance_of(account));
              };
            } catch e {
              #err("Balance call failed for reason:\n" # Error.message(e));
            };
            switch balanceCallResponse {
              case (#err err) {
                isAlreadyProcessingLookup_.delete(id);
                #err({ kind = #CaughtException(err) });
              };
              case (#ok currentBalance) {
                if (currentBalance == 0) {
                  isAlreadyProcessingLookup_.delete(id);
                  return #err({ kind = #NoBalance });
                } else {
                  let fee = SupportedToken.getTransactionFee(token);
                  // Verify amount to transfer is enough not to trap covering the
                  // transfer fee and ending up transferring at least one token.
                  if (currentBalance <= fee) {
                    isAlreadyProcessingLookup_.delete(id);
                    return #err({ kind = #InsufficientTransferAmount });
                  } else {
                    let stTransferArgs = SupportedToken.getTransferArgsFromInvoiceSubaccount({
                      id;
                      creator;
                      amountLessTheFee = (currentBalance - fee);
                      fee;
                      to = destinationAddress;
                    });
                    let transferCallResponse : Result.Result<SupportedToken.TransferResult, Text> = try {
                      switch stTransferArgs {
                        case (#ICP transferArgs) {
                          let transferResult = await Ledger_ICP.transfer(getInvoiceCanisterId_(), transferArgs);
                          #ok(#ICP(transferResult));
                        };
                        case (#ICRC1 transferArgs) {
                          let transferResult = await Ledger_ICRC1.icrc1_transfer(getInvoiceCanisterId_(), transferArgs);
                          #ok(#ICRC1(transferResult));
                        };
                      };
                    } catch e {
                      #err("Transfer call failed for reason:\n" # Error.message(e));
                    };
                    switch transferCallResponse {
                      case (#err errMsg) {
                        isAlreadyProcessingLookup_.delete(id);
                        #err({ kind = #CaughtException(errMsg) });
                      };
                      case (#ok stTransferResult) {
                        switch (SupportedToken.rewrapTransferResults(stTransferResult)) {
                          case (#ok transferSuccess) {
                            let balanceRecovered = SupportedToken.wrapAsTokenAmount(token, currentBalance - fee);
                            isAlreadyProcessingLookup_.delete(id);
                            #ok({ transferSuccess; balanceRecovered });
                          };
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
    they do not have to also pass in which token (type).  
    * If the caller passes in text they must also pass in which token (type) to decode as an address.  
    * Finally if the caller only passes which token (type) this will return the address corresponding  
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
    // Determine if caller supplied address or is calling for default subaccount computation.
    switch address {
      case null {
        // Caller didn't supply address or text, determine which kind of default
        // subaccount address to compute from the caller's principal.
        switch token {
          case (?tokenType) {
            #ok(SupportedToken.getDefaultSubaccountAddress(tokenType, caller));
          };
          // Caller passed neither address nor token type.
          case null #err({ kind = #MissingTokenType });
        };
      };
      case (?recipientAddress) {
        switch recipientAddress {
          case (#HumanReadable asText) {
            // Caller supplied text to decode.
            switch token {
              // Determine which address type to decode text into.
              case (?tokenType) {
                switch (SupportedToken.getAddressOrUnitErr(tokenType, recipientAddress)) {
                  case (#ok asAddress) #ok({ asAddress; asText });
                  case (#err) #err({ kind = #InvalidDestination });
                };
              };
              case null #err({ kind = #MissingTokenType });
            };
          };
          case (#CanisterExpected asAddress) {
            switch (SupportedToken.encodeAddressOrUnitErr(asAddress)) {
              case (#ok asText) #ok({ asAddress; asText });
              case (#err) #err({ kind = #InvalidDestination });
            };
          };
        };
      };
    };
  };
};
