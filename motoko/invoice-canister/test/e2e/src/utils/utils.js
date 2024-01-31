import { Principal } from '@dfinity/principal';
import { SupportedTokens } from './constants.js';

// Copied from https://github.com/dfinity/ic-js/blob/main/packages/nns/src/utils/converter.utils.ts
export const toHexStringFromUInt8Array = bytes => {
  return bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');
};

// Adapted from https://github.com/dfinity/ic-js/blob/main/packages/ledger/src/utils/ledger.utils.ts
export const toHexStringFromICRC1Account = ({ owner, subaccount }) => {
  if (!subaccount || subaccount.length === 0) {
    return owner.toText();
  } else {
    const shrink = bytes => {
      const shrinked = Array.from(bytes);
      while (shrinked[0] === 0) {
        shrinked.shift();
      }
      return Uint8Array.from(shrinked);
    };
    const subaccountbytes = shrink(subaccount[0]);
    return Principal.fromUint8Array(
      Uint8Array.from([
        ...owner.toUint8Array(),
        ...subaccountbytes,
        subaccountbytes.length,
        parseInt('7F', 16),
      ]),
    ).toText();
  }
};

export const principalFromText = text => Principal.fromText(text);

const encoder = new TextEncoder();

/**Generates `create_invoice` args. Only `whichToken` param is required. 
  Note NO **validation** is performed (such as whether values in permissions 
  arrays are indeed principals or principal literals! */
export const getTestCreateInvoiceArgs = ({
  whichToken,
  amountDue,
  getPermissions,
  verifyPermissions,
  description,
  meta,
}) => {
  if (typeof whichToken === 'undefined') {
    throw new Error('Tried to create create_invoice args without specifying which token.');
  }
  const billableAmountDue = amountDue ?? 100000000n;
  const setTokenAmount = whichToken => {
    switch (whichToken) {
      case SupportedTokens.asVariantTagLiteral.ICP:
      case SupportedTokens.asVariantTagLiteral.ICP_nns:
        return {
          [`${whichToken}`]: {
            e8s: billableAmountDue,
          }, // ie becomes `{ ICP: { e8s: 100000n }}` | `{ ICP_nns: { e8s: 100000n }}`
        };
      case SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken:
      case SupportedTokens.asVariantTagLiteral.ICRC1_ExampleToken2:
        return {
          [`${whichToken}`]: billableAmountDue,
        }; // ie becomes `{ ICRC1_ExampleToken: 100000n }` | `{ ICRC1_ExampleToken2: 100000n }`
    }
  };

  const setPermissions = (get, verify) => {
    const permissions = [];
    if (get || verify) {
      permissions[0] = {
        canGet: get ? [...get] : [],
        canVerify: verify ? [...verify] : [],
      };
    }
    return permissions;
  };

  const setDetails = (desc, m) => {
    const deets = [];
    if (desc || m) {
      deets[0] = {
        description: desc ?? '',
        meta: Array.from(encoder.encode(JSON.stringify(m))),
      };
    }
    return deets;
  };

  return {
    permissions: setPermissions(getPermissions, verifyPermissions),
    tokenAmount: setTokenAmount(whichToken),
    details: setDetails(description, meta),
  };
};
