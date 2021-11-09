import Array     "mo:base/Array";
import Blob      "mo:base/Blob";
import Nat8      "mo:base/Nat8";
import Nat32     "mo:base/Nat32";
import Principal "mo:base/Principal";
import Text      "mo:base/Text";
import CRC32     "./CRC32";
import SHA224    "./SHA224";

module {
  // 28-byte array.
  public type AccountIdentifier = Blob.Blob;
  // 32-byte array.
  public type Subaccount = Blob.Blob;
  // 32-byte array.
  public type Address = Blob.Blob;

  func beBytes(n: Nat32) : [Nat8] {
    func byte(n: Nat32) : Nat8 {
      Nat8.fromNat(Nat32.toNat(n & 0xff))
    };
    [byte(n >> 24), byte(n >> 16), byte(n >> 8), byte(n)]
  };

  public func defaultSubaccount() : Subaccount {
    Blob.fromArrayMut(Array.init(32, 0 : Nat8))
  };

  public func accountIdentifier(principal: Principal, subaccount: Subaccount) : AccountIdentifier {
    let hash = SHA224.Digest();
    hash.write([0x0A]);
    hash.write(Blob.toArray(Text.encodeUtf8("account-id")));
    hash.write(Blob.toArray(Principal.toBlob(principal)));
    hash.write(Blob.toArray(subaccount));
    Blob.fromArray(hash.sum())
  };

  public func address(principal: Principal, subaccount: Subaccount) : Address {
    let accountIdBytes = Blob.toArray(accountIdentifier(principal, subaccount));
    let crc32Bytes = beBytes(CRC32.ofArray(accountIdBytes));
    Blob.fromArray(Array.append(crc32Bytes, accountIdBytes))
  };

  public func validateAddress(address: Address) : ?AccountIdentifier {
    if (address.size() != 32) {
      return null;
    };
    let a = Blob.toArray(address);
    let accIdPart    = Array.tabulate(28, func(i: Nat): Nat8 { a[i + 4] });
    let checksumPart = Array.tabulate(4,  func(i: Nat): Nat8 { a[i] });
    let crc32 = CRC32.ofArray(accIdPart);
    if (Array.equal(beBytes(crc32), checksumPart, Nat8.equal)) {
      ?Blob.fromArray(accIdPart)
    } else {
      null
    }
  };
}
