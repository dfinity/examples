/// https://github.com/flyq/ic_codec/blob/8a6f2fff88758125fd7e3ce54f99366f9c121eda/src/base32.mo
/// This library lets you encode and decode in either RFC4648 Base32 or in Crockford Base32.

import Array "mo:base/Array";
import Iter "mo:base/Iter";
import Text "mo:base/Text";
import Char "mo:base/Char";
import Nat8 "mo:base/Nat8";
import Int8 "mo:base/Int8";
import Nat "mo:base/Nat";
import Nat32 "mo:base/Nat32";
import Int "mo:base/Int";

// refers: https://docs.rs/crate/base32/0.4.0/source/src/lib.rs
module {
  let RFC4648_ALPHABET: [Nat8]= [65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 50, 51, 52, 53, 54, 55]; // b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
  let CROCKFORD_ALPHABET: [Nat8] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 74, 75, 77, 78, 80, 81, 82, 83, 84, 86, 87, 88, 89, 90]; // b"0123456789ABCDEFGHJKMNPQRSTVWXYZ"
  let RFC4648_INV_ALPHABET: [Int8] = [-1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, 0, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
  let CROCKFORD_INV_ALPHABET: [Int8] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, 16, 17, 1, 18, 19, 1, 20, 21, 0, 22, 23, 24, 25, 26, -1, 27, 28, 29, 30, 31];

  /// RFC4648 Base32 or Crockford Base32
  public type Alphabet = {
    #RFC4648: { padding: Bool; };
    #Crockford;
  };

  /// encode the bytes
  public func encode(alphabet: Alphabet, data: [Nat8]) : Text {
    let (alpha, padding) = switch alphabet {
      case (#RFC4648 { padding }) { (RFC4648_ALPHABET, padding); };
      case (#Crockford) { (CROCKFORD_ALPHABET, false); };
    };
    let len =(data.size() + 3)/4*5;
    var ret: [var Nat8] = [var];
    var res: Text = "";
    let chunks = bytesToChunks(data, 5);
    for (i in chunks.keys()) {
      let buf: [var Nat8] = Array.init<Nat8>(5, 0);
      for (j in chunks[i].keys()) {
        buf[j] := chunks[i][j];
      };
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat((buf[0] & 0xF8) >> 3)]]));
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat(((buf[0] & 0x07) << 2) | ((buf[1] & 0xC0) >> 6))]]));
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat((buf[1] & 0x3E) >> 1)]]));
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat(((buf[1] & 0x01) << 4) | ((buf[2] & 0xF0) >> 4))]]));
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat(((buf[2] & 0x0F) << 1) | (buf[3] >> 7))]]));
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat((buf[3] & 0x7C) >> 2)]]));
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat(((buf[3] & 0x03) << 3) | ((buf[4] & 0xE0) >> 5))]]));
      ret := Array.thaw(Array.append(Array.freeze(ret), [alpha[Nat8.toNat(buf[4] & 0x1F)]]));
    };
    var len_ret: Nat = ret.size();
    if ((data.size() % 5) != 0) {
      let len = ret.size();
      var  num_extra = 0;
      if (8 < ((data.size() % 5 * 8 + 4) / 5)) {
        num_extra := 0;
      } else {
        num_extra := 8 - ((data.size() % 5 * 8 + 4) / 5);
      };
      if padding {
        for (i in Iter.range(1, num_extra)) {
          ret[len - i] := 61; // b'=' == 61
        };
      } else {
        len_ret := len - num_extra;
      };
    };
    for (i in Iter.range(0, len_ret-1)) {
      res := res # Char.toText(Char.fromNat32(Nat32.fromNat(Nat8.toNat(ret[i]) )));
    };
    return res;
  };

  /// decode the text
  public func decode(alphabet: Alphabet, data: Text) : ?[Nat8] {
    if (not is_ascii(data)) {
      return null;
    };
    var bytes: [Nat8] = [];
    for (i in Text.toIter(data)) {
      bytes := Array.append(bytes, [Nat8.fromNat(Nat32.toNat(Char.toNat32(i)))]);
    };
    let alpha = switch (alphabet) {
      case (#RFC4648 { padding }) { RFC4648_INV_ALPHABET; };
      case (#Crockford) { CROCKFORD_INV_ALPHABET; };
    };
    var unpadded_bytes_length = bytes.size();
    label l for (i in Iter.range(1, Nat.min(6, bytes.size()))) {
      if (bytes[bytes.size() - i] != 61) { // b'=' == 61
        break l;
      };
      unpadded_bytes_length -= 1;
    };
    let output_length = unpadded_bytes_length*5/8;
    var ret: [Nat8] = [];
    let ret_len = (output_length+4)/5*5;
    let chunks = bytesToChunks(bytes, 8);
    for (i in chunks.keys()) {
      let buf: [var Nat8] = Array.init<Nat8>(8, 0);
      for (j in chunks[i].keys()) {
        switch (get_element(alpha, Nat8.toNat(wrapping_sub(to_ascii_uppercase(chunks[i][j]), 48)) )) { // b'0' == 48
          case (?-1 or null) { return null; };
          case (?val) { buf[j] := Int8.toNat8(val); };
        }
      };
      ret := Array.append(ret, [((buf[0] << 3) | (buf[1] >> 2))]);
      ret := Array.append(ret, [((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4))]);
      ret := Array.append(ret, [((buf[3] << 4) | (buf[4] >> 1))]);
      ret := Array.append(ret, [((buf[4] << 7) | (buf[5] << 2)) | (buf[6] >> 3)]);
      ret := Array.append(ret, [((buf[6] << 5) | buf[7])]);
    };
    var res = Array.init<Nat8>(output_length, 0);
    for (i in res.keys()) {
      res[i] := ret[i];
    };
    return ?Array.freeze(res);     
  };

  func bytesToChunks(bytes: [Nat8], interval: Nat) : [[Nat8]] {
    let len = bytes.size();
    var ret: [[Nat8]] = [];
    for (i in Iter.range(1, len)) {
      var chunk: [var Nat8] = Array.init<Nat8>(interval, 0);
      if (i % interval == 0) {
        for (j in Iter.range(0, interval-1)) {
          chunk[j] := bytes[i-(interval-j)];
        };
        ret := Array.append(ret, [Array.freeze(chunk)]);
      };
    };
    if (len % interval != 0) {
      var chunk: [Nat8] = [];
      for (i in Iter.range(0, (len % interval) - 1)) {
        chunk := Array.append<Nat8>(chunk, [bytes[len - (len % interval) + i]]);
      };
      ret := Array.append<[Nat8]>(ret, [chunk]);
    };
    return ret;
  };

  func is_ascii(a: Text) : Bool {
    for (i in Text.toIter(a)) {
      if (Char.toNat32(i) > 0x7F) {
        return false;
      };
    };
    return true;
  };

  func to_ascii_uppercase(a: Nat8) : Nat8 {
    if (is_ascii_lowercase(Char.fromNat32(Nat32.fromNat(Nat8.toNat(a))))) {
      return 32 ^ a;
    } else {
      return a;
    };
  };

  func is_ascii_lowercase(a: Char) : Bool {
    return (a >= 'a' and a <= 'z');
  };

  func get_element(a: [Int8], index: Nat) : ?Int8 {
    if (index < a.size()) {
      return ?a[index];
    } else {
      return null;
    };
  };

  func wrapping_sub(a: Nat8, b: Nat8) :  Nat8 {
    if (a < b) {
      return 255 - b + a + 1;
    } else {
      return a - b;
    };
  };
};