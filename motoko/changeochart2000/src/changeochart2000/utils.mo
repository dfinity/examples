import Prim "mo:prim";
import Array "mo:stdlib/Array";
import Iter "mo:stdlib/Iter";
import Text "mo:stdlib/Text";

module {
  func toChars(text: Text): [Char] { Iter.toArray<Char>(Text.toIter(text)) };

  public func first<T>(arr: [T]): T { arr[0] };
  public func last<T>(arr: [T]): T { arr[arr.len() - 1] };
  public func firstChar(text: Text): Char { first<Char>(toChars(text)) };
  public func lastChar(text: Text): Char { last<Char>(toChars(text)) };

  public func split(text: Text, delim: Char): [Text] {
    var texts: [Text] = [];
    var chars: [Char] = [];
    for (char in Text.toIter(text)) {
      if (char == delim) {
        texts := Array.append<Text>(texts, [joinChars(chars)]);
        chars := [];
      } else {
        chars := Array.append<Char>(chars, [char]);
      };
    };
    Array.append<Text>(texts, [joinChars(chars)])
  };

  public func joinChars(chars: [Char]): Text {
    func concat(acc: Text, val: Char): Text { acc # Prim.charToText(val) };
    Array.foldl<Char, Text>(concat, "", chars);
  };

  public func joinText(texts: [Text]): Text {
    func concat(acc: Text, val: Text): Text { acc # val };
    Array.foldl<Text, Text>(concat, "", texts);
  };

  public func toUpper(char: Char): Char {
    var charCode = Prim.charToWord32(char);
    if (charCode >= Prim.charToWord32('a') and charCode <= Prim.charToWord32('z')) {
      charCode -= 32;
    };
    Prim.word32ToChar(charCode)
  };
}
