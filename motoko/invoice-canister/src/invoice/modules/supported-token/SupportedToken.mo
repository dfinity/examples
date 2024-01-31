import Blob "mo:base/Blob";
import Nat64 "mo:base/Nat64";
import Result "mo:base/Result";
import AccountIdentifierBlob "mo:principal/blob/AccountIdentifier";

import Supertype_ICP "./token-specific/icp/ActorSupertype";
import { icpAdapter } "./token-specific/icp/Adapter";
import ICP "./token-specific/icp/Types";
import Supertype_ICRC1 "./token-specific/icrc1/ActorSupertype";
import { icrc1Adapter } "./token-specific/icrc1/Adapter";
import ICRC1 "./token-specific/icrc1/Types";

/****Core module composing the uniform interface between types adapting token-ledger canister 
  actors to types processed for and expected by the invoice canister caller.**  
  \
  If adding support for more ICP based or ICRC1 standard tokens, first add a new tag that will
  correspond to the token to support in the `SupportedToken<T1, T2>` variant declaration below.  
  \
  When this declaration is modified, it will trigger the Motoko VSCode extension to indicate  
  all the other places that need to be edited to complete integration for support of that token.   
  These include all the methods of this module and the four relevant Invoice Canister API methods:   
    `get_caller_balance()`,  
    `verify_invoice()`,  
    `transfer()` and   
    `recover_invoice_subaccount_balance()`.  
  \
  Each method of `./SupportedToken.mo` contains a switch that will need an added case  
  for the new tag; the four API methods in `Invoice.mo` also have switches that need an added case,   
  however the logic of those cases involve adding the correct actor supertype calls. In all cases,  
  the existing implementation can be used as a guide to copy the logic from just be sure to   
  correctly update all references to the new tag.  
  \
  It is recommended to first start by adding the case to `getTokenVerbose()` as the transfer fee  
  assigned in that tag's TokenVerbose will be used to execute transactions correctly by the   
  Invoice Canister.  
  \
  Note that when the `SupportedToken<T1, T2>` declaration (from `./Types.mo`) is modified, it will  
  trigger the Motoko VSCode extension to indicate all the other places that need to be edited to  
  complete integration for support of that token. After the methods in this file have been completed,  
  the last changes to make are the four relevant API methods in `Invoice.mo`.  */
module SupportedToken {

  /** Redeclared actor supertype of an ICP ledger canister for importing convenience.  */
  public type Actor_Supertype_ICP = Supertype_ICP.Actor;

  /** Redeclared actor supertype of an ICRC1 token-ledger canister for importing convenience.  */
  public type Actor_Supertype_ICRC1 = Supertype_ICRC1.Actor;

  // Redeclared to visually distinguish the code below.
  let ICP_Adapter = icpAdapter;
  let ICRC1_Adapter = icrc1Adapter;

  /****Some type that makes the rest possible.**  
    Tag entries of this variant are all the tokens this invoice canister supports.  
    The generic arguments are the distinct types each token uses and may be shared: additional  
    ICRC1 standard tokens can reuse the generic type T2. Adding or removing a tag entry will  
    trigger the VSCode Motoko extension to indicate all the methods' switches' cases that need  
    to be edited to add or remove support for a particular token (the methods of this module  
    and the four of `Invoice.mo`).  */
  public type SupportedToken<T1, T2> = {
    #ICP : T1;
    #ICP_nns : T1;
    #ICRC1_ExampleToken : T2;
    #ICRC1_ExampleToken2 : T2;
    //  #ICRC1_xdr       : T2;
    //  #ICRC1_ckbtc     : T2;
    //  etc
  };

  /** Corresponding supported token "base" types: the tag is used a parameter to specify which token. */
  public type UnitType = SupportedToken<(), ()>;

  /** Corresponding supported token type canister expected amount types. */
  public type Amount = SupportedToken<ICP.Tokens, ICRC1.Tokens>;

  /** Corresponding supported token type canister expected address types. */
  public type Address = SupportedToken<ICP.AccountIdentifier, ICRC1.Account>;

  /** Corresponding supported token type canister expected TransferArgs types. */
  public type TransferArgs = SupportedToken<ICP.TransferArgs, ICRC1.TransferArgs>;

  /** Corresponding supported token type canister expected TransferResult types. */
  public type TransferResult = SupportedToken<ICP.TransferResult, ICRC1.TransferResult>;

