import { Button } from "@adobe/react-spectrum";
import React, { memo } from "react";
import { invoiceActor } from "../identity";
import { sellerActor } from "../identity";

function Payment({ id, destination, amount, onPaid }) {
  const [processing, setProcessing] = React.useState(false);
  const makePayment = async () => {
    setProcessing(true);
    await invoiceActor.deposit_free_money({
      accountIdentifier: destination,
      amount,
    });
    let verifyResult = await sellerActor.verify_invoice(id);
    console.log(verifyResult);
    setProcessing(false);
    onPaid();
  };

  return (
    <Button type="primary" onPress={makePayment} isDisabled={processing}>
      Confirm and Purchase
    </Button>
  );
}

export default memo(Payment);
