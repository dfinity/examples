import Debug "mo:core/Debug";
import Map "mo:core/Map";
import Nat "mo:core/Nat";
import Nat8 "mo:core/Nat8";
import Nat64 "mo:core/Nat64";
import Principal "mo:core/Principal";
import Runtime "mo:core/Runtime";
import Text "mo:core/Text";
import Time "mo:core/Time";

import Types "app.types";

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
    icrc1_symbol : () -> async Text;
    icrc1_decimals : () -> async Nat8;
  };

  let merchantStore = Map.empty<Text, Types.Merchant>();
  // Next ledger block index to scan for incoming transfers.
  var nextBlock : Nat = _startBlock;
  // Token metadata, cached lazily from the ledger to format logged amounts.
  // Transient: re-fetched on demand after an upgrade.
  transient var tokenSymbol : Text = "tokens";
  transient var tokenDecimals : Nat = 8;
  transient var metadataLoaded : Bool = false;

  // How many ledger blocks to scan per timer tick. Draining a batch (rather than
  // one block per tick) means a payment is logged within a single tick even when
  // the monitor is catching up on a backlog (e.g. the initial mint).
  let scanBatchSize : Nat = 100;

  // Get the caller's merchant information.
  public query (context) func getMerchant() : async Types.Response<Types.Merchant> {
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
  public shared (context) func updateMerchant(merchant : Types.Merchant) : async Types.Response<Types.Merchant> {
    merchantStore.add(context.caller.toText(), merchant);
    { status = 200; status_text = "OK"; data = ?merchant; error_text = null };
  };

  // Resolve the ledger canister, injected as PUBLIC_CANISTER_ID:icrc1_ledger.
  func ledger<system>() : Ledger {
    switch (Runtime.envVar<system>("PUBLIC_CANISTER_ID:icrc1_ledger")) {
      case (?id) actor (id) : Ledger;
      case null Runtime.trap("PUBLIC_CANISTER_ID:icrc1_ledger not set — run icp deploy");
    };
  };

  // Fetch the token's symbol and decimals from the ledger once, then cache them.
  func loadMetadataOnce<system>() : async () {
    if (metadataLoaded) return;
    let l = ledger<system>();
    tokenSymbol := await l.icrc1_symbol();
    tokenDecimals := Nat8.toNat(await l.icrc1_decimals());
    metadataLoaded := true;
  };

  // Render a base-unit amount using the cached decimals and symbol, e.g. with
  // 8 decimals: 100_000_000 -> "1 LICRC1", 50_000 -> "0.0005 LICRC1".
  func formatAmount(amount : Nat) : Text {
    let base = 10 ** tokenDecimals;
    let whole = (amount / base).toText();
    let frac = amount % base;
    if (frac == 0) return whole # " " # tokenSymbol;
    // Left-pad the fractional digits to `tokenDecimals`, then drop trailing zeros.
    var fracText = frac.toText();
    while (fracText.size() < tokenDecimals) { fracText := "0" # fracText };
    whole # "." # Text.trimEnd(fracText, #char '0') # " " # tokenSymbol;
  };

  // Scan for new transactions and log a would-be notification for each payment
  // to a merchant that has notifications enabled. Called by the global timer.
  system func timer(setGlobalTimer : Nat64 -> ()) : async () {
    let next = Nat64.fromIntWrap(Time.now()) + 20_000_000_000; // 20 seconds
    setGlobalTimer(next);
    await notify<system>();
  };

  func notify<system>() : async () {
    let response = await ledger<system>().get_transactions({ start = nextBlock; length = scanBatchSize });
    let txs = response.transactions;
    if (txs.size() == 0) return; // caught up; check again next tick
    nextBlock += txs.size();

    var i = 0;
    label scan while (i < txs.size()) {
      let t = txs[i];
      i += 1;
      if (t.kind != "transfer") continue scan;
      switch (t.transfer) {
        case (?transfer) {
          switch (merchantStore.get(transfer.to.owner.toText())) {
            case (?merchant) {
              // A payment for a merchant with notifications enabled: emit a
              // canister log (observable via `icp canister logs backend`) marking
              // where a real notification would be sent. To actually send one,
              // use HTTPS outcalls —
              // https://docs.internetcomputer.org/guides/backends/https-outcalls
              if (merchant.email_notifications or merchant.phone_notifications) {
                if (not metadataLoaded) await loadMetadataOnce<system>();
                Debug.print(
                  "Payment of " # formatAmount(transfer.amount) # " received by merchant '" # merchant.name #
                  "' from " # transfer.from.owner.toText() #
                  ". A notification could be sent here via HTTPS outcalls."
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
};
