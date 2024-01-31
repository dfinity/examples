import Nat "mo:base/Nat";
import Result "mo:base/Result";
import Time "mo:base/Time";

import SupportedToken "./supported-token/SupportedToken";
import Types "./Types";

/****Module for migrating invoice record type be compatible with SupportedToken.**  
  For migrating invoices using original invoice canister implementation record type to  \
  to the `Invoice_` record type compatible with SupportedToken used in this canister\ 
  implementation. Note that a ULID is not generated, but the original invoice id is 
  converted to text as the Nat it is so its known value can still be expected. */
module {

  // !!IMPORTANT!! Set the SupportedToken variant tag invoices should be set with here
  // before performing migration!
  public let ICP_VARIANT_TAG_TO_CONVERT_INTO = #ICP;

  /****Converts an "ICP" `token` specific invoice to the `Invoice_` record type.***/
  public func convertOne(
    invoiceCanisterId : Principal,
    {
      id : Nat;
      creator : Principal;
      details : ?{
        description : Text;
        meta : Blob;
      };
      permissions : ?{ canGet : [Principal]; canVerify : [Principal] };
      amount : Nat;
      amountPaid : Nat;
      verifiedAtTime : ?Time.Time;
      paid : Bool;
      destination : Blob;
    },
  ) : Types.Invoice_ {
    let newIdAsSimplyNatLiteral = Nat.toText(id);
    let paymentAddress : Text = SupportedToken.getEncodedInvoiceSubaccountAddress({
      token = ICP_VARIANT_TAG_TO_CONVERT_INTO;
      id = newIdAsSimplyNatLiteral;
      creator;
      canisterId = invoiceCanisterId;
    });
    return {
      token = ICP_VARIANT_TAG_TO_CONVERT_INTO;
      id = newIdAsSimplyNatLiteral;
      creator;
      details;
      permissions;
      paymentAddress;
      amountDue = amount;
      amountPaid;
      verifiedPaidAtTime = verifiedAtTime;
    };
  };
};
