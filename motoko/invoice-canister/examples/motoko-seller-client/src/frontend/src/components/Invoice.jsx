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
    height: 32px;
  }
`;

// invoice:
// amount: 1000000000n
// amountPaid: 0n
// creator: Principal {_arr: Uint8Array(10), _isPrincipal: true}
// destination: {text: 'a79f2628a7934b78e0c8b227b92e5bb8bad7e09666152cd33a1bf898b0698598'}
// details: [{…}]
// expiration: 1645475804024331000n
// id: 52n
// paid: false
// permissions: []
// refundAccount: []
// refunded: false
// refundedAtTime: []
// token: {decimals: 8n, meta: Array(1), symbol: 'ICP'}
// verifiedAtTime: []

function Invoice({ invoice }) {
  const decimals = Number(invoice.token.decimals);
  const locale = navigator.language;
  const amountInToken = Number(invoice.amount) / Math.pow(10, decimals);

  console.log(amountInToken);

  const formatted = new Intl.NumberFormat(locale, {
    maximumSignificantDigits: decimals,
  }).format(amountInToken);
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
          {formatted} {invoice.token.symbol}
        </dd>
        <dt>Destination address:</dt>
        <dd>
          <textarea
            name="destination"
            id="destination"
            cols="30"
            rows="1"
            readOnly
            defaultValue={invoice.destination.text}
          />
        </dd>
      </dl>
    </Grid>
  );
}

export default memo(Invoice);
