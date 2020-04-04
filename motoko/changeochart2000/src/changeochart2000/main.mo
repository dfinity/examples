import Prim "mo:prim";
import Names "./names";
import Text "mo:stdlib/Text";
import Utils "./utils";

actor changeochart2000 {
  public func translate(name: Text): async Text {
    let names = Utils.split(name, ' ');

    let firstLetterOfFirstName = Utils.firstChar(Utils.first<Text>(names));
    let firstLetterOfLastName = Utils.firstChar(Utils.last<Text>(names));
    let lastLetterOfLastName = Utils.lastChar(Utils.last<Text>(names));

    Utils.joinText([
      Names.newFirstName(firstLetterOfFirstName),
      " ",
      Names.firstHalfOfNewLastName(firstLetterOfLastName),
      Names.secondHalfOfNewLastName(lastLetterOfLastName)
    ])
  };
};
