import { Button } from "@adobe/react-spectrum";
import React, { memo } from "react";
import { invoiceActor } from "../identity";
import { sellerActor } from "../identity";


function Payment({ id, token, paymentAddress, amount, onPaid }) {
  const [processing, setProcessing] = React.useState(false);

  const getTokenSpecificAmount = () => {
    if (token === 'ICP') {
      return { ICP: { e8s: amount } };
    } else if (token === 'ICRC1') {
      return { ICRC1: amount };
    } else {
      throw new Error("Invalid token passed into payment to get corresponding amount arg for payment");
    }
  };

  const makePayment = async () => {
    setProcessing(true);
    await invoiceActor.deposit_free_money({
      destination : { HumanReadable: paymentAddress },
      tokenAmount: getTokenSpecificAmount()
    });
    await sellerActor.verify_invoice(id);
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
