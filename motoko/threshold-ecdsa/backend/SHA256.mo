import Array "mo:core/Array";
import Nat "mo:core/Nat";
import Nat8 "mo:core/Nat8";
import Nat32 "mo:core/Nat32";
import Nat64 "mo:core/Nat64";

module {

  private let K : [Nat32] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
  ];

  private let S : [Nat32] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
  ];

  public func sha256(data : [Nat8]) : [Nat8] {
    let digest = Digest();
    digest.write(data);
    return digest.sum();
  };

  public class Digest() {

    private let s : [var Nat32] = S.toVarArray();

    private let x = Array.repeat(0 : Nat8, 64).toVarArray();

    private var nx = 0;

    private var len : Nat64 = 0;

    public func reset() {
      for (i in s.keys()) {
        s[i] := S[i];
      };
      nx := 0;
      len := 0;
    };

    public func write(data : [Nat8]) {
      var p = data;
      len +%= Nat64.fromIntWrap(p.size());
      if (nx > 0) {
        let n = Nat.min(p.size(), 64 - nx);
        var i = 0;
        while (i < n) {
          x[nx + i] := p[i];
          i += 1;
        };
        nx += n;
        if (nx == 64) {
          let buf = Array.fromVarArray(x);
          block(buf);
          nx := 0;
        };
        p := Array.tabulate(p.size() - n, func(i : Nat) : Nat8 { p[n + i] });
      };
      if (p.size() >= 64) {
        let n = (Nat64.fromIntWrap(p.size()) & (^ 63)).toNat();
        let buf = Array.tabulate(n, func(i : Nat) : Nat8 { p[i] });
        block(buf);
        p := Array.tabulate(p.size() - n, func(i : Nat) : Nat8 { p[n + i] });
      };
      if (p.size() > 0) {
        for (i in p.keys()) {
          x[i] := p[i];
        };
        nx := p.size();
      };
    };

    public func sum() : [Nat8] {
      var m = 0;
      var n = len;
      let t = n.toNat() % 64;
      var buf : [var Nat8] = [var];
      if (56 > t) {
        m := 56 - t;
      } else {
        m := 120 - t;
      };
      n := n << 3;
      buf := Array.repeat(0 : Nat8, m).toVarArray();
      if (m > 0) {
        buf[0] := 0x80;
      };
      write(Array.fromVarArray(buf));
      buf := Array.repeat(0 : Nat8, 8).toVarArray();
      for (i in buf.keys()) {
        let j : Nat64 = 56 -% 8 *% Nat64.fromIntWrap(i);
        buf[i] := Nat8.fromIntWrap((n >> j).toNat());
      };
      write(Array.fromVarArray(buf));
      let hash = Array.repeat(0 : Nat8, 32).toVarArray();
      for (i in s.keys()) {
        var j = 0;
        while (j < 4) {
          let k : Nat32 = 24 -% 8 *% Nat32.fromIntWrap(j);
          hash[4 * i + j] := Nat8.fromIntWrap((s[i] >> k).toNat());
          j += 1;
        };
      };
      return Array.fromVarArray(hash);
    };

    private func block(data : [Nat8]) {
      var p = data;
      let w = Array.repeat(0 : Nat32, 64).toVarArray();
      while (p.size() >= 64) {
        var i = 0;
        var j = 0;
        while (i < 16) {
          j := i * 4;
          w[i] :=
            Nat32.fromIntWrap(p[j + 0].toNat()) << 24 |
            Nat32.fromIntWrap(p[j + 1].toNat()) << 16 |
            Nat32.fromIntWrap(p[j + 2].toNat()) << 08 |
            Nat32.fromIntWrap(p[j + 3].toNat()) << 00;
          i += 1;
        };
        var v1 : Nat32 = 0;
        var v2 : Nat32 = 0;
        var t1 : Nat32 = 0;
        var t2 : Nat32 = 0;
        i := 16;
        while (i < 64) {
          v1 := w[i - 02];
          v2 := w[i - 15];
          t1 := rot(v1, 17) ^ rot(v1, 19) ^ (v1 >> 10);
          t2 := rot(v2, 07) ^ rot(v2, 18) ^ (v2 >> 03);
          w[i] :=
              t1 +% w[i - 07] +%
              t2 +% w[i - 16];
          i += 1;
        };
        var a = s[0];
        var b = s[1];
        var c = s[2];
        var d = s[3];
        var e = s[4];
        var f = s[5];
        var g = s[6];
        var h = s[7];
        for (i in w.keys()) {
          t1 := rot(e, 06) ^ rot(e, 11) ^ rot(e, 25);
          t1 +%= (e & f) ^ (^ e & g) +% h +% K[i] +% w[i];
          t2 := rot(a, 02) ^ rot(a, 13) ^ rot(a, 22);
          t2 +%= (a & b) ^ (a & c) ^ (b & c);
          h := g;
          g := f;
          f := e;
          e := d +% t1;
          d := c;
          c := b;
          b := a;
          a := t1 +% t2;
        };
        s[0] +%= a;
        s[1] +%= b;
        s[2] +%= c;
        s[3] +%= d;
        s[4] +%= e;
        s[5] +%= f;
        s[6] +%= g;
        s[7] +%= h;
        p := Array.tabulate(p.size() - 64, func(i : Nat) : Nat8 { p[i + 64] });
      };
    };
  };

  private let rot : (Nat32, Nat32) -> Nat32 = Nat32.bitrotRight;
};
