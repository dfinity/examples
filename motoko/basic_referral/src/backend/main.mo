import Array        "mo:base/Array";
import Blob         "mo:base/Blob";
import AsyncSource  "mo:uuid/async/SourceV4";
import Debug        "mo:base/Debug";
import Float        "mo:base/Float";
import Error        "mo:base/Error";
import Int          "mo:base/Int";
import Int64        "mo:base/Int64";
import Iter         "mo:base/Iter";
import List         "mo:base/List";
import Nat64        "mo:base/Nat64";
import Nat          "mo:base/Nat";
import Option       "mo:base/Option";
import Principal    "mo:base/Principal";
import Result       "mo:base/Result";
import Text         "mo:base/Text";
import Time         "mo:base/Time";
import Trie         "mo:base/Trie";
import UUID         "mo:uuid/UUID";

import Account      "./plugins/Account";
import Ledger       "./plugins/Ledger";
import Types        "types";
import State        "state";
import Env          ".env";

shared ({ caller = owner }) actor class Backend() = this {
  stable var transferFee    : Nat64 = 10_000;
  stable var referralAward  : Nat64 = 40_000;
  stable var referralLimit  : Int = 99;

  var state : State.State = State.empty();

  private stable var profiles       : [(Principal, Types.Profile)] = [];
  private stable var transactions   : [(Text, Types.TxRecord)] = [];
  private stable var referrals      : [(Text, Types.Referral)] = [];

  system func preupgrade() {
    Debug.print("Begin preupgrade");
    profiles := Iter.toArray(state.profiles.entries());
    referrals := Iter.toArray(state.referrals.entries());
    transactions := Iter.toArray(state.transactions.entries());
    Debug.print("End preupgrade");
  };

  system func postupgrade() {
    Debug.print("Begin postupgrade");
    for ((k, v) in Iter.fromArray(profiles)) {
      state.profiles.put(k, v);
    };
    for ((k, v) in Iter.fromArray(referrals)) {
      state.referrals.put(k, v);
    };
    for ((k, v) in Iter.fromArray(transactions)) {
      state.transactions.put(k, v);
    };
    Debug.print("End postupgrade");
  };

  type Response<Ok> = Result.Result<Ok, Types.Error>;
  private let ledger : Ledger.Interface = actor (Env.LEDGER_ID);

  public shared ({ caller }) func getBalance() : async Ledger.ICP {
    let accountId = Account.accountIdentifier(
      Principal.fromActor(this),
      Account.principalToSubaccount(caller)
    );
    await ledger.account_balance({ account = accountId });
  };

  public func getSystemBalance() : async Ledger.ICP {
    let accountId = Account.accountIdentifier(Principal.fromActor(this), Account.defaultSubaccount());
    await ledger.account_balance({ account = accountId });
  };

  public query func getSystemAddress() : async Blob {
    Account.accountIdentifier(Principal.fromActor(this), Account.defaultSubaccount());
  };

  public query func getSystemAddressAsText() : async Text {
    Account.toText(
      Account.accountIdentifier(Principal.fromActor(this), Account.defaultSubaccount())
    );
  };

  public shared ({ caller }) func register(
    inviterID : ?Text,
    username : ?Text,
    avatar : ?Text,
    phone : ?Text,
  ) : async Response<Text> {
    if (Principal.toText(caller) == "2vxsx-fae") {
      return #err(#NotAuthorized); //isNotAuthorized
    };
    switch (state.profiles.get(caller)) {
      case null {
        Debug.print("Create profile");
        let inviter = Account.principalFromText(Option.get(inviterID, ""));
        let profile = state.profiles.put(
          caller,
          {
            username;
            avatar;
            phone;
            inviter
          }
        );
        switch (inviter) {
          case null {};
          case (?user) {
            switch (state.profiles.get(user)) {
              case null { Debug.print("Inviter not found"); };
              case (_) {
                await rewardReferral(user, caller);
              };
            };
          };
        };
        #ok("Success!");
      };
      case (_) {
        #err(#AlreadyExisting);
      };
    };
  };

  public query ({ caller }) func getReferralCount() : async Response<Int> {
    if (Principal.toText(caller) == "2vxsx-fae") {
      return #err(#NotAuthorized); //isNotAuthorized
    };
    var count : Int = 0;
    for ((_uuid, referral) in state.referrals.entries()) {
      if (referral.uid == caller) {
        count := count + 1;
      };
    };
    #ok(count);
  };

  public query ({ caller }) func getPrincipalText() : async Text {
    Principal.toText(caller);
  };

  func rewardReferral(inviter : Principal, member : Principal) : async () {
    var referralCount : Int = 0;
    for ((_uuid, referral) in state.referrals.entries()) {
      if (referral.uid == inviter) {
        referralCount := referralCount + 1;
      };
    };
    if (referralCount < referralLimit) {
      let uuid = await createUUID();
      let payload = {
        uid = inviter;
        member;
      };
      Debug.print("Reward referral");
      let receipt = await transferToUser(referralAward, inviter);
      switch (receipt) {
        case (#Err(error)) {
          Debug.print(debug_show error);
        };
        case (#Ok(bIndex)) {
          let newReferral = state.referrals.put(uuid, payload);
          Debug.print("Record transaction");
          await recordTransaction(
            Principal.fromActor(this),
            referralAward,
            Principal.fromActor(this),
            inviter,
            #awardReferral,
            ?uuid,
            bIndex
          );
        };
      };
    };
  };

  func transferToUser(
    amount : Nat64, toPrincipal : Principal
  ) : async Ledger.TransferResult {
    let accountId = Account.accountIdentifier(Principal.fromActor(this), Account.principalToSubaccount(toPrincipal));
    await ledger.transfer({
      memo : Nat64 = 0;
      from_subaccount = ?Account.defaultSubaccount();
      to = accountId;
      amount = { e8s = amount };
      fee = { e8s = transferFee };
      created_at_time = ?{ timestamp_nanos = Nat64.fromNat(Int.abs(Time.now())) };
    });
  };

  private func createUUID() : async Text {
    var ae = AsyncSource.Source();
    let id = await ae.new();
    UUID.toText(id);
  };

  private func recordTransaction(
    caller : Principal,
    amount : Nat64,
    fromPrincipal : Principal,
    toPrincipal : Principal,
    refType : Types.Operation,
    refId : ?Text,
    blockIndex : Nat64
  ) : async () {
    let uuid : Text = await createUUID();
    let transaction : Types.TxRecord = {
      uuid;
      caller;
      refType;
      refId;
      blockIndex;
      fromPrincipal;
      toPrincipal;
      amount;
      fee = transferFee;
      timestamp = Time.now();
    };
    state.transactions.put(uuid, transaction);
  };
};