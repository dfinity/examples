import Invoice "canister:invoice";

import Hash       "mo:base/Hash";
import HashMap    "mo:base/HashMap";
import Iter       "mo:base/Iter";
import Nat        "mo:base/Nat";
import Principal  "mo:base/Principal";
import Result     "mo:base/Result";
import Text       "mo:base/Text";

actor Seller {

  let ONE_ICP_IN_E8S = 100_000_000;

  stable var invoicesStable : [(Nat, Invoice.Invoice)] = [];
  var invoices: HashMap.HashMap<Nat, Invoice.Invoice> = HashMap.HashMap(16, Nat.equal, Hash.hash);

  stable var licensesStable : [(Principal, Bool)] = [];
  var licenses: HashMap.HashMap<Principal, Bool> = HashMap.HashMap(16, Principal.equal, Principal.hash);

// #region create_invoice
  public shared ({caller}) func create_invoice() : async Invoice.CreateInvoiceResult {
    let invoiceCreateArgs : Invoice.CreateInvoiceArgs = {
      amount = ONE_ICP_IN_E8S * 10;
      token = {
        symbol = "ICP";
      };
      permissions = null;
      details = ?{
        description = "Example license certifying status";
        // JSON string as a blob
        meta = Text.encodeUtf8(
          "{\n" #
          "  \"seller\": \"Invoice Canister Example Dapp\",\n" #
          "  \"itemized_bill\": [\"Standard License\"],\n" #
          "}"
        );
      };
    };
    let invoiceResult = await Invoice.create_invoice(invoiceCreateArgs);
    switch(invoiceResult){
      case(#err _) {};
      case(#ok result) {
        invoices.put(result.invoice.id, result.invoice);
      };
    };
    return invoiceResult;
  };

  public shared query ({caller}) func check_license_status() : async Bool {
    let licenseResult = licenses.get(caller);
    switch(licenseResult) {
      case(null){
        return false;
      };
      case (? license){
        return license;
      };
    };
  };

  public shared query ({caller}) func get_invoice(id: Nat) : async ?Invoice.Invoice {
    invoices.get(id);
  };

  public shared ({caller}) func verify_invoice(id: Nat) : async Invoice.VerifyInvoiceResult {
    let invoiceResult = invoices.get(id);
    switch(invoiceResult){
      case(null){
        return #err({
          kind = #Other;
          message = ?"Invoice not found for this user";
        });
      };
      case (? invoice){
        let verifyResult = await Invoice.verify_invoice({id = invoice.id});
        switch(verifyResult){
          case(#err _) {};
          case(#ok result) {
            switch(result){
              case(#Paid p){
                invoices.put(id, p.invoice);
              };
              case(#AlreadyPaid a){
                invoices.put(id, a.invoice);
              };
            };
            // update licenses with the verified invoice
            licenses.put(caller, true);
          };
        };
        
        return verifyResult;
      };
    };
  };

// #region Utils
  public shared ({caller}) func reset_license() : async () {
    licenses.put(caller, false);
    ();
  };
// #endregion

// #region Upgrade Hooks
  system func preupgrade() {
      invoicesStable := Iter.toArray(invoices.entries());
      licensesStable := Iter.toArray(licenses.entries());
  };

  system func postupgrade() {
      invoices := HashMap.fromIter(Iter.fromArray(invoicesStable), 16, Nat.equal, Hash.hash);
      invoicesStable := [];
      licenses := HashMap.fromIter(Iter.fromArray(licensesStable), 16, Principal.equal, Principal.hash);
      licensesStable := [];
  };
// #endregion
};
