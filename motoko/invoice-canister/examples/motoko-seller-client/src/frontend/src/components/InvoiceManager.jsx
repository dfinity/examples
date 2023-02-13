import React, { memo } from "react";
import { Button } from "@adobe/react-spectrum";
import { sellerActor } from "../identity";
import InvoicePayDialog from "./InvoicePayDialog";
import { get, set } from "local-storage";

function InvoiceManager(props) {
  const [invoice, setInvoice] = React.useState(null);
  const { status, setStatus } = props;

  if (status === null) return null;

  const generateInvoice = async () => {
    const savedId = get("license-id");
    if (savedId) {
      const result = await sellerActor.get_invoice(BigInt(Number(savedId)));
      setInvoice(result[0]);
    } else {
      const result = await sellerActor.create_invoice();
      setInvoice(result.ok.invoice);
      set("invoice-id", result.ok.invoice.id.toString());
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
          invoice={invoice}
          onPaid={onPaid}
        />
      )}
    </section>
  );
}

export default memo(InvoiceManager);
