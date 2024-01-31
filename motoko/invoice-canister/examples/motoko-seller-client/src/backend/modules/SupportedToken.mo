// Note about how to edit this file to add support for additional ICRC1 token-ledger canisters:
/** If adding more ICRC1 token-ledger canisters, the **only** edits necessary in this file are the methods 
  declared after the `SupportedToken<T1, T2>` variant (ie in the `SupportedToken` module's outermost 
  scope). Add a new tag in this variant for each additional ICRC1 token-ledger canister to support, and 
  then update the corresponding switches in the methods that follow until all the cases are covered. The 
  only other changes necessary are the corresponding switch cases to be added in `Invoice.mo`. 
  Note that once the variant tag is added to `SupportedToken<T1, T2>` , this will cause the Motoko 
  VScode extension to indicate where all switches need to be updated to include the required cases 
  for supporting that token.\
  Also note the `Supertype_ICRC1_Actor` type declaration can be reused in `Invoice.mo` as different
  instances to perform the corresponding intercanister calls to the token-ledger canisters of
  the additional ICRC1 tokens to support.          
  See main's project files and readme for explanation. */

import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Buffer "mo:base/Buffer";
import Nat64 "mo:base/Nat64";
import Nat8 "mo:base/Nat8";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Text "mo:base/Text";
import Time "mo:base/Time";
import SHA224 "mo:crypto/SHA/SHA224";
import Binary "mo:encoding/Binary";
import Hex "mo:encoding/Hex";
import CRC32 "mo:hash/CRC32";
import AccountIdentifierBlob "mo:principal/blob/AccountIdentifier";

/****Core module composing the uniform interface between types adapting token-ledger canister 
  actors to types processed for and expected by the invoice canister caller.***/
module SupportedToken {

  /****Types and functions specific to either those of an ICP or ICRC1 token-ledger canisters  
    necessary for the functionality of `SupportedToken`.**  
    _Note that the actor supertypes are not strictly necessary but are included for consistency_.  
    _Also note both `Adapter`s are private to their respective module, but their methods are  
    redeclared as the functions of the public record types `icpAdapter` and `icrc1Adapter` at  
    the end of either module for convenience simple importing._  */
  module TokenSpecific {

    /****ICP ledger canister related variable types, adapter and supertype.**  
      _**Note** both the adapter and supertype are dependent on these type declarations._  */
    public module ICP {

