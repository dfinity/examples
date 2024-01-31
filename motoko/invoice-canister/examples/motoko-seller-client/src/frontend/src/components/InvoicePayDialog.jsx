import React from "react";
import {
  Button,
  ButtonGroup,
  Content,
  Dialog,
  DialogTrigger,
  Divider,
  Heading,
  Text,
} from "@adobe/react-spectrum";
import Invoice from "./Invoice";
import Payment from "./Payment";
import { Switch } from '@adobe/react-spectrum'
import styled from "styled-components";

const TokenSelectionDiv = styled.div`
  display: flex;
  flex-direction: column;
  .heading {
    font-size: 1.5em;
    font-weight: 600;
  }
  margin-bottom: 12px;
`
const InvoicePayDialog = ({ invoice, generateInvoice, onPaid, clearInvoice }) => {

  const [isSelected, setSelection] = React.useState(false);
  const [paymentTokenType, setPaymentTokenType] = React.useState('ICP');

  const onTokenPaymentTypeChanged = () => {
    setSelection(!isSelected);
    setPaymentTokenType(isSelected ? 'ICP' : 'ICRC1');
  }

  const onGenerateInvoice = () => {
    clearInvoice();
    generateInvoice(paymentTokenType);
  }

  return (
    <>
    <TokenSelectionDiv>
      <span className="heading">Payment Token Type:</span>
        <Switch
          isSelected={isSelected}
          onChange={onTokenPaymentTypeChanged}>
          {paymentTokenType}
        </Switch>
    </TokenSelectionDiv>

    <DialogTrigger>
      <div>
        <Button variant="cta" onPress={onGenerateInvoice}>
            Purchase a Premium License
        </Button>
      </div>
      {(close) => (
        <Dialog>
          <Heading>Invoice for Premium License</Heading>
          <Divider />
          <Content>
            {invoice ? (
              <div>
                <Invoice invoice={invoice} />
                <Payment
                  /* The following use of object keys is one way to get the variant tag as a literal. */
                  token={Object.keys(invoice?.token)[0]}
                  amount={invoice.amountDue}
                  id={invoice.id}
                  paymentAddress={invoice.paymentAddress}
                  onPaid={() => {
                    close();
                    onPaid();
                  }}
                />
              </div>
            ) : (
              <Text>Pulling up your invoice...</Text>
            )}
          </Content>
          <ButtonGroup>
            <Button variant="secondary" onPress={close}>
              Cancel
            </Button>
          </ButtonGroup>
        </Dialog>
      )}
    </DialogTrigger>
    </>
  );
};

export default InvoicePayDialog;
