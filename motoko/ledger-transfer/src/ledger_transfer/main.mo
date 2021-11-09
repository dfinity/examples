import Ledger    "canister:ledger";

import Debug     "mo:base/Debug";
import Error     "mo:base/Error";
import Int       "mo:base/Int";
import HashMap   "mo:base/HashMap";
import List      "mo:base/List";
import Nat64     "mo:base/Nat64";
import Principal "mo:base/Principal";
import Time      "mo:base/Time";

import Account   "./Account";

actor Self {
  public type Post = {
    text : Text;
    created_at : Int;
  };

  public type Posts = List.List<Post>;

  var posts : HashMap.HashMap<Principal, Posts> = HashMap.HashMap(10, Principal.equal, Principal.hash);

  func addPost(author : Principal, text : Text) {
    let post = { text = text; created_at = Time.now(); };
    
    let newPosts = switch (posts.get(author)) {
      case null { List.make(post) };
      case (?oldPosts) { List.push(post, oldPosts) };
    };

    posts.put(author, newPosts);
  };

  public shared ({ caller }) func post(lit : Text) : async () {
    addPost(caller, lit);
  };

  public shared query ({ caller }) func myPosts() : async Posts {
    switch (posts.get(caller)) {
      case null { null };
      case (?p) { p };
    }
  };

  public query func canisterAddress() : async Account.Address {
    Account.address(Principal.fromActor(Self), Account.defaultSubaccount())
  };

  public func distributeRewards() : async ?Principal {
    let weekNanos = 7 * 24 * 3600 * 1_000_000_000;
    let now = Time.now();
    let threshold = if (now < weekNanos) { 0 } else { now - weekNanos };

    var maxPosts = 0;
    var mostProlificAuthor : ?Principal = null;

    for ((author, posts) in posts.entries()) {
      let numFreshPosts = List.foldLeft(posts, 0 : Nat, func (acc : Nat, post : Post) : Nat {
        if (post.created_at >= threshold) { acc + 1 } else { acc }
      });
      if (numFreshPosts > maxPosts) {
        maxPosts := numFreshPosts;
        mostProlificAuthor := ?author;
      };
    };
    
    switch (mostProlificAuthor) {
      case null {};
      case (?principal) {
        let res = await Ledger.transfer({
          memo = Nat64.fromNat(maxPosts);
          from_subaccount = null;
          to = Account.address(principal, Account.defaultSubaccount());
          amount = { e8s = 100_000_000 };
          fee = { e8s = 10_000 };
          created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(now)) };
        });
        switch (res) {
          case (#Ok(blockIndex)) {
            Debug.print("Paid reward to " # debug_show principal # " in block " # debug_show blockIndex);
          };
          case (#Err(#InsufficientFunds { balance })) {
            throw Error.reject("Top me up! The balance is only " # debug_show balance # " e8s");
          };
          case (#Err(other)) {
            throw Error.reject("Unexpected error: " # debug_show other);
          };
        };
      };
    };

    mostProlificAuthor
  };
};
