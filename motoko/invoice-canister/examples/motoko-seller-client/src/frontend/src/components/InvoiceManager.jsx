import React, { memo } from "react";
import { sellerActor } from "../identity";
import InvoicePayDialog from "./InvoicePayDialog";
import { get, set } from "local-storage";

function InvoiceManager(props) {

  const [invoice, setInvoice] = React.useState(null);
  const { status, setStatus } = props;

  if (status === null) return null;

  const clearInvoice = () => {
    // Otherwise the previous invoice will show in the dialog
    // until the new invoice is returned and loaded.
    setInvoice(null);
  };

  const generateInvoice = async (paymentTokenType) => {
    if (!paymentTokenType) {
      throw new Error("Cannot generate invoice without specifying type of token.");
    }
    const savedId = get("license-id");      
    if (savedId) {
      // Invoice been created, get it from the seller actor.
      const getCallResult = await sellerActor.get_invoice(savedId);
      setInvoice(getCallResult[0]);
    } else {
      // Invoice has not yet been created, trigger seller actor to create one to use.
      const createCallResult = await sellerActor.create_invoice(
        [ paymentTokenType === 'ICP' ? { ICP: null} : { ICRC1 : null} ]
      );
      setInvoice(createCallResult.ok.invoice);
      set("invoice-id", createCallResult.ok.invoice.id);
    }
  };

  const onPaid = () => {
    setStatus(true);
  };

  return (
    <section id="invoice-manager">
      {status ? null : (
        <InvoicePayDialog
          generateInvoice={generateInvoice}
          clearInvoice={clearInvoice}
          invoice={invoice}
          onPaid={onPaid}
        />
      )}
    </section>
  );
}

export default memo(InvoiceManager);