      // #region ICP module scoped fields copied from the ICP ledger canister's public did.
      public type AccountIdentifier = Blob;
      public type Subaccount = Blob;
      public type Tokens = { e8s : Nat64 };
      public type Timestamp = { timestamp_nanos : Nat64 };
      public type Memo = Nat64;
      public type BlockIndex = Nat64;
      public type Hash = Blob;
      public type SenderArgs = AccountBalanceArgs;
      public type AccountBalanceArgs = { account : AccountIdentifier };
      public type TransferArgs = {
        memo : Memo;
        amount : Tokens;
        fee : Tokens;
        from_subaccount : ?Subaccount;
        to : AccountIdentifier;
        created_at_time : ?Timestamp;
      };
      public type Result<T, E> = { #Ok : T; #Err : E };
      public type TransferResult = Result<BlockIndex, TransferError>;
      public type TransferError = {
        #BadFee : { expected_fee : Tokens };
        #InsufficientFunds : { balance : Tokens };
        #TxTooOld : { allowed_window_nanos : Nat64 };
        #TxCreatedInFuture;
        #TxDuplicate : { duplicate_of : BlockIndex };
      };
      // #endregion

      /****ICP ledger canister supertype actor included for consistency.***/
      public module Supertype {
        public type Actor = actor {
          transfer : shared TransferArgs -> async TransferResult;
          account_balance : shared query AccountBalanceArgs -> async Tokens;
        };
      };

      /****Set of addressing computations the invoice canister uses for processing ICP transactions.***/
      module Adapter {

        /****Returns whether it is a valid account identifier's (the optional) subaccount.***/
        public func isValidSubaccount(s : Subaccount) : Bool {
          (s.size() == 32);
        };

        /****Returns whether it is a valid account identifier.***/
        public func isValidAddress(a : AccountIdentifier) : Bool {
          if (a.size() != 32) { return false };
          let arr = Blob.toArray(a);
          let accIdPart = Array.tabulate(28, func(i : Nat) : Nat8 { arr[i + 4] });
          let checksumPart = Array.tabulate(4, func(i : Nat) : Nat8 { arr[i] });
          let crc32 = CRC32.checksum(accIdPart);
          Array.equal(Binary.BigEndian.fromNat32(crc32), checksumPart, Nat8.equal);
        };

        /****Encodes given valid account identifier (no validation performed).***/
        public func encodeAddress(a : AccountIdentifier) : Text {
          Hex.encode(Blob.toArray(a));
        };

        /****Decodes given text into an account identifier if valid or if not returns the unit err.***/
        public func decodeAddress(t : Text) : Result.Result<AccountIdentifier, ()> {
          switch (AccountIdentifierBlob.fromText(t)) {
            case (#ok(accountIdentifier)) {
              if (isValidAddress(accountIdentifier)) return #ok(accountIdentifier); //
              // else the CRC32 hash did not correctly match its expected value.
            };
            case _ { /* proceed to return error */ };
          };
          #err;
        };

        /****Computes an invoice's subaccount blob from the given invoice id and creator's principal.***/
        public func computeInvoiceSubaccount(id : Text, p : Principal) : Subaccount {
          let hash = SHA224.New();
          // Length of domain separator.
          hash.write([0x0A]);
          // Domain separator.
          hash.write(Blob.toArray(Text.encodeUtf8("invoice-id")));
          // Invoice id.
          hash.write(Blob.toArray(Text.encodeUtf8(id)));
          // Creator's principal.
          hash.write(Blob.toArray(Principal.toBlob(p)));
          let hashSum = hash.sum([]);
          // Crc32Bytes in the subaccount blob are not strictly required.
          let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
          Blob.fromArray(Array.flatten<Nat8>([crc32bytes, hashSum]));
        };

        /****Computes an invoice's subaccount account identifier from the given 
          invoice id, creator's principal and invoice canister id.***/
        public func computeInvoiceSubaccountAddress(
          id : Text,
          creator : Principal,
          canisterId : Principal,
        ) : AccountIdentifier {
          AccountIdentifierBlob.fromPrincipal(
            canisterId,
            ?Blob.toArray(computeInvoiceSubaccount(id, creator)),
          );
        };

        /****Computes an invoice creator's subaccount blob from their given principal.***/
        public func computeCreatorSubaccount(p : Principal) : Subaccount {
          let hash = SHA224.New();
          // Length of domain separator.
          hash.write([0x0A]);
          // Domain separator.
          hash.write(Blob.toArray(Text.encodeUtf8("creator-id"))); //
          // Principal of creator.
          hash.write(Blob.toArray(Principal.toBlob(p)));
          let hashSum = hash.sum([]);
          // CRC32 bytes are required for valid account identifiers.
          let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
          Blob.fromArray(Array.flatten<Nat8>([crc32bytes, hashSum]));
        };

        /****Computes an invoice creator's subaccount account identifier  
          from their given principal and invoice canister id.***/
        public func computeCreatorSubaccountAddress(
          creator : Principal,
          canisterId : Principal,
        ) : AccountIdentifier {
          AccountIdentifierBlob.fromPrincipal(
            canisterId,
            ?Blob.toArray(computeCreatorSubaccount(creator)),
          );
        };
      };

      /****Externally accessible "record literal" of ICP `Adapter`***/
      public let icpAdapter = {
        isValidSubaccount = Adapter.isValidSubaccount;
        isValidAddress = Adapter.isValidAddress;
        encodeAddress = Adapter.encodeAddress;
        decodeAddress = Adapter.decodeAddress;
        computeInvoiceSubaccount = Adapter.computeInvoiceSubaccount;
        computeInvoiceSubaccountAddress = Adapter.computeInvoiceSubaccountAddress;
        computeCreatorSubaccount = Adapter.computeCreatorSubaccount;
        computeCreatorSubaccountAddress = Adapter.computeCreatorSubaccountAddress;
      };
    };

