import HashMap "mo:base/HashMap";
import Iter "mo:base/Iter";
import Nat64 "mo:base/Nat64";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Text "mo:base/Text";

import Invoice "./Invoice";
import Types "./modules/Types";

actor Seller {

  // Invoice canister is now a class actor, can't import it as a mo:canister import.
  let invoiceCanister : Invoice.Invoice = actor ("rrkah-fqaaa-aaaaa-aaaaq-cai");

  // ICRC1 token in this case uses same number of decimals, so
  // this is reused when creating an invoice for ICRC1.
  let ONE_ICP_IN_E8S : Nat64 = 100_000_000;

  stable var licensesStable : [(Principal, Bool)] = [];
  var licenses : HashMap.HashMap<Principal, Bool> = HashMap.fromIter(
    Iter.fromArray(licensesStable),
    16,
    Principal.equal,
    Principal.hash,
  );

  // If token type is not specified, the invoice will be created for ICP.
  public shared ({ caller }) func create_invoice(token : ?{ #ICP; #ICRC1 }) : async Types.CreateInvoiceResult {
    let tokenType : { #ICP; #ICRC1 } = switch token {
      case null #ICP;
      case (?userElectedType) {
        switch userElectedType {
          case (#ICP) #ICP;
          case (#ICRC1) #ICRC1;
        };
      };
    };
    // Only used to put the literal into meta blob below.
    let tokenLiteral : Text = switch tokenType {
      case (#ICP) "ICP";
      case (#ICRC1) "ICRC1";
    };
    // Create invoice arg currently requires amount be passed in as token-ledger canister expected
    // values; internally the base unit amount is always used as a Nat type, (but) and a caller will
    // always be returned the corresponding canister expected values (ie if ICP { e8s : Nat64 }).
    let tokenAmount = switch tokenType {
      case (#ICP) #ICP({ e8s = (ONE_ICP_IN_E8S * 10) : Nat64 });
      case (#ICRC1) #ICRC1(Nat64.toNat(ONE_ICP_IN_E8S * 10));
    };
    // Prepare the detail's meta.
    let metaLine1 = "{\n\"seller\": {\n\"principal\": \"" # Principal.toText(caller) # "\",\n\"store\": \"Invoice Canister Example Dapp\",\n},\n";
    let metaLine2 = "\"itemized_bill\": {\n\"rendered\": [\"Standard License\"],\"tokenCurrency\": \"" # tokenLiteral # "\"}\n}";
    let createArgs : Types.CreateInvoiceArgs = {
      tokenAmount;
      permissions = null;
      details = ?{
        description = "Example license certifying status";
        meta = Text.encodeUtf8(metaLine1 # metaLine2);
      };
    };
    await invoiceCanister.create_invoice(createArgs);
  };

  public shared ({ caller }) func get_invoice(id : Types.InvoiceId) : async ?Types.Invoice {
    switch (await invoiceCanister.get_invoice({ id })) {
      case (#err getInvoiceErr) return null;
      case (#ok invoice) {
        ?(invoice.invoice);
      };
    };
  };

  public shared ({ caller }) func verify_invoice(id : Types.InvoiceId) : async Types.VerifyInvoiceResult {
    let verifyCallResult = await invoiceCanister.verify_invoice({ id });
    switch verifyCallResult {
      case (#err _) {};
      case (#ok _) {
        // The result is going to be returned either way, but if
        // it's verified paid the licences needs to be updated.
        licenses.put(caller, true);
      };
    };
    verifyCallResult;
  };

  public shared query ({ caller }) func check_license_status() : async Bool {
    switch (licenses.get(caller)) {
      case (null) return false;
      case (?license) {
        license;
      };
    };
  };

  public shared ({ caller }) func reset_license() : async () {
    licenses.put(caller, false);
  };

  system func preupgrade() {
    licensesStable := Iter.toArray(licenses.entries());
  };
  system func postupgrade() { licensesStable := [] };
};
