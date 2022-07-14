import React from "react";
import {
  ActionButton,
  Button,
  ButtonGroup,
  Content,
  Dialog,
  DialogTrigger,
  Divider,
  Header,
  Heading,
  Text,
} from "@adobe/react-spectrum";
import Invoice from "./Invoice";
import Payment from "./Payment";

const InvoicePayDialog = ({ invoice, generateInvoice, onPaid }) => {
  return (
    <DialogTrigger>
      <Button variant="cta" onPress={generateInvoice}>
        Purchase a Premium License
      </Button>
      {(close) => (
        <Dialog>
          <Heading>Invoice for Premium License</Heading>
          <Divider />
          <Content>
            {invoice ? (
              <div>
                <Invoice invoice={invoice} />
                <Payment
                  amount={invoice.amount}
                  id={invoice.id}
                  destination={invoice.destination}
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
  );
};

export default InvoicePayDialog;
