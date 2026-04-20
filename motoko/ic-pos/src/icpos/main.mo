import Array "mo:core/Array";
import Blob "mo:core/Blob";
import Debug "mo:core/Debug";
import Map "mo:core/Map";
import Nat "mo:core/Nat";
import Nat64 "mo:core/Nat64";
import Principal "mo:core/Principal";
import Text "mo:core/Text";
import Time "mo:core/Time";

import MainTypes "main.types";
import CkBtcLedger "canister:icrc1_ledger";
import HttpTypes "http/http.types";

/**
*  This actor is responsible for:
*  - Storing merchant information
*  - Monitoring the ledger for new transactions
*  - Notifying merchants of new transactions
*
*  `_startBlock` is the block number to start monitoring transactions from.
*/
shared (actorContext) persistent actor class Main(_startBlock : Nat) {

  private let merchantStore = Map.empty<Text, MainTypes.Merchant>();
  private var latestTransactionIndex : Nat = 0;
  private var courierApiKey : Text = "";
  private transient var logData : [Text] = [];

  /**
    *  Get the merchant's information
    */
  public query (context) func getMerchant() : async MainTypes.Response<MainTypes.Merchant> {
    let caller : Principal = context.caller;

    switch (Map.get(merchantStore, Text.compare, Principal.toText(caller))) {
      case (?merchant) {
        {
          status = 200;
          status_text = "OK";
          data = ?merchant;
          error_text = null;
        };
      };
      case null {
        {
          status = 404;
          status_text = "Not Found";
          data = null;
          error_text = ?("Merchant with principal ID: " # Principal.toText(caller) # " not found.");
        };
      };
    };
  };

  /**
    * Update the merchant's information
    */
  public shared (context) func updateMerchant(merchant : MainTypes.Merchant) : async MainTypes.Response<MainTypes.Merchant> {

    let caller : Principal = context.caller;
    let _ = Map.swap(merchantStore, Text.compare, Principal.toText(caller), merchant);
    {
      status = 200;
      status_text = "OK";
      data = ?merchant;
      error_text = null;
    };
  };

  /**
    * Set the courier API key. Only the owner can set the courier API key.
    */
  public shared (context) func setCourierApiKey(apiKey : Text) : async MainTypes.Response<Text> {
    if (not Principal.equal(context.caller, actorContext.caller)) {
      return {
        status = 403;
        status_text = "Forbidden";
        data = null;
        error_text = ?"Only the owner can set the courier API key.";
      };
    };
    courierApiKey := apiKey;
    {
      status = 200;
      status_text = "OK";
      data = ?courierApiKey;
      error_text = null;
    };
  };

  /**
  * Get latest log items. Log output is capped at 100 items.
  */
  public query func getLogs() : async [Text] {
    logData;
  };

  /**
    * Log a message. Log output is capped at 100 items.
    */
  private func log(text : Text) {
    Debug.print(text);
    logData := Array.tabulate<Text>(Nat.min(logData.size() + 1, 100), func(i : Nat) : Text {
      if (i == 0) text else logData[i - 1]
    });
  };

  /**
    * Check for new transactions and notify the merchant if a new transaction is found.
    * This function is called by the global timer.
    */
  system func timer(setGlobalTimer : Nat64 -> ()) : async () {
    let next = Nat64.fromIntWrap(Time.now()) + 20_000_000_000; // 20 seconds
    setGlobalTimer(next);
    await notify();
  };

  /**
    * Notify the merchant if a new transaction is found.
    */
  private func notify() : async () {
    var start : Nat = _startBlock;
    if (latestTransactionIndex > 0) {
      start := latestTransactionIndex + 1;
    };

    var response = await CkBtcLedger.get_transactions({
      start = start;
      length = 1;
    });

    if (response.transactions.size() > 0) {
      latestTransactionIndex := start;

      if (response.transactions[0].kind == "transfer") {
        let t = response.transactions[0];
        switch (t.transfer) {
          case (?transfer) {
            let to = transfer.to.owner;
            switch (Map.get(merchantStore, Text.compare, Principal.toText(to))) {
              case (?merchant) {
                if (merchant.email_notifications or merchant.phone_notifications) {
                  log("Sending notification to: " # debug_show (merchant.email_address));
                  await sendNotification(merchant, t);
                };
              };
              case null {
                // No action required if merchant not found
              };
            };
          };
          case null {
            // No action required if transfer is null
          };
        };
      };
    };
  };

  /**
    * Send a notification to a merchant about a received payment
    */
  private func sendNotification(merchant : MainTypes.Merchant, transaction : CkBtcLedger.Transaction) : async () {
    let ic : HttpTypes.IC = actor ("aaaaa-aa");

    var amount = "0";
    var from = "";
    switch (transaction.transfer) {
      case (?transfer) {
        amount := Nat.toText(transfer.amount);
        from := Principal.toText(transfer.from.owner);
      };
      case null {};
    };
    let idempotencyKey : Text = Text.concat(merchant.name, Nat64.toText(transaction.timestamp));
    let requestBodyJson : Text = "{ \"idempotencyKey\": \"" # idempotencyKey # "\", \"email\": \"" # merchant.email_address # "\", \"phone\": \"" # merchant.phone_number # "\", \"amount\": \"" # amount # "\", \"payer\": \"" # from # "\"}";
    let requestBodyAsBlob : Blob = Text.encodeUtf8(requestBodyJson);
    let requestBodyAsNat8 : [Nat8] = Blob.toArray(requestBodyAsBlob);

    let httpRequest : HttpTypes.HttpRequestArgs = {
      url = "https://icpos-notifications.xyz/.netlify/functions/notify";
      max_response_bytes = ?Nat64.fromNat(1000);
      headers = [
        { name = "Content-Type"; value = "application/json" },
      ];
      body = ?requestBodyAsNat8;
      method = #post;
      transform = null;
    };

    // Cycle cost of sending a notification
    // 49.14M + 5200 * request_size + 10400 * max_response_bytes
    // 49.14M + (5200 * 1000) + (10400 * 1000) = 64.74M
    let httpResponse : HttpTypes.HttpResponsePayload = await (with cycles = 70_000_000) ic.http_request(httpRequest);

    if (httpResponse.status > 299) {
      let response_body : Blob = Blob.fromArray(httpResponse.body);
      let decoded_text : Text = switch (Text.decodeUtf8(response_body)) {
        case (null) { "No value returned" };
        case (?y) { y };
      };
      log("Error sending notification: " # decoded_text);
    } else {
      log("Notification sent");
    };
  };

  system func postupgrade() {
    // Make sure we start to montitor transactions from the block set on deployment
    latestTransactionIndex := _startBlock;
  };
};