    /****ICRC1 ledger canister related variable types, adapter and supertype.**  
      _**Note** both the adapter and supertype are dependent on these type declarations._  */
    public module ICRC1 {

      // #region ICP module scoped fields copied from the ICRC1 token-ledger canister's public did.
      public type Account = { owner : Principal; subaccount : ?Subaccount };
      public type Subaccount = Blob;
      public type Memo = Blob;
      public type Timestamp = Nat64;
      public type Duration = Nat64;
      public type TxIndex = Nat;
      public type Tokens = Nat;
      public type BalanceArgs = Account;
      public type TransferArgs = {
        from_subaccount : ?Subaccount;
        to : Account;
        amount : Tokens;
        fee : ?Tokens;
        memo : ?Memo;
        created_at_time : ?Timestamp;
      };
      public type Result<T, E> = { #Ok : T; #Err : E };
      public type TransferResult = Result<Tokens, TransferError>;
      public type CommonFields = {
        memo : ?Memo;
        fee : ?Tokens;
        created_at_time : ?Timestamp;
      };
      public type DeduplicationError = {
        #TooOld;
        #Duplicate : { duplicate_of : TxIndex };
        #CreatedInFuture : { ledger_time : Timestamp };
      };
      public type CommonError = {
        #InsufficientFunds : { balance : Tokens };
        #BadFee : { expected_fee : Tokens };
        #TemporarilyUnavailable;
        #GenericError : { error_code : Nat; message : Text };
      };
      public type TransferError = DeduplicationError or CommonError or {
        // In case the invoice canister would be used to directly manage a token-ledger canister.
        #BadBurn : { min_burn_amount : Tokens };
      };
      // #endregion

      /****ICRC1 token-ledger canister supertype actor included for consistency.***/
      public module Supertype {
        public type Actor = actor {
          icrc1_transfer : shared TransferArgs -> async TransferResult;
          icrc1_balance_of : shared query BalanceArgs -> async Tokens;
        };
      };

      /****Set of addressing computations the invoice canister uses for processing ICRC1 transactions.***/
      module Adapter {

        /****Returns whether it is a valid icrc1 account's subaccount.***/
        public func isValidSubaccount(s : Subaccount) : Bool {
          (s.size() == 32);
        };

        /****Returns whether it is a valid icrc1 account.***/
        public func isValidAddress(a : Account) : Bool {
          if (Principal.isAnonymous(a.owner)) {
            return false;
          };
          switch (a.subaccount) {
            case (null) {
              // No subaccount so verify it's not only a reserved principal.
              let pbArr = Blob.toArray(Principal.toBlob(a.owner));
              if (pbArr[pbArr.size() - 1] == 127) {
                // Ends in x7F and thus is a reserved principal, so it is required
                // to have a non-trivial subaccount to be a valid icrc1 account.
                return false;
              };
            };
            case (?blob) { return isValidSubaccount(blob) };
          };
          true;
        };

        /****Encodes given icrc1 account as text (no validation performed).***/
        public func encodeAddress(a : Account) : Text {
          AccountTextConverter.toText(a);
        };

        /****Decodes given text into an icrc1 account if valid or if not returns the unit err.***/
        public func decodeAddress(t : Text) : Result.Result<Account, ()> {
          if (not simpleAccountTextInputCheck(t)) {
            return #err();
          };
          switch (AccountTextConverter.fromText(t)) {
            case (#ok account) #ok(account);
            case (#err err) {
              switch err {
                case (#bad_length) #err();
                case (#not_canonical) #err();
              };
            };
          };
        };

        /****Computes an invoice's subaccount blob from the given invoice id and creator's principal.***/
        public func computeInvoiceSubaccount(id : Text, invoiceCreator : Principal) : Subaccount {
          let hash = SHA224.New();
          hash.write([0x0A]);
          hash.write(Blob.toArray(Text.encodeUtf8("invoice-id")));
          hash.write(Blob.toArray(Text.encodeUtf8(id)));
          hash.write(Blob.toArray(Principal.toBlob(invoiceCreator)));
          let hashSum = hash.sum([]);
          let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
          Blob.fromArray(Array.flatten<Nat8>([crc32bytes, hashSum]));
        };

        /****Computes an invoice's subaccount icrc1 account from the given 
          invoice id, creator's principal and invoice canister id.***/
        public func computeInvoiceSubaccountAddress(
          id : Text,
          creator : Principal,
          canisterId : Principal,
        ) : Account {
          {
            owner = canisterId;
            subaccount = ?(
              computeInvoiceSubaccount(
                id,
                creator,
              ),
            );
          };
        };

        /****Computes an invoice creator's subaccount blob from their given principal.***/
        public func computeCreatorSubaccount(p : Principal) : Subaccount {
          let hash = SHA224.New();
          hash.write([0x0A]);
          hash.write(Blob.toArray(Text.encodeUtf8("creator-id")));
          hash.write(Blob.toArray(Principal.toBlob(p)));
          let hashSum = hash.sum([]);
          let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
          Blob.fromArray(Array.flatten<Nat8>([crc32bytes, hashSum]));
        };

        /****Computes an invoice creator's subaccount icrc1 account from their given principal and invoice canister id.***/
        public func computeCreatorSubaccountAddress(creator : Principal, canisterId : Principal) : Account {
          {
            owner = canisterId;
            subaccount = ?(computeCreatorSubaccount(creator));
          };
        };

        /** Prevents unsanitized text input causing `Principal.fromText` to trap when decoding  
          the given text into an icrc1 account. Returns false if any character in the Text's  
          string isn't an alphanumeric character except for every sixth character which must  
          be a dash which also cannot be the last character of the string.  */
        func simpleAccountTextInputCheck(t : Text) : Bool {
          // Minimum principal length with crc and last char not being '-'.
          if (t.size() < 6) return false;
          // Can't end in a dash, only dash may be at this char slot.
          if (t.size() % 6 == 0) return false;
          var count = 1;
          for (c in t.chars()) {
            if (not isAlphaNumericChar(c)) {
              if (count % 6 == 0 and c == '-') {
                // Dash is where it's supposed to be.
              } else {
                return false;
              };
            };
            count += 1;
          };
          true;
        };

        /** Returns whether the given char is (a-z | A-Z | 0-9) or not. */
        func isAlphaNumericChar(c : Char) : Bool {
          // Returns `true` or `false` if the `char` is (a-z or A-Z or 0-9) or not.
          switch c {
            case ('a' or 'A') true;
            case ('b' or 'B') true;
            case ('c' or 'C') true;
            case ('d' or 'D') true;
            case ('e' or 'E') true;
            case ('f' or 'F') true;
            case ('g' or 'G') true;
            case ('h' or 'H') true;
            case ('i' or 'I') true;
            case ('j' or 'J') true;
            case ('k' or 'K') true;
            case ('l' or 'L') true;
            case ('m' or 'M') true;
            case ('n' or 'N') true;
            case ('o' or 'O') true;
            case ('p' or 'P') true;
            case ('q' or 'Q') true;
            case ('r' or 'R') true;
            case ('s' or 'S') true;
            case ('t' or 'T') true;
            case ('u' or 'U') true;
            case ('v' or 'V') true;
            case ('w' or 'W') true;
            case ('x' or 'X') true;
            case ('y' or 'Y') true;
            case ('z' or 'Z') true;
            case ('0') true;
            case ('1') true;
            case ('2') true;
            case ('3') true;
            case ('4') true;
            case ('5') true;
            case ('6') true;
            case ('7') true;
            case ('8') true;
            case ('9') true;
            case _ false;
          };
        };

        /****!!Important!! Should be updated if ICRC1 Accounting encoding/decoding spec changes.**    
          Encodes an ICRC1 account into its text form and decodes valid text into an ICRC1 account.  
          Copied from code of the Ledger and Tokenization Working Group.  */
        module AccountTextConverter {
          public type Account = { owner : Principal; subaccount : ?Blob };
          public type DecodeError = {
            // Subaccount length is invalid.
            #bad_length;
            // The subaccount encoding is not canonical.
            #not_canonical;
          };

          public func toText(acc : Account) : Text {
            switch (acc.subaccount) {
              case (null) { Principal.toText(acc.owner) };
              case (?blob) {
                assert (blob.size() == 32);
                var zeroCount = 0;
                label l for (byte in blob.vals()) {
                  if (byte == 0) { zeroCount += 1 } else break l;
                };
                if (zeroCount == 32) {
                  Principal.toText(acc.owner);
                } else {
                  let principalBytes = Principal.toBlob(acc.owner);
                  let buf = Buffer.Buffer<Nat8>(principalBytes.size() + blob.size() - zeroCount + 2);
                  for (b in principalBytes.vals()) {
                    buf.add(b);
                  };
                  var j = 0;
                  label l for (b in blob.vals()) {
                    j += 1;
                    if (j <= zeroCount) {
                      continue l;
                    };
                    buf.add(b);
                  };
                  buf.add(Nat8.fromNat(32 - zeroCount));
                  buf.add(Nat8.fromNat(0x7f));
                  Principal.toText(Principal.fromBlob(Blob.fromArray(Buffer.toArray(buf))));
                };
              };
            };
          };

          public func fromText(text : Text) : Result.Result<Account, DecodeError> {
            let principal = Principal.fromText(text);
            let bytes = Blob.toArray(Principal.toBlob(principal));
            if (bytes.size() == 0 or bytes[bytes.size() - 1] != Nat8.fromNat(0x7f)) {
              return #ok({ owner = principal; subaccount = null });
            };
            if (bytes.size() == 1) {
              return #err(#bad_length);
            };
            let n = Nat8.toNat(bytes[bytes.size() - 2]);
            if (n == 0) {
              return #err(#not_canonical);
            };
            if (n > 32 or bytes.size() < n + 2) {
              return #err(#bad_length);
            };
            if (bytes[bytes.size() - n - 2] == Nat8.fromNat(0)) {
              return #err(#not_canonical);
            };
            let zeroCount = 32 - n : Nat;
            let subaccount = Blob.fromArray(
              Array.tabulate(
                32,
                func(i : Nat) : Nat8 {
                  if (i < zeroCount) { Nat8.fromNat(0) } else {
                    bytes[bytes.size() - n - 2 + i - zeroCount];
                  };
                },
              ),
            );
            let owner = Blob.fromArray(Array.tabulate((bytes.size() - n - 2) : Nat, func(i : Nat) : Nat8 { bytes[i] }));
            #ok({ owner = Principal.fromBlob(owner); subaccount = ?subaccount });
          };
        };
      };

      /****Externally accessible "record literal" of ICRC1 `Adapter`***/
      public let icrc1Adapter = {
        isValidSubaccount = Adapter.isValidSubaccount;
        isValidAddress = Adapter.isValidAddress;
        encodeAddress = Adapter.encodeAddress;
        decodeAddress = Adapter.decodeAddress;
        computeInvoiceSubaccount = Adapter.computeInvoiceSubaccount;
        computeInvoiceSubaccountAddress = Adapter.computeInvoiceSubaccountAddress;
        computeCreatorSubaccount = Adapter.computeCreatorSubaccount;
        computeCreatorSubaccountAddress = Adapter.computeCreatorSubaccountAddress;
      };
    };
  };

  // These two redeclared to make accessible outside of `SupportedToken`:
  public type Supertype_ICP_Actor = TokenSpecific.ICP.Supertype.Actor;
  public type Supertype_ICRC1_Actor = TokenSpecific.ICRC1.Supertype.Actor;

  // These two used internally in `SupportedToken`,
  // (but made public to be available in unit testing):
  public let { icpAdapter = ICP_Adapter } = TokenSpecific.ICP;
  public let { icrc1Adapter = ICRC1_Adapter } = TokenSpecific.ICRC1;

  /****Some type that makes the rest possible.**  
    Entries of this variant are all the token-ledger canisters this invoice canister supports. The generic arguments  
    are the distinct types each token uses and may be shared: additional ICRC1 standard tokens can reuse the generic  
    type T2. Adding or removing an entry will trigger the VSCode Motoko extension to indicate all the methods' switches'  
    cases that need to be edited to add or remove support for a particular ICP or ICRC1 standard token (in this file,  
    it is only the methods below this declaration; there are several in `Invoice.mo` as well).  */
  public type SupportedToken<T1, T2> = {
    #ICP : T1;
    #ICRC1 : T2;
  };

  /** Corresponding supported token "base" types. */
  public type UnitType = SupportedToken<(), ()>;

  /** Corresponding supported token type canister expected amount types. */
  public type Amount = SupportedToken<TokenSpecific.ICP.Tokens, TokenSpecific.ICRC1.Tokens>;

  /** Corresponding supported token type canister expected address types. */
  public type Address = SupportedToken<TokenSpecific.ICP.AccountIdentifier, TokenSpecific.ICRC1.Account>;

  /** Corresponding supported token type canister expected TransferArgs types. */
  public type TransferArgs = SupportedToken<TokenSpecific.ICP.TransferArgs, TokenSpecific.ICRC1.TransferArgs>;

  /** Corresponding supported token type canister expected TransferResult types. */
  public type TransferResult = SupportedToken<TokenSpecific.ICP.TransferResult, TokenSpecific.ICRC1.TransferResult>;

  /** Corresponding supported token type canister expected TransferSuccess types. */
  public type TransferSuccess = SupportedToken<TokenSpecific.ICP.BlockIndex, TokenSpecific.ICRC1.TxIndex>;

  /** Corresponding supported token type canister expected TransferErr types. */
  public type TransferErr = SupportedToken<TokenSpecific.ICP.TransferError, TokenSpecific.ICRC1.TransferError>;

  /****Sum type for converting between human parsable and canister expected address types.***/
  public type RecipientAddress = {
    #HumanReadable : Text;
    #CanisterExpected : Address;
  };

  /****Hard coded token's information record.**  
    At least the fee must be correctly defined for additional tokens in `getTokenVerbose` below.  */
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
          symbol = "_MICP";
          name = "_Internet Computer Protocol Token Seller Example Edition";
          decimals = 8 : Int;
          fee = 10_000;
          meta = ?{
            Issuer = "e8s - For Demonstration Purposes";
            Url = "https//internetcomputer.org";
          };
        };
      };
      case (#ICRC1 T2) {
        return {
          symbol = "_MICRC1";
          name = "_Internet Computer Random Currency One Example Token Seller Example Edition";
          decimals = 8 : Int;
          fee = 10_000;
          meta = ?{
            Issuer = "This Token is For Demonstration Purposes Only";
            Url = "https//selleredition.shoppingonchain.org";
          };
        };
      };
    };
  };

