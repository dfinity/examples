import Utils "./utils";
import Map "./map";

module {
  func getFromMap(map: [(Char, Text)], key: Char): Text {
    Map.fromEntries<Text>(map).get(Utils.toUpper(key))
  };

  public func newFirstName(key: Char): Text { getFromMap(newFirstNames, key) };
  public func firstHalfOfNewLastName(key: Char): Text { getFromMap(firstHalfOfNewLastNames, key) };
  public func secondHalfOfNewLastName(key: Char): Text { getFromMap(secondHalfOfNewLastNames, key) };

  let newFirstNames = [
    ('A', "Stinky"),
    ('B', "Lumpy"),
    ('C', "Buttercup"),
    ('D', "Gidget"),
    ('E', "Crusty"),
    ('F', "Greasy"),
    ('G', "Fluffy"),
    ('H', "Cheeseball"),
    ('I', "Chim-Chim"),
    ('J', "Poopsie"),
    ('K', "Flunky"),
    ('L', "Booger"),
    ('M', "Pinky"),
    ('N', "Zippy"),
    ('O', "Goober"),
    ('P', "Doofus"),
    ('Q', "Slimy"),
    ('R', "Loopy"),
    ('S', "Snotty"),
    ('T', "Falafel"),
    ('U', "Dorky"),
    ('V', "Squeezit"),
    ('W', "Oprah"),
    ('X', "Skipper"),
    ('Y', "Dinky"),
    ('Z', "Zsa-Zsa")
  ];

  let firstHalfOfNewLastNames = [
    ('A', "Diaper"),
    ('B', "Toilet"),
    ('C', "Giggle"),
    ('D', "Bubble"),
    ('E', "Girdle"),
    ('F', "Barf"),
    ('G', "Lizard"),
    ('H', "Waffle"),
    ('I', "Cootie"),
    ('J', "Monkey"),
    ('K', "Potty"),
    ('L', "Liver"),
    ('M', "Banana"),
    ('N', "Rhino"),
    ('O', "Burger"),
    ('P', "Hamster"),
    ('Q', "Toad"),
    ('R', "Gizzard"),
    ('S', "Pizza"),
    ('T', "Gerbil"),
    ('U', "Chicken"),
    ('V', "Pickle"),
    ('W', "Chuckle"),
    ('X', "Tofu"),
    ('Y', "Gorilla"),
    ('Z', "Stinker")
  ];

  let secondHalfOfNewLastNames = [
    ('A', "head"),
    ('B', "mouth"),
    ('C', "face"),
    ('D', "nose"),
    ('E', "tush"),
    ('F', "breath"),
    ('G', "pants"),
    ('H', "shorts"),
    ('I', "lips"),
    ('J', "honker"),
    ('K', "butt"),
    ('L', "brain"),
    ('M', "tushy"),
    ('N', "chunks"),
    ('O', "honey"),
    ('P', "biscuit"),
    ('Q', "toes"),
    ('R', "buns"),
    ('S', "fanny"),
    ('T', "sniffer"),
    ('U', "sprinkles"),
    ('V', "kisser"),
    ('W', "squirt"),
    ('X', "humperdinck"),
    ('Y', "brains"),
    ('Z', "juice")
  ];
}
