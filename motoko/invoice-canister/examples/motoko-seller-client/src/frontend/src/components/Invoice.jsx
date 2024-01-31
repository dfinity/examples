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
    margin-top: 2px;
    grid-column: 1;
  }
  dd {
    margin: 0;
    margin-top: 2px;
    grid-column: 2;
  }
  textarea {
    width: 274px;
  }
`;
/*
Invoice from invoice canister in motoko-seller-client as it would appear in the browser's enviroment: 
{
  "id": "6GNGGRXAKGTXG070DV4GW2JKCJ",
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

// Copied from https://github.com/extrawurst/ulid/blob/master/lib/index.umd.js#L84
const ULID_time_decoder = () => {
  const ENCODING = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
  const ENCODING_LEN = ENCODING.length;
  const TIME_MAX = Math.pow(2, 48) - 1;
  const TIME_LEN = 10;
  const RANDOM_LEN = 16;

  const decodeTime = (id) => {
    if (id.length !== TIME_LEN + RANDOM_LEN) {
        throw createError("malformed ulid");
    }
    const time = id.substr(0, TIME_LEN).split("").reverse().reduce((carry, char, index) => {
        const encodingIndex = ENCODING.indexOf(char);
        if (encodingIndex === -1) {
            throw createError("invalid character found: " + char);
        }
        return carry += encodingIndex * Math.pow(ENCODING_LEN, index);
    }, 0);
    if (time > TIME_MAX) {
        throw createError("malformed ulid, timestamp too large");
    }
    return time;
  }

  return {
    decodeTime: (id) => decodeTime(id)
  }
}
// To decode the built-in timestamp of the invoice's ULID id. 
const ulidTimeDecoder = ULID_time_decoder();

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
  
  // Get the creation timestamp from the invoice's ULID.
  const creationTime = new Date(ulidTimeDecoder.decodeTime(invoice.id)).toLocaleString();

  return (
    <Grid>
      <h2>Invoice Statement</h2>
      <dl>
        <dt>
          Id:
        </dt>
        <dd>{invoice.id}</dd>
        <dt>
          <h3>Items:</h3>
        </dt>
        <dd>License for Example Dapp Premium</dd>
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
        <dt>Created:</dt>
        <dd>
          {creationTime}
        </dd>
      </dl>
    </Grid>
  );
}

export default memo(Invoice);