  // All switches in the following methods that use one of the above `SupportedToken` types must have the case added
  // that corresponds to the variant of the added ICRC1 token to support. The same is true for those switches in
  // `Invoice.mo`. As stated earlier, once the new variant tag is added, the Motoko VSCode extension will indicate
  // each and all switches that need an added case for that variant tag to add support for the corresponding token.

  /****Returns the transaction fee as immutably defined in the `getTokenVerbose` for the given token type.***/
  public func getTransactionFee<T1, T2>(supportedToken : SupportedToken<T1, T2>) : Nat {
    let { fee } = getTokenVerbose(supportedToken);
    fee;
  };

  /****Returns the corresponding `UnitType` & amount in `Nat` base units for the given `SupportedToken.Amount`.***/
  public func unwrapTokenAmount(amount : Amount) : (UnitType, Nat) {
    switch amount {
      case (#ICP { e8s })(#ICP, Nat64.toNat(e8s));
      case (#ICRC1 tokens)(#ICRC1, (tokens));
    };
  };

  /****Reforms the base unit amount into the given specific token's amount record type.***/
  public func wrapAsTokenAmount(token : SupportedToken.UnitType, amount : Nat) : SupportedToken.Amount {
    return switch token {
      case (#ICP) #ICP({ e8s = Nat64.fromNat(amount) });
      case (#ICRC1) #ICRC1(amount);
    };
  };

  /****Encodes a given address into text **without** validation.**  
    _For addresses computed by the invoice canister in a way **known** to be rigorously tested._  */
  public func encodeAddress(a : Address) : Text {
    switch a {
      case (#ICP accountIdentifier) ICP_Adapter.encodeAddress(accountIdentifier);
      case (#ICRC1 account) ICRC1_Adapter.encodeAddress(account);
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
      case (#ICRC1 account) {
        if (ICRC1_Adapter.isValidAddress(account)) {
          #ok(encodeAddress(a));
        } else { #err };
      };
    };
  };

  /****Returns an address decoded from given text or returns a given address if valid.**  
    **Validation occurs in either case.** Also note in either case whether text or address is  
    given the correct matching token type must also be given as the passed `token` argument.  
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
          case (#ICRC1) {
            switch (ICRC1_Adapter.decodeAddress(addressAsText)) {
              case (#ok account) #ok(#ICRC1 account);
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
          case (#ICRC1 account) {
            let tokenTypeMatches = (token == #ICRC1);
            if (ICRC1_Adapter.isValidAddress(account) and tokenTypeMatches) {
              return #ok(#ICRC1 account);
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
      case (#ICRC1) #ICRC1(ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
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
      case (#ICRC1) ICRC1_Adapter.encodeAddress(ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
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
      case (#ICRC1) #ICRC1(ICRC1_Adapter.computeCreatorSubaccountAddress(creator, canisterId));
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
      case (#ICRC1 account) {
        #ICRC1 {
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
      case (#ICRC1 account) {
        #ICRC1 {
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
      case (#ICRC1 transferResult) {
        switch transferResult {
          case (#Ok txIndex) #ok(#ICRC1(txIndex));
          case (#Err transferErr) #err(#ICRC1(transferErr));
        };
      };
    };
  };

  /****Computes the default subaccount for the address type given by `tokenType` for the given principal.***/
  public func getDefaultSubaccountAddress(
    tokenType : SupportedToken.UnitType,
    p : Principal,
  ) : { asAddress : SupportedToken.Address; asText : Text } {
    switch tokenType {
      case (#ICP) {
        let stAddress = #ICP(AccountIdentifierBlob.fromPrincipal(p, null));
        { asAddress = stAddress; asText = encodeAddress(stAddress) };
      };
      case (#ICRC1) {
        let stAddress = #ICRC1({ owner = p; subaccount = null });
        { asAddress = stAddress; asText = encodeAddress(stAddress) };
      };
    };
  };
};