  /** Corresponding supported token type canister expected TransferSuccess types. */
  public type TransferSuccess = SupportedToken<ICP.BlockIndex, ICRC1.TxIndex>;

  /** Corresponding supported token type canister expected TransferErr types. */
  public type TransferErr = SupportedToken<ICP.TransferError, ICRC1.TransferError>;

  /****Sum type for converting between human parsable and canister expected address types.***/
  public type RecipientAddress = {
    #HumanReadable : Text;
    #CanisterExpected : Address;
  };

  /****Hard coded token specific information record.**  
    At least the fee must be correctly defined for additional tokens in `getTokenVerbose()` below.  
    It is strongly encouraged to set the other fields correctly as well, in particular the `Url` 
    can point to the token-ledger canister's url of the ICP dashboard.  */
  public type TokenVerbose = {
    symbol : Text;
    name : Text;
    decimals : Int;
    fee : Nat;
    meta : ?{
      Issuer : Text;
      Url : Text;
    };
  };

  /****Additional supported tokens **must** at least have their correct transfer fee defined here.***/
  public func getTokenVerbose<T1, T2>(supportedToken : SupportedToken<T1, T2>) : TokenVerbose {
    switch supportedToken {
      case (#ICP T1) {
        return {
          symbol = "_ICP";
          name = "Internet Computer Protocol Token";
          decimals = 8 : Int;
          fee = 10_000;
          meta = ?{
            Issuer = "e8s - For Demonstration Purposes";
            Url = "https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/interact-with-ledger";
          };
        };
      };
      case (#ICP_nns T1) {
        return {
          symbol = "_ICP_nns";
          name = "Internet Computer Protocol Token NNS";
          decimals = 8 : Int;
          fee = 10_000;
          meta = ?{
            Issuer = "e8s - For Demonstration Purposes";
            Url = "https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md#_dfx_nns_install";
          };
        };
      };
      case (#ICRC1_ExampleToken T2) {
        return {
          symbol = "_1ICRC1EX";
          name = "Internet Computer Random Currency One Example Token";
          decimals = 8 : Int;
          fee = 10_000;
          meta = ?{
            Issuer = "This Token is For Demonstration Purposes Only";
            Url = "https://github.com/dfinity/ICRC-1";
          };
        };
      };
      case (#ICRC1_ExampleToken2 T2) {
        return {
          symbol = "_2ICRC1EX";
          name = "Two Internet Computer Random Currency One Example Token";
          decimals = 8 : Int;
          fee = 10_000;
          meta = ?{
            Issuer = "This Token is For Demonstration Purposes Only";
            Url = "https://github.com/dfinity/ic/tree/master/rs/rosetta-api/icrc1/ledger";
          };
        };
      };
    };
  };

  /****Returns the transaction fee as immutably defined in the `getTokenVerbose` for the given token type.***/
  public func getTransactionFee<T1, T2>(supportedToken : SupportedToken<T1, T2>) : Nat {
    // This method requires no modification when a
    // new tag is added when integrating a new token.
    let { fee } = getTokenVerbose(supportedToken);
    fee;
  };

