import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Buffer "mo:base/Buffer";
import Nat8 "mo:base/Nat8";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Text "mo:base/Text";
import SHA224 "mo:crypto/SHA/SHA224";
import Binary "mo:encoding/Binary";
import CRC32 "mo:hash/CRC32";

import Types "./Types";
import AccountTextConverter "AccountTextConverter";

/****Set of addressing computations the invoice canister uses for processing ICRC1 transactions.***/
module Adapter {

  // These methods are left public so the generated docs can pick them up, however
  // they are used through importing the record literal at the end of this module
  // that contains the complete set of these methods.

  /****ICRC1 address type consisting of a record of an owner's principal and optional subaccount blob.***/
  public type Account = Types.Account;

  /****ICRC1 component address type used to map unique addresses to a single owner.**
    Is an arbitrary 32 byte sequence. The sequence of all zeros is known as the default subaccount. */
  public type Subaccount = Types.Subaccount;

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
        // No subaccount verify it's not only a reservered principal.
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
  public func encodeAddress(a : Types.Account) : Text {
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
    // Length of domain separator.
    hash.write([0x0A]);
    // Domain separator.
    hash.write(Blob.toArray(Text.encodeUtf8("invoice-id")));
    // Invoice id.
    hash.write(Blob.toArray(Text.encodeUtf8(id)));
    // Creator's principal.
    hash.write(Blob.toArray(Principal.toBlob(invoiceCreator)));
    let hashSum = hash.sum([]);
    // Crc32Bytes in the subaccount blob are not strictly required.
    let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
    // Create the subaccount blob.
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
    // Length of domain separator.
    hash.write([0x0A]);
    // Domain separator.
    hash.write(Blob.toArray(Text.encodeUtf8("creator-id")));
    // Creator's principal.
    hash.write(Blob.toArray(Principal.toBlob(p)));
    let hashSum = hash.sum([]);
    // Crc32Bytes in the subaccount blob are not strictly required.
    let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
    // Create the subaccount blob.
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

  /****Record literal of the ICRC1 Adapter's set of methods.**  
    The above methods could be made private to protect them and then
    accessed through importing this record.  */
  public let icrc1Adapter = {
    isValidSubaccount;
    isValidAddress;
    encodeAddress;
    decodeAddress;
    computeInvoiceSubaccount;
    computeInvoiceSubaccountAddress;
    computeCreatorSubaccount;
    computeCreatorSubaccountAddress;
  };
};
