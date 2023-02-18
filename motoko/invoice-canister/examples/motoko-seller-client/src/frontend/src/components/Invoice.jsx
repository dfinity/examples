import React, { memo } from "react";
import styled from "styled-components";

const Grid = styled.div`
  display: grid;
  grid-template-columns: 1fr auto;
  h2,
  h3 {
    margin: 0;
    grid-column: span 2;
  }
  dl {
    display: inline-grid;
  }
  dt {
    grid-column: 1;
  }
  dd {
    margin: 0;
    grid-column: 2;
  }
  textarea {
    width: 274px;
  }
`;
/*
Invoice from invoice canister in motoko-seller-client as it'll appear in the browser's enviroment: 
{
  "id": "7",
  "permissions": [],
  "creator": {
    "_arr": { <ommited> }
    "_isPrincipal": true
  },
  "token": {        // or this would also be "ICR1 : null"
    "ICP": null
  },
  "tokenVerbose": {
    "fee": "10000",
    "decimals": "8",
    "meta": [
      {
        "Url": "https//internetcomputer.org",
        "Issuer": "e8s - For Demonstration Purposes"
      }
    ],
    "name": "_Internet Computer Protocol Token Seller Example Edition",
    "symbol": "_MICP"
  },
  "verifiedPaidAtTime": [],
  "paid": false,
  "paymentAddress": "84153d7052741d8a386afbe8141f911b3b5206cdbea00dc58350642836c5ef84",
  "amountPaid": "0",
  "details": [ < there are details when running the example but I was debugging something when I copied this> ],
  "amountDue": "1000000000"
}
*/

function Invoice({ invoice }) {

  // Recall token is the unit type variant 
  // so it'll be formed in JS as:
  // invoice.token = { ICP: null } or 
  // invoice.token = { ICRC1: null }. 
 
  // One way to get the variant's tag literal.
  // (Requires it not to be undefined).
  const token = Object.keys(invoice.token)[0];

  const getUnitsReminder = () => {
    // Just a reminder, while ICP <> e8s is well known amoung (at least ICP developers?),
    // this may not be the case for users new to the Internet Computer or when using a 
    // newly minted ICRC1 token with a non-typical decimals value. 
    switch (token) {
      case 'ICP': 
        return `(or ${invoice.amountDue} e8s)`
      case 'ICRC1':
        return `(or ${invoice.amountDue} total ICRC1 tokens)`
    }
  }

  // Format the amount. 
  const decimals = Number(invoice.tokenVerbose.decimals);
  const amountInToken = Number(invoice.amountDue) / Math.pow(10, decimals);
  const locale = navigator.language;
  const formatted = new Intl.NumberFormat(locale, {maximumSignificantDigits: decimals}).format(amountInToken); 
  
  return (
    <Grid>
      <h2>Invoice #{Number(invoice.id)}</h2>
      <dl>
        <dt>
          <h3>Items:</h3>
        </dt>
        <dd>1 license for Example Dapp Premium</dd>
        <dt>Price:</dt>
        <dd>
          {formatted} {token} { getUnitsReminder() }
        </dd>
        <dt>Payment address:</dt>
        <dd>
          <textarea
            name="destination"
            id="destination"
            cols="30"
            rows={token === 'ICP' ? "3" : "4"}
            resize="none"
            readOnly
            defaultValue={invoice.paymentAddress}
          />
        </dd>
      </dl>
    </Grid>
  );
}

export default memo(Invoice);