  /****Returns the corresponding `UnitType` & amount in `Nat` base units for the given `SupportedToken.Amount`.***/
  public func unwrapTokenAmount(amount : Amount) : (UnitType, Nat) {
    switch amount {
      case (#ICP { e8s })(#ICP, Nat64.toNat(e8s));
      case (#ICP_nns { e8s })(#ICP_nns, Nat64.toNat(e8s));
      case (#ICRC1_ExampleToken tokens)(#ICRC1_ExampleToken, tokens);
      case (#ICRC1_ExampleToken2 tokens)(#ICRC1_ExampleToken2, tokens);
    };
  };

  /****Reforms the base unit amount into the given specific token's amount record type.***/
  public func wrapAsTokenAmount(token : UnitType, amount : Nat) : Amount {
    return switch token {
      case (#ICP) #ICP({ e8s = Nat64.fromNat(amount) });
      case (#ICP_nns) #ICP_nns({ e8s = Nat64.fromNat(amount) });
      case (#ICRC1_ExampleToken) #ICRC1_ExampleToken(amount);
      case (#ICRC1_ExampleToken2) #ICRC1_ExampleToken2(amount);
    };
  };

  /****Encodes a given address into text **without** validation.**  
    _For addresses computed by the invoice canister in a way **known** to be rigorously tested._  */
  public func encodeAddress(a : Address) : Text {
    switch a {
      case (#ICP accountIdentifier) ICP_Adapter.encodeAddress(accountIdentifier);
      case (#ICP_nns accountIdentifier) ICP_Adapter.encodeAddress(accountIdentifier);
      case (#ICRC1_ExampleToken account) ICRC1_Adapter.encodeAddress(account);
      case (#ICRC1_ExampleToken2 account) ICRC1_Adapter.encodeAddress(account);
    };
  };

  /****Encodes a given address into text **with** validation.**  
    _For addresses supplied by a caller that **are not** necessarily known to be valid._  */
  public func encodeAddressOrUnitErr(a : Address) : Result.Result<Text, ()> {
    switch a {
      case (#ICP accountIdentifier) {
        if (ICP_Adapter.isValidAddress(accountIdentifier)) {
          return #ok(encodeAddress(a));
        } else { #err };
      };
      case (#ICP_nns accountIdentifier) {
        if (ICP_Adapter.isValidAddress(accountIdentifier)) {
          return #ok(encodeAddress(a));
        } else { #err };
      };
      case (#ICRC1_ExampleToken account) {
        if (ICRC1_Adapter.isValidAddress(account)) {
          #ok(encodeAddress(a));
        } else { #err };
      };
      case (#ICRC1_ExampleToken2 account) {
        if (ICRC1_Adapter.isValidAddress(account)) {
          #ok(encodeAddress(a));
        } else { #err };
      };
    };
  };

  /****Returns an address decoded from given text or returns a given address if valid.**  
    **Validation occurs in either case.** Also note in either case whether text or address is  
    given the correct matching token type must be given as the passed `token` argument.  
    _For addresses supplied by a caller that **are not** necessarily known to be valid._  */
  public func getAddressOrUnitErr(
    token : UnitType,
    destination : RecipientAddress,
  ) : Result.Result<Address, ()> {
    switch destination {
      case (#HumanReadable addressAsText) {
        switch token {
          case (#ICP) {
            switch (ICP_Adapter.decodeAddress(addressAsText)) {
              case (#ok accountIdentifier) #ok(#ICP accountIdentifier);
              case (#err) #err;
            };
          };
          case (#ICP_nns) {
            switch (ICP_Adapter.decodeAddress(addressAsText)) {
              case (#ok accountIdentifier) #ok(#ICP_nns accountIdentifier);
              case (#err) #err;
            };
          };
          case (#ICRC1_ExampleToken) {
            switch (ICRC1_Adapter.decodeAddress(addressAsText)) {
              case (#ok account) #ok(#ICRC1_ExampleToken account);
              case (#err) #err;
            };
          };
          case (#ICRC1_ExampleToken2) {
            switch (ICRC1_Adapter.decodeAddress(addressAsText)) {
              case (#ok account) #ok(#ICRC1_ExampleToken2 account);
              case (#err) #err;
            };
          };
        };
      };
      case (#CanisterExpected supportedTokenAddress) {
        switch supportedTokenAddress {
          case (#ICP accountIdentifier) {
            let tokenTypeMatches = (token == #ICP);
            if (ICP_Adapter.isValidAddress(accountIdentifier) and tokenTypeMatches) {
              return #ok(#ICP accountIdentifier);
            } else #err;
          };
          case (#ICP_nns accountIdentifier) {
            let tokenTypeMatches = (token == #ICP_nns);
            if (ICP_Adapter.isValidAddress(accountIdentifier) and tokenTypeMatches) {
              return #ok(#ICP_nns accountIdentifier);
            } else #err;
          };
          case (#ICRC1_ExampleToken account) {
            let tokenTypeMatches = (token == #ICRC1_ExampleToken);
            if (ICRC1_Adapter.isValidAddress(account) and tokenTypeMatches) {
              return #ok(#ICRC1_ExampleToken account);
            } else #err;
          };
          case (#ICRC1_ExampleToken2 account) {
            let tokenTypeMatches = (token == #ICRC1_ExampleToken2);
            if (ICRC1_Adapter.isValidAddress(account) and tokenTypeMatches) {
              return #ok(#ICRC1_ExampleToken2 account);
            } else #err;
          };
        };
      };
    };
  };

  /****Returns the corresponding token address for an invoice subaccount.***/
  public func getInvoiceSubaccountAddress({
    token : UnitType;
    id : Text;
    creator : Principal;
    canisterId : Principal;
  }) : Address {
    switch token {
      case (#ICP) #ICP(ICP_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
      case (#ICP_nns) #ICP_nns(ICP_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
      case (#ICRC1_ExampleToken) #ICRC1_ExampleToken(ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
      case (#ICRC1_ExampleToken2) #ICRC1_ExampleToken2(ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
    };
  };

  /****Returns the corresponding token address encoded as text of the invoice subaccount address  
    computed from the given invoice id, creator's principal and invoice canister id.**   
    _Specifically used when creating an invoice._  */
  public func getEncodedInvoiceSubaccountAddress({
    token : UnitType;
    id : Text;
    creator : Principal;
    canisterId : Principal;
  }) : Text {
    switch token {
      case (#ICP) ICP_Adapter.encodeAddress(ICP_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
      case (#ICP_nns) ICP_Adapter.encodeAddress(ICP_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
      case (#ICRC1_ExampleToken) ICRC1_Adapter.encodeAddress(ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
      case (#ICRC1_ExampleToken2) ICRC1_Adapter.encodeAddress(ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
    };
  };

  /****Returns the corresponding token address for an invoice creator's subaccount.***/
  public func getCreatorSubaccountAddress({
    token : UnitType;
    creator : Principal;
    canisterId : Principal;
  }) : Address {
    switch token {
      case (#ICP) #ICP(ICP_Adapter.computeCreatorSubaccountAddress(creator, canisterId));
      case (#ICP_nns) #ICP_nns(ICP_Adapter.computeCreatorSubaccountAddress(creator, canisterId));
      case (#ICRC1_ExampleToken) #ICRC1_ExampleToken(ICRC1_Adapter.computeCreatorSubaccountAddress(creator, canisterId));
      case (#ICRC1_ExampleToken2) #ICRC1_ExampleToken2(ICRC1_Adapter.computeCreatorSubaccountAddress(creator, canisterId));
    };
  };

  /****Returns `TransferArgs` from invoice's subaccount to a specified destination address.**  
    **Note!** Fees must be subtracted **before** calling this method as `TransferArgs` can use their own Nat types.  
    **Note!** Validation of the input for the specified destination address must happen _before_ calling this method.  */
  public func getTransferArgsFromInvoiceSubaccount({
    to : Address;
    amountLessTheFee : Nat;
    fee : Nat;
    id : Text;
    creator : Principal;
  }) : TransferArgs {
    switch (to) {
      case (#ICP accountIdentifier) {
        #ICP {
          memo = 1;
          amount = { e8s = Nat64.fromNat(amountLessTheFee) };
          fee = { e8s = Nat64.fromNat(fee) };
          from_subaccount = ?ICP_Adapter.computeInvoiceSubaccount(id, creator);
          to = accountIdentifier;
          created_at_time = null;
        };
      };
      case (#ICP_nns accountIdentifier) {
        #ICP_nns {
          memo = 1;
          amount = { e8s = Nat64.fromNat(amountLessTheFee) };
          fee = { e8s = Nat64.fromNat(fee) };
          from_subaccount = ?ICP_Adapter.computeInvoiceSubaccount(id, creator);
          to = accountIdentifier;
          created_at_time = null;
        };
      };
      case (#ICRC1_ExampleToken account) {
        #ICRC1_ExampleToken {
          amount = amountLessTheFee;
          from_subaccount = ?ICRC1_Adapter.computeInvoiceSubaccount(id, creator);
          to = account;
          memo = ?Blob.fromArray([1]);
          fee = ?fee;
          created_at_time = null;
        };
      };
      case (#ICRC1_ExampleToken2 account) {
        #ICRC1_ExampleToken2 {
          amount = amountLessTheFee;
          from_subaccount = ?ICRC1_Adapter.computeInvoiceSubaccount(id, creator);
          to = account;
          memo = ?Blob.fromArray([1]);
          fee = ?fee;
          created_at_time = null;
        };
      };
    };
  };

  /****Returns `TransferArgs` from invoice creator's subaccount to a specified destination address.**  
    **Note!** Fees must be subtracted **before** calling this method as `TransferArgs` can use their own Nat types.  
    **Note!** Validation of the input for the specified destination address must happen _before_ calling this method.  */
  public func getTransferArgsFromCreatorSubaccount({
    to : Address;
    amountLessTheFee : Nat;
    fee : Nat;
    creator : Principal;
  }) : TransferArgs {
    switch to {
      case (#ICP accountIdentifier) {
        #ICP {
          memo = 1;
          amount = { e8s = Nat64.fromNat(amountLessTheFee) };
          fee = { e8s = Nat64.fromNat(fee) };
          from_subaccount = ?ICP_Adapter.computeCreatorSubaccount(creator);
          to = accountIdentifier;
          created_at_time = null;
        };
      };
      case (#ICP_nns accountIdentifier) {
        #ICP_nns {
          memo = 1;
          amount = { e8s = Nat64.fromNat(amountLessTheFee) };
          fee = { e8s = Nat64.fromNat(fee) };
          from_subaccount = ?ICP_Adapter.computeCreatorSubaccount(creator);
          to = accountIdentifier;
          created_at_time = null;
        };
      };
      case (#ICRC1_ExampleToken account) {
        #ICRC1_ExampleToken {
          amount = amountLessTheFee;
          from_subaccount = ?ICRC1_Adapter.computeCreatorSubaccount(creator);
          to = account;
          memo = ?Blob.fromArray([1]);
          fee = ?fee;
          created_at_time = null;
        };
      };
      case (#ICRC1_ExampleToken2 account) {
        #ICRC1_ExampleToken2 {
          amount = amountLessTheFee;
          from_subaccount = ?ICRC1_Adapter.computeCreatorSubaccount(creator);
          to = account;
          memo = ?Blob.fromArray([1]);
          fee = ?fee;
          created_at_time = null;
        };
      };
    };
  };

  /****Rewraps result types returned from corresponding token canisters' transfer calls.**  
    To that expected by the invoice canister caller. Note that before they are returned  
    to the caller, if they are of type `#Err` they are set as the `#err`   
    `SupportedTokenTransferErr` kind.  */
  public func rewrapTransferResults(sttransferResult : TransferResult) : Result.Result<TransferSuccess, TransferErr> {
    switch (sttransferResult) {
      case (#ICP transferResult) {
        switch transferResult {
          case (#Ok blockIndex) #ok(#ICP(blockIndex));
          case (#Err transferErr) #err(#ICP(transferErr));
        };
      };
      case (#ICP_nns transferResult) {
        switch transferResult {
          case (#Ok blockIndex) #ok(#ICP_nns(blockIndex));
          case (#Err transferErr) #err(#ICP_nns(transferErr));
        };
      };
      case (#ICRC1_ExampleToken transferResult) {
        switch transferResult {
          case (#Ok txIndex) #ok(#ICRC1_ExampleToken(txIndex));
          case (#Err transferErr) #err(#ICRC1_ExampleToken(transferErr));
        };
      };
      case (#ICRC1_ExampleToken2 transferResult) {
        switch transferResult {
          case (#Ok txIndex) #ok(#ICRC1_ExampleToken2(txIndex));
          case (#Err transferErr) #err(#ICRC1_ExampleToken2(transferErr));
        };
      };
    };
  };

  /****Computes the default subaccount for the address type given by `tokenType` for the given principal.***/
  public func getDefaultSubaccountAddress(
    tokenType : UnitType,
    p : Principal,
  ) : { asAddress : Address; asText : Text } {
    switch tokenType {
      case (#ICP) {
        let stAddress = #ICP(AccountIdentifierBlob.fromPrincipal(p, null));
        { asAddress = stAddress; asText = encodeAddress(stAddress) };
      };
      case (#ICP_nns) {
        let stAddress = #ICP_nns(AccountIdentifierBlob.fromPrincipal(p, null));
        { asAddress = stAddress; asText = encodeAddress(stAddress) };
      };
      case (#ICRC1_ExampleToken) {
        let stAddress = #ICRC1_ExampleToken({ owner = p; subaccount = null });
        { asAddress = stAddress; asText = encodeAddress(stAddress) };
      };
      case (#ICRC1_ExampleToken2) {
        let stAddress = #ICRC1_ExampleToken2({ owner = p; subaccount = null });
        { asAddress = stAddress; asText = encodeAddress(stAddress) };
      };
    };
  };
};
