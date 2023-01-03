// imports from Dfinity Motoko Base package
import Array                        "mo:base/Array";
import Blob                         "mo:base/Blob";
import Buffer                       "mo:base/Buffer";
import Nat32                        "mo:base/Nat32";
import Nat8                         "mo:base/Nat8";
import Principal                    "mo:base/Principal";
import Text                         "mo:base/Text";
import Result                       "mo:base/Result";

// imports from Internet Computer Open Services
import SHA224                       "mo:crypto/SHA/SHA224";
import SHA2                         "mo:crypto/SHA/SHA2";
import Binary                       "mo:encoding/Binary";
import CRC32                        "mo:hash/CRC32";
import Hex                          "mo:encoding/Hex";
import AccountIdentifierBlob        "mo:principal/blob/AccountIdentifier";

// For preparing ICP specific invoices and transactions
module {

/**
* args : { invoiceId : Nat, invoiceCreator  : Principal }
* Takes the id and creator's principal and returns the blob form of the subaccount for that invoice.
* Note this is what's passed to the (from) subaccount field when calling the transfer method of the ICP ledger and is not a full account identifier.
*/
  public func subaccountForInvoice(invoiceId : Nat, invoiceCreator : Principal) : Blob {
    let hash = SHA224.New();
    // Length of domain separator
    hash.write([0x0A]);
    // Domain separator
    hash.write(Blob.toArray(Text.encodeUtf8("invoice-id")));
    // Counter as Nonce
    let idBytes = Binary.BigEndian.fromNat32(Nat32.fromNat(invoiceId));
    hash.write(idBytes);
    // Principal of caller
    hash.write(Blob.toArray(Principal.toBlob(invoiceCreator)));
    let hashSum = hash.sum([]);
    let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
    Blob.fromArray(Array.flatten<Nat8>([crc32bytes, hashSum]));
  };

/**
* args : { p : Principal }
* Takes a principal and returns the blob form of the subaccount for that principal.
* Note this is what's passed to the (from) subaccount field when calling the transfer method of the ICP ledger and is not a full account identifier.
*/
  public func subaccountForPrincipal(p : Principal) : Blob {
    let hash = SHA224.New();
    /* adding this resolves SEC-F27 TODO future commit
    hash.write([0x0A]);
    hash.write(Blob.toArray(Text.encodeUtf8("creator-id"))); 
    */
    hash.write(Blob.toArray(Principal.toBlob(p)));
    let hashSum = hash.sum([]);
    let crc32bytes = Binary.BigEndian.fromNat32(CRC32.checksum(hashSum));
    Blob.fromArray(Array.flatten<Nat8>([crc32bytes, hashSum]));
  };

/**
* args : { p : Principal, subaccount : Blob }
* Note subaccount arg is not validated as this method is to be used in conjuction with subaccountForInvoice and subaccountForPrincipal.
* Takes a principal and a subaccount and returns the blob form of the corresponding full account identifier address.
* When the invoice canister id and the invoice subaccount returned from subaccountForInvoice is passed in, this returns the address to which ICP can be transfered to complete payment for an invoice.
* When the invoice canister id and the principal subaccount returned from subaccountForPrincipal is passed in, this returns the address whose balance will reflect the proceeds of all invoices created by that principal, that have been paid for and successfully verified (minus the transfer fee times the number of invoices).
* Note this is what's passed to the to field when calling the transfer method of the ICP ledger. 
*/
  public func toAccountIdentifierAddress(p : Principal, subaccount : Blob) : Blob {
    AccountIdentifierBlob.fromPrincipal(p, ?Blob.toArray(subaccount));
  };

/**
* args : { b : Blob }
* Takes a blob and returns it in text form.
* For converting the blobs of account identifiers or subaccounts into their human readable form.
* Note does not check if the produced blob is a valid account identifier, use accountIdentifierIsValid if that is needed.
*/
  public func toHumanReadableForm(b : Blob) : Text { Hex.encode(Blob.toArray(b)) };

/**
* args : { accountIdentifier : Blob }
* Takes the blob form of a full account identifier and returns true or false if it is valid (of correct length and has correct CRC32 hash).
*/
  public func accountIdentifierIsValid(accountIdentifier : Blob) : Bool {
    if (accountIdentifier.size() != 32) { return false };
    let a = Blob.toArray(accountIdentifier);
    let accIdPart = Array.tabulate(28, func(i : Nat) : Nat8 { a[i + 4] });
    let checksumPart = Array.tabulate(4, func(i : Nat) : Nat8 { a[i] });
    let crc32 = CRC32.checksum(accIdPart);
    Array.equal(Binary.BigEndian.fromNat32(crc32), checksumPart, Nat8.equal);
  };

/**
* args : { accountId : Text }
* Takes the textual form of an account identifier and returns its blob form or error if input is an invalid account identifier.
*/
  public func accountIdentifierFromValidText(accountId : Text) : Result.Result<Blob, Text> {
    switch (AccountIdentifierBlob.fromText(accountId)) {
      case (#ok(aid)) {
        if (accountIdentifierIsValid(aid)) { 
          return #ok(aid); 
        } 
        else {
          return #err("Textual account identifier was invalid: CRC32 hash did not match.");
        };
      };
      case _ { /* proceed to return error */ };
    };
    return #err("Textual account identifier was not valid account identifier.");
  };
}