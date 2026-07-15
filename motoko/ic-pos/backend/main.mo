import Array "mo:core/Array";
import Debug "mo:core/Debug";
import Map "mo:core/Map";
import Nat "mo:core/Nat";
import Nat64 "mo:core/Nat64";
import Principal "mo:core/Principal";
import Runtime "mo:core/Runtime";
import Text "mo:core/Text";
import Time "mo:core/Time";

import MainTypes "main.types";

// This actor:
//  - stores merchant information,
//  - monitors the ICRC-1 ledger for incoming transfers, and
//  - logs where a merchant notification would be sent.
//
// The ledger canister is resolved at runtime from the injected
// `PUBLIC_CANISTER_ID:icrc1_ledger` environment variable:
//   local: the pre-built ICRC-1 ledger deployed by deploy.sh
//   ic:    the TICRC1 test ledger (see icp.yaml)
//
// `_startBlock` is the ledger block index to start monitoring from.
//
// NOTE: notifications are illustrative. The original example sent email/SMS via
// an HTTPS outcall to a third party; this version only logs that a notification
// could be sent. To implement real notifications, use HTTPS outcalls — see
// https://docs.internetcomputer.org/guides/backends/https-outcalls
//
// NOTE: scanning the ledger's global transaction log sequentially does not scale
// to a busy shared ledger (such as TICRC1); it is illustrative only. A
// production app would query the index canister per merchant account instead
// (as this example's frontend already does).
actor class Main(_startBlock : Nat) {

  // Minimal subset of the ICRC-1 ledger interface this actor uses. Candid
  // ignores wire fields not declared here, so only the read fields are listed.
  type Account = { owner : Principal };
  type Transfer = { to : Account; from : Account; amount : Nat };
  type Transaction = { kind : Text; transfer : ?Transfer };
  type GetTransactionsRequest = { start : Nat; length : Nat };
  type GetTransactionsResponse = { transactions : [Transaction] };
  type Ledger = actor {
    get_transactions : GetTransactionsRequest -> async GetTransactionsResponse;
  };

  let merchantStore = Map.empty<Text, MainTypes.Merchant>();
  // Next ledger block index to scan for incoming transfers.
  var nextBlock : Nat = _startBlock;
  transient var logData : [Text] = [];

  // Get the caller's merchant information.
  public query (context) func getMerchant() : async MainTypes.Response<MainTypes.Merchant> {
    switch (merchantStore.get(context.caller.toText())) {
      case (?merchant) {
        { status = 200; status_text = "OK"; data = ?merchant; error_text = null };
      };
      case null {
        {
          status = 404;
          status_text = "Not Found";
          data = null;
          error_text = ?("Merchant with principal ID: " # context.caller.toText() # " not found.");
        };
      };
    };
  };

  // Create or update the caller's merchant information.
  public shared (context) func updateMerchant(merchant : MainTypes.Merchant) : async MainTypes.Response<MainTypes.Merchant> {
    merchantStore.add(context.caller.toText(), merchant);
    { status = 200; status_text = "OK"; data = ?merchant; error_text = null };
  };

  // Get the latest log items (capped at 100).
  public query func getLogs() : async [Text] {
    logData;
  };

  // Append a log message, keeping the 100 most recent.
  func log(text : Text) {
    Debug.print(text);
    logData := Array.tabulate(Nat.min(logData.size() + 1, 100), func(i) { if (i == 0) text else logData[i - 1] });
  };

  // Resolve the ledger canister, injected as PUBLIC_CANISTER_ID:icrc1_ledger.
  func ledger<system>() : Ledger {
    switch (Runtime.envVar<system>("PUBLIC_CANISTER_ID:icrc1_ledger")) {
      case (?id) actor (id) : Ledger;
      case null Runtime.trap("PUBLIC_CANISTER_ID:icrc1_ledger not set — run icp deploy");
    };
  };

  // Check for a new transaction and log a would-be notification for the
  // receiving merchant. Called by the global timer.
  system func timer(setGlobalTimer : Nat64 -> ()) : async () {
    let next = Nat64.fromIntWrap(Time.now()) + 20_000_000_000; // 20 seconds
    setGlobalTimer(next);
    await notify<system>();
  };

  func notify<system>() : async () {
    let response = await ledger<system>().get_transactions({ start = nextBlock; length = 1 });
    if (response.transactions.size() == 0) return; // caught up; retry this block next tick
    nextBlock += 1;

    let t = response.transactions[0];
    if (t.kind != "transfer") return;

    switch (t.transfer) {
      case (?transfer) {
        switch (merchantStore.get(transfer.to.owner.toText())) {
          case (?merchant) {
            if (merchant.email_notifications or merchant.phone_notifications) {
              log(
                "Payment of " # transfer.amount.toText() # " received by merchant '" # merchant.name #
                "' from " # transfer.from.owner.toText() #
                ". A notification could be sent here via HTTPS outcalls " #
                "(see https://docs.internetcomputer.org/guides/backends/https-outcalls)."
              );
            };
          };
          case null {};
        };
      };
      case null {};
    };
  };
};
