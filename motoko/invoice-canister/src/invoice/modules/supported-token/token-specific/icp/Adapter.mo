import Array "mo:base/Array";
import Blob "mo:base/Blob";
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

import Types "./Types";

/****Set of addressing computations the invoice canister uses for processing ICP transactions.***/
module Adapter {

  /****ICP address type computed from a principal and an optional subaccount blob.***/
  public type AccountIdentifier = Types.AccountIdentifier;

  /****ICP component address type used to map unique addresses to a single owner.**
    Is an arbitrary 32 byte sequence. The sequence of all zeros is known as the default subaccount. */
  public type Subaccount = Types.Subaccount;

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
    // Create the subaccount blob.
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
    // Create the subaccount blob.
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

  /****Record literal of the ICP Adapter's set of methods.**  
    The above methods could be made private to protect them and then
    accessed through importing this record.  */
  public let icpAdapter = {
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
