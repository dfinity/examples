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

  // Posts indexed by the author.
  // Newest posts are at the front of the post list.
  var posts : HashMap.HashMap<Principal, Posts> = HashMap.HashMap(10, Principal.equal, Principal.hash);

  // Records a new message posted by the specified author.
  func addPost(author : Principal, text : Text) {
    let post = { text = text; created_at = Time.now(); };
    
    let newPosts = switch (posts.get(author)) {
      case null { List.make(post) };
      case (?oldPosts) { List.push(post, oldPosts) };
    };

    posts.put(author, newPosts);
  };

  // Returns the default account identifier of this canister.
  func myAccountId() : Account.AccountIdentifier {
    Account.accountIdentifier(Principal.fromActor(Self), Account.defaultSubaccount())
  };

  // Adds a new post.
  public shared ({ caller }) func post(lit : Text) : async () {
    addPost(caller, lit);
  };

  // Returns messages posted by the caller.
  public shared query ({ caller }) func myPosts() : async Posts {
    switch (posts.get(caller)) {
      case null { null };
      case (?p) { p };
    }
  };

  // Returns canister's default account identifier as a blob.
  public query func canisterAccount() : async Account.AccountIdentifier {
    myAccountId()
  };

  // Returns current balance on the default account of this canister.
  public func canisterBalance() : async Ledger.Tokens {
    await Ledger.account_balance({ account = myAccountId() })
  };

  // Rewards the most prolific author of the last week with 1 token.
  //
  // Returns the principal of the winner, if there is one.
  public func distributeRewards() : async ?Principal {
    let weekNanos = 7 * 24 * 3600 * 1_000_000_000;
    let now = Time.now();
    let threshold = if (now < weekNanos) { 0 } else { now - weekNanos };

    var maxPosts = 0;
    var mostProlificAuthor : ?Principal = null;

    // Go over all the posts and find the most prolific author.
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
        // If there is a winner, transfer 1 Token to the winner.
        let res = await Ledger.transfer({
          memo = Nat64.fromNat(maxPosts);
          from_subaccount = null;
          to = Account.accountIdentifier(principal, Account.defaultSubaccount());
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
