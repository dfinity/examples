import Array "mo:core/Array";
import Option "mo:core/Option";
import Nat8 "mo:core/Nat8";
import Char "mo:core/Char";
import Result "mo:core/Result";
import Text "mo:core/Text";
import Prim "mo:⛔";

module {

  private type Result<Ok, Err> = Result.Result<Ok, Err>;

  private let base : Nat8 = 0x10;

  private let symbols = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
  ];

  public type DecodeError = {
    #msg : Text;
  };

  public func encode(array : [Nat8]) : Text {
    var encoded = "";
    for (w8 in array.vals()) {
      encoded #= encodeW8(w8);
    };
    encoded.map(Prim.charToLower);
  };

  private func encodeW8(w8 : Nat8) : Text {
    let c1 = symbols[(w8 / base).toNat()];
    let c2 = symbols[(w8 % base).toNat()];
    c1.toText() # c2.toText();
  };

  public func decode(text : Text) : Result<[Nat8], DecodeError> {
    let upper = text.map(Prim.charToUpper);
    let next = upper.chars().next;
    func parse() : Result<Nat8, DecodeError> {
      (do ? {
        let c1 = next()!;
        let c2 = next()!;
        decodeW4(c1).chain(func(x1 : Nat8) : Result<Nat8, DecodeError> {
          decodeW4(c2).chain(func(x2 : Nat8) : Result<Nat8, DecodeError> {
            #ok(x1 * base + x2);
          })
        })
      }).get(#err(#msg "Not enough input!"));
    };
    var i = 0;
    let n = upper.size() / 2 + upper.size() % 2;
    let array = Array.repeat(0 : Nat8, n).toVarArray();
    while (i != n) {
      switch (parse()) {
        case (#ok w8) {
          array[i] := w8;
          i += 1;
        };
        case (#err err) {
          return #err err;
        };
      };
    };
    #ok(Array.fromVarArray(array));
  };

  private func decodeW4(char : Char) : Result<Nat8, DecodeError> {
    for (i in symbols.keys()) {
      if (symbols[i] == char) {
        return #ok(Nat8.fromNat(i));
      };
    };
    let str = "Unexpected character: " # char.toText();
    #err(#msg str);
  };
};
