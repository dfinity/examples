/// Certified variable — counter backed by `mo:core/CertifiedData`.
///
/// The canister holds a single 32-bit counter.  Every write also updates the
/// system-level certified-data hash so that query callers can cryptographically
/// verify the returned value without waiting for a full consensus round.
import CD "mo:core/CertifiedData";
import Blob "mo:core/Blob";
import Nat32 "mo:core/Nat32";

actor CertVar {

  var value : Nat32 = 0;

  /// Encode `n` as a 4-byte little-endian blob.
  /// LE matches the Candid encoding of Nat32, which lets the frontend decode
  /// the certified data directly with the Candid codec.
  func blobOfNat32(n : Nat32) : Blob {
    let byteMask : Nat32 = 0xff;
    func byte(x : Nat32) : Nat8 = x.toNat8();
    Blob.fromArray([
      byte(((byteMask << 0) & n) >> 0),
      byte(((byteMask << 8) & n) >> 8),
      byte(((byteMask << 16) & n) >> 16),
      byte(((byteMask << 24) & n) >> 24),
    ]);
  };

  /// Increment the counter by one, update the certificate, and return the new value.
  public func inc() : async Nat32 {
    value += 1;
    CD.set(blobOfNat32(value));
    value;
  };

  /// Set the counter to `newValue` and update the certificate.
  public func set(newValue : Nat32) : async () {
    value := newValue;
    CD.set(blobOfNat32(value));
  };

  /// Return the current counter value together with an optional system
  /// certificate.  The certificate is only present for query calls; update
  /// calls and inter-canister calls return `null` because the system already
  /// certifies those responses through consensus.
  public query func get() : async { value : Nat32; certificate : ?Blob } {
    { value; certificate = CD.getCertificate() };
  };
};
